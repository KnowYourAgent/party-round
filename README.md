
# Party Round 🎉

An **educational**, **open-source** implementation of a party round fundraising mechanism on the **Solana** blockchain. This project demonstrates how to create a fundraising round where:

- Participants contribute SOL during a fixed time window
- 90% of funds are allocated for DeFi investments
- 10% of funds pair with tokens to seed AMM liquidity
- Token holders can redeem or trade their positions after the round closes

> ⚠️ **For Educational Purposes Only**  
> This code is unaudited and **not** intended for production use. Token sales may be subject to legal and regulatory requirements in your jurisdiction. Consult qualified professionals before any live fundraising.

---

## Table of Contents
1. [Features](#features)  
2. [Quick Start](#quick-start)  
3. [Project Structure](#project-structure)  
4. [Core Instructions](#core-instructions)  
5. [Usage & Example Flow](#usage--example-flow)  
6. [Development](#development)  
7. [Security & Legal](#security--legal)  
8. [Contributing](#contributing)  
9. [References](#references)  
10. [License](#license)

---

## Features
- **Fixed-Price Fundraising**: Set a token price and contribution window.  
- **Optional Allowlist**: Restrict participation to specific wallet addresses if needed.  
- **Treasury Management**: Automatically allocate 90% of funds to DeFi strategies, 10% to AMM liquidity.  
- **AMM Integration**: Seamlessly create a liquidity pool for continuous trading of the DAO token.  
- **Token Redemption**: Allow participants to redeem underlying assets once the round closes.

---

## Quick Start

### Prerequisites
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)  
- [Anchor CLI](https://book.anchor-lang.com/chapter_1/installation.html)  
- Node.js 16+ (and Yarn or npm)

### Installation
```bash
# Clone the repository
git clone https://github.com/knowyourdao/party-round.git
cd party-round

# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
anchor test

# Deploy to Devnet
# Configure Solana CLI for devnet
solana config set --url devnet

# (Optional) Airdrop devnet SOL for testing
solana airdrop 2

# Check your program ID
solana-keygen pubkey target/deploy/party_round-keypair.json

# Update the program ID in Anchor.toml and lib.rs as necessary

# Rebuild and deploy
anchor build
anchor deploy
```

## Project Structure

```
party-round/
├── programs/
│   └── party-round/
│       ├── src/
│       │   ├── instructions/
│       │   │   ├── close_fundraise.rs
│       │   │   ├── contribute_funds.rs
│       │   │   ├── initialize_dao.rs
│       │   │   ├── mod.rs
│       │   │   └── redeem_tokens.rs
│       │   ├── errors.rs
│       │   ├── lib.rs
│       │   └── state.rs
│       └── Cargo.toml
├── tests/
│   └── party-round.ts
├── migrations/
│   └── deploy.ts
├── Anchor.toml
├── Cargo.toml
├── package.json
├── tsconfig.json
└── README.md
```

- `programs/party-round`: Main on-chain Anchor program (smart contract).
- `tests`: Contains TypeScript test files.
- `migrations`: Deployment scripts.
- `Anchor.toml`: Anchor configuration.


## Core Instructions
1. **Initialize Party Round**: Set token parameters (supply, price, decimals), define start/end timestamps, configure optional allowlist.
2. **Contribute Funds**: Users send SOL to the program in exchange for DAO tokens. Mint DAO tokens at a fixed price. Track total contributions and participants.
3. **Close Fundraise**: End the contribution window. Allocate 90% of funds to DAO treasury/DeFi strategies. Use 10% to provide initial liquidity on an AMM (e.g., Raydium/Orca).
4. **Redeem Tokens**: Token holders claim proportional treasury assets when redeeming. Burn redeemed tokens. Allow continued trading on the AMM.

## Usage & Example Flow
1. **Initialize the DAO**: Deploy the contract, specifying token details and the fundraising window. Mint tokens to the DAO’s treasury account.
2. **User Contribution**: Contributors call `contribute_funds` with SOL. The contract transfers DAO tokens to the contributor’s wallet.
3. **Closing the Round**: Once the time window has passed, call `close_fundraise`. 90% of SOL remains in the treasury; 10% seeds an AMM pool.
4. **Redeeming or Trading**: Token holders can redeem tokens for a share of treasury assets. Alternatively, tokens can be traded on the AMM.

## Development

To run a local validator and test your program:

```bash
# Start a local Solana validator
solana-test-validator

# Run tests (skipping local validator since it's already running)
anchor test --skip-local-validator
```

- Adjust parameters in `Anchor.toml` or test scripts.
- Check logs and error codes to debug.

## Security & Legal
- **Not Audited**: This code has no security audits. Do not use for production.
- **Regulatory Caution**: DAO fundraising may be subject to securities regulations. Consult legal experts.
- **Best Practices**: This reference includes basic checks but lacks comprehensive safeguards for real-world usage.

## Contributing
1. Fork this repository
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -am 'Add amazing feature'`)
4. Push to your branch (`git push origin feature/amazing`)
5. Open a Pull Request

## References
- knowyourdao.com
- Solana Docs
- Anchor Book
- Raydium Docs
- Orca Docs
- SPL Token Docs

## License
MIT License. See LICENSE for details.

**Disclaimer**: This repository is for educational and demonstration purposes only. It does not constitute financial or legal advice. Use at your own risk.
```
