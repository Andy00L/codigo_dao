# Solana Reputation DAO

Advanced reputation scoreboard with multi-realm governance and AI-enhanced validation, built with Anchor on Solana.

## Prerequisites

- Node.js 18+
- pnpm or yarn
- Rust + Solana toolchain
- Anchor CLI 0.28+ (0.29 recommended)

## Setup

1. Install deps:
   pnpm install

2. Update program ID:

   - Generate a new key: anchor keys list (or anchor keys create)
   - Replace the program ID in:
     - Anchor.toml at [programs.localnet].solana_reputation_dao
     - programs/solana-reputation-dao/src/lib.rs declare_id!(...)

3. Build and test:
   anchor build
   anchor test

## Project Structure

- programs/solana-reputation-dao: On-chain program with instructions, state, utils, and errors
- tests: Anchor mocha tests
- app/src/idl: Compiled IDL (for front-end integration)

## Notes

- This implementation uses PDAs for profiles and realms:
  - Profile PDA: ["reputation", user_pubkey]
  - Realm PDA: ["realm", realm_name]
- Interaction events are simple initialized accounts (not PDAs) in tests for convenience.
- Adjust constants in utils/constants.rs to tune the system.

## Scripts

- pnpm test: Runs mocha tests
- anchor test: Runs end-to-end tests on localnet
