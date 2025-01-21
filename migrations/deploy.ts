import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PartyRound } from "../target/types/party_round";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount } from "@solana/spl-token";

async function main() {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PartyRound as Program<PartyRound>;

  // Generate PDAs
  const [mintAuthority] = await PublicKey.findProgramAddress(
    [Buffer.from("mint_authority")],
    program.programId
  );

  const [treasuryAuthority] = await PublicKey.findProgramAddress(
    [Buffer.from("treasury")],
    program.programId
  );

  const [daoState] = await PublicKey.findProgramAddress(
    [Buffer.from("dao_state")],
    program.programId
  );

  // Create token mint
  console.log("Creating token mint...");
  const daoMint = await createMint(
    provider.connection,
    provider.wallet as any,
    mintAuthority,
    null,
    9 // decimals
  );

  // Create treasury token account
  console.log("Creating treasury token account...");
  const daoTreasury = await createAccount(
    provider.connection,
    provider.wallet as any,
    daoMint,
    treasuryAuthority
  );

  // Initialize DAO
  console.log("Initializing DAO...");
  const now = Math.floor(Date.now() / 1000);
  const fundraiseEndTs = now + 7 * 24 * 60 * 60; // 1 week from now

  await program.methods
    .initializeDao({
      tokenName: "Party Round",
      tokenSymbol: "PARTY",
      totalSupply: new anchor.BN(1_000_000 * LAMPORTS_PER_SOL),
      fundraiseEndTs: new anchor.BN(fundraiseEndTs),
      tokenPriceLamports: new anchor.BN(LAMPORTS_PER_SOL / 100), // 0.01 SOL per token
      allowlistedAddresses: [], // Empty for public sale
    })
    .accounts({
      daoState,
      daoMint,
      mintAuthority,
      daoTreasury,
      treasuryAuthority,
      payer: provider.wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .rpc();

  console.log("DAO initialized!");
  console.log("Program ID:", program.programId.toString());
  console.log("DAO State:", daoState.toString());
  console.log("DAO Mint:", daoMint.toString());
  console.log("DAO Treasury:", daoTreasury.toString());
}

main().then(
  () => process.exit(0),
  (err) => {
    console.error(err);
    process.exit(1);
  }
); 