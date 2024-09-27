# Exploring Solana: A Developer's Journey

Welcome to my Solana development portfolio! This repository showcases my journey in exploring the Solana through a series of Anchor programs. Each project demonstrates different aspects of Solana development, from basic concepts to more advanced implementations.

## Table of Contents

1. [Introduction](#introduction)
2. [Projects](#projects)
   - [Vault](#vault)
   - [Escrow](#escrow)
   - [Automated Market Maker (AMM)](#automated-market-maker-amm)
   - [NFT Staking](#nft-staking)
   - [NFT Staking Core](#nft-staking-core)
   - [Clean and Claim](#clean-and-claim)
3. [Getting Started](#getting-started)
4. [Contact](#contact)

## Introduction

Hi, I'm Het, a software developer exploring Solana. This repository contains a collection of Anchor programs I've built while learning. These projects range from basic token vaults to more complex systems like automated market makers and NFT staking. By browsing this repo, you'll see my progress in understanding Solana's architecture and implementing various DeFi concepts.

## Projects

### Vault

A secure token storage solution implemented on Solana.

Key features:
- Secure token deposits and withdrawals
- Access control and ownership management
- Interest accrual mechanism (if applicable)

[View Vault Project](./vault)

### Escrow

A trustless escrow service for secure peer-to-peer transactions.

Key features:
- Atomic swaps between two parties
- Time-locked transactions
- Cancellation and refund mechanisms

[View Escrow Project](./escrow)

### Automated Market Maker (AMM)

A decentralized exchange protocol for swapping tokens.

Key features:
- Liquidity pool management
- Constant product formula (x * y = k)
- Slippage protection and fee mechanism

[View AMM Project](./amm)

### NFT Staking

A program that allows users to stake their NFTs and earn rewards.

Key features:
- NFT deposit and withdrawal
- Reward distribution based on staking duration
- Support for multiple NFT collections

[View NFT Staking Project](./nft-staking)

### NFT Staking Core

The NFT staking program implemented for new MPL-Core standard.

Key features:
- Easy to implement
- Escrowless
- No need to manage rewards

[View NFT Staking Core Project](./nft-staking-core)

### Clean and Claim

My capstone project, addressing community fundraising in a novel way.

#### Problem
How to raise funds from the community without ripping them off?

#### Solution
Take the rent SOL from users' empty token accounts in exchange for your own token. The frontend should be a blink. Users get to clear their empty accounts in exchange for your tokens, creating a mutually beneficial deal.

#### Key Features
- Users can clean up their empty token accounts
- In return, users receive project tokens
- Simple and quick user experience
- Fundraising mechanism for projects

#### Limitations
- Limited by the number of empty token accounts (only 0.002 SOL per account)
- Need to clear 10,000 accounts to raise approximately $3,000
- No fair distribution mechanism (can be gamed)
- Primary goal is fundraising, not fair token distribution

#### Program Details
The Clean and Claim program is implemented using Anchor framework. It allows users to close their empty token accounts and claim project tokens in return. The program includes functionality for initialization, replenishing the reward pool, and the main clean-and-claim operation.

**Devnet Program ID:** `C5tBGFF2h8s2432GyS5xG5Qf5QWo7KJmfmUT9R8a51bu`

[View Clean and Claim Project](./clean-and-claim)

## Getting Started

To explore these projects and start your Solana development journey:

1. Clone this repository
2. Install Solana and Anchor on your local machine
3. Navigate to each project folder
4. Run `anchor build` to compile the programs
5. Run `anchor deploy` to deploy the programs on devnet
6. Run `anchor test` to execute the test suites

### Resources for Learning Solana Development

1. [Solana Cookbook](https://solanacookbook.com/) - A developer resource for building on Solana
2. [Solana Programming Library (SPL) Documentation](https://spl.solana.com/) - Official docs for Solana's token standards and programs
3. [Anchor Documentation](https://www.anchor-lang.com/) - Learn about the Anchor framework for Solana development
4. [Solana Developers Portal](https://solana.com/developers) - Official resources and tools for Solana developers
5. [Solana Playground](https://beta.solpg.io/) - Interactive web-based IDE for Solana development
6. [Solana Bootcamp](https://www.youtube.com/playlist?list=PLilwLeBwGuK7Z2dXft_pmLZ675fuPgkA0) - Video series covering Solana basics to advanced topics
7. [Buildspace Solana Core](https://buildspace.so/solana-core) - Interactive course for building on Solana
8. [Solana Stack Exchange](https://solana.stackexchange.com/) - Q&A platform for Solana developers

## Contact

I'm always eager to discuss Solana development, blockchain technology, or potential collaborations. Feel free to reach out to me at:

- Email: [daglihet@gmail.com](mailto:daglihet@gmail.com)
- LinkedIn: [Het Dagli](https://www.linkedin.com/in/hetdagli)
- Twitter: [@daglihet](https://twitter.com/daglihet)

Thank you for exploring my Solana development journey!