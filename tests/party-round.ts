import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PartyRound } from "../target/types/party_round";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  getAccount,
  getMint,
} from "@solana/spl-token";
import { expect } from "chai";

describe("party-round", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PartyRound as Program<PartyRound>;
  
  // Test accounts
  const admin = Keypair.generate();
  const contributor = Keypair.generate();
  let daoState: PublicKey;
  let daoMint: PublicKey;
  let daoTreasury: PublicKey;
  let contributorTokenAccount: PublicKey;
  let mintAuthority: PublicKey;
  let treasuryAuthority: PublicKey;
  let mintAuthorityBump: number;
  let treasuryAuthorityBump: number;

  const TOKEN_DECIMALS = 9;
  const TOTAL_SUPPLY = 1_000_000 * Math.pow(10, TOKEN_DECIMALS);
  const TOKEN_PRICE = LAMPORTS_PER_SOL / 100; // 0.01 SOL per token

  before(async () => {
    // Airdrop SOL to admin and contributor
    await provider.connection.requestAirdrop(admin.publicKey, 10 * LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(contributor.publicKey, 10 * LAMPORTS_PER_SOL);

    // Find PDA addresses
    [mintAuthority, mintAuthorityBump] = await PublicKey.findProgramAddress(
      [Buffer.from("mint_authority")],
      program.programId
    );

    [treasuryAuthority, treasuryAuthorityBump] = await PublicKey.findProgramAddress(
      [Buffer.from("treasury")],
      program.programId
    );

    [daoState] = await PublicKey.findProgramAddress(
      [Buffer.from("dao_state")],
      program.programId
    );
  });

  it("Initialize DAO", async () => {
    // Create token mint
    daoMint = await createMint(
      provider.connection,
      admin,
      mintAuthority,
      null,
      TOKEN_DECIMALS
    );

    // Create treasury token account
    daoTreasury = await createAccount(
      provider.connection,
      admin,
      daoMint,
      treasuryAuthority
    );

    // Create contributor's token account
    contributorTokenAccount = await createAccount(
      provider.connection,
      contributor,
      daoMint,
      contributor.publicKey
    );

    const now = Math.floor(Date.now() / 1000);
    const fundraiseEndTs = now + 7 * 24 * 60 * 60; // 1 week from now

    await program.methods
      .initializeDao({
        tokenName: "Test DAO",
        tokenSymbol: "TDAO",
        totalSupply: new anchor.BN(TOTAL_SUPPLY),
        fundraiseEndTs: new anchor.BN(fundraiseEndTs),
        tokenPriceLamports: new anchor.BN(TOKEN_PRICE),
        allowlistedAddresses: [contributor.publicKey],
      })
      .accounts({
        daoState,
        daoMint,
        mintAuthority,
        daoTreasury,
        treasuryAuthority,
        payer: admin.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([admin])
      .rpc();

    const state = await program.account.daoState.fetch(daoState);
    expect(state.tokenName).to.equal("Test DAO");
    expect(state.tokenSymbol).to.equal("TDAO");
    expect(state.totalSupply.toNumber()).to.equal(TOTAL_SUPPLY);
  });

  it("Contribute funds", async () => {
    const contributionAmount = LAMPORTS_PER_SOL; // 1 SOL
    const expectedTokens = contributionAmount / TOKEN_PRICE;

    await program.methods
      .contributeFunds(new anchor.BN(contributionAmount))
      .accounts({
        daoState,
        daoTreasury,
        treasuryAuthority,
        contributor: contributor.publicKey,
        contributorTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([contributor])
      .rpc();

    const tokenAccount = await getAccount(
      provider.connection,
      contributorTokenAccount
    );
    expect(Number(tokenAccount.amount)).to.equal(expectedTokens);
  });

  it("Close fundraise", async () => {
    // Fast forward time
    await new Promise((resolve) => setTimeout(resolve, 1000));

    await program.methods
      .closeFundraise()
      .accounts({
        daoState,
        daoTreasury,
        treasuryAuthority,
        admin: admin.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    const state = await program.account.daoState.fetch(daoState);
    expect(state.fundraiseEnded).to.be.true;
  });

  it("Redeem tokens", async () => {
    const redeemAmount = 100 * Math.pow(10, TOKEN_DECIMALS);
    const treasuryBalance = await provider.connection.getBalance(daoTreasury);
    const totalSupply = (await getMint(provider.connection, daoMint)).supply;
    
    const expectedRedemption = Math.floor(
      (treasuryBalance * redeemAmount) / Number(totalSupply)
    );

    const oldBalance = await provider.connection.getBalance(contributor.publicKey);

    await program.methods
      .redeemTokens(new anchor.BN(redeemAmount))
      .accounts({
        daoState,
        daoTreasury,
        treasuryAuthority,
        redeemer: contributor.publicKey,
        redeemerTokenAccount: contributorTokenAccount,
        daoMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([contributor])
      .rpc();

    const newBalance = await provider.connection.getBalance(contributor.publicKey);
    expect(newBalance - oldBalance).to.be.approximately(expectedRedemption, 10000);
  });
}); 