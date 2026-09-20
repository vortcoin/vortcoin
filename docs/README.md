# VORTCOIN NETWORK Layer-1 Core Engine

[![Rust](https://shields.io)](https://rust-lang.org)
[![Version](https://shields.io)](#)
[![License](https://shields.io)](LICENSE)

VORTCOIN (VORT) is a long-term adaptive Layer-1 blockchain engineered to fuse the immutable scarcity physics of hard-sound money with the ultra-high throughput efficiency of parallelized monolithic architectures. Designed to seamlessly bridge the concrete stability of Real World Assets (RWA) and the fluid velocity of community-driven tokens, VORTCOIN integrates specialized post-quantum cryptographic primitives to secure a resilient, future-proof global financial infrastructure. The network operates on a unique native consensus mechanism known as **Proof of Adaptive Velocity (PoAV)**.

---

## 🛠 Core Architectural Features (Tri-Core Layer 1)

1. **PoAV Consensus (Proof of Adaptive Velocity)**: An advanced cryptographic consensus that dynamically scales handshake difficulty in real-time based on the network's immediate TPS payload.
2. **Quantum-Safe Cryptography**: Heavy-duty protection for standard BIP-39 24-word passphrases, utilizing an enhanced `pbkdf2` key-stretching mechanism backed by 3,690 Tesla matrix iterations.
3. **Hyper-Deflationary Mechanism (Tesla Alignment 3-6-9)**: Maximum token supply is hard-capped at exactly **36,900,000 VORT** with 9-decimal precision. Every on-chain transaction automatically burns **36.9% of the gas fees** directly into an unrecoverable black hole address.
4. **Native Vesting & Bridge Infrastructure**: Out-of-the-box automated Vesting-Cliff protocols for private allocations alongside embedded cross-chain bridges (EVM/Solana) handled directly at the core protocol level.

---

## 🌐 Network Port Specifications

When deploying a VORTCOIN L1 validator or node on your server (GCP, AWS, or bare-metal VPS), ensure your Linux firewall rules allow inbound traffic on the following two primary ports:
* **Port `3690` (P2P Socket)**: Used for secure node-to-node communications, block synchronization, and processing PoAV handshake mechanics.
* **Port `8545` (RPC HTTP API)**: Used by Web3 frontends, local block explorers, and browser extension wallets to asynchronously query on-chain data.

---

## 💻 Command Line Interface (CLI) Quick Start

Before interacting with the ledger daemon, activate your localized environment shortcut script:
```bash
source vortcoin-env.sh
```

### A. Account & Wallet Management
* **Generate a Quantum-Safe Wallet**:
  ```bash
  vortcoin wallet-create
  ```
* **Query On-Chain Account Balance**:
  ```bash
  vortcoin balance --address <YOUR_VORTCOIN_ADDRESS>
  ```

### B. Transactions & Deflationary Transfers
* **Send VORT Coins (Automated 36.9% Gas Burn)**:
  ```bash
  vortcoin transfer --from <SENDER_ADDRESS> --to <RECEIVER_ADDRESS> --amount <AMOUNT>
  ```

### C. Node Operations & Macro Ecosystem
* **Launch a PoAV Miner/Validator Node**:
  ```bash
  vortcoin node-start --miner-address <PAYOUT_WALLET_ADDRESS>
  ```

* **Lock Native Coins for External Cross-Chain Bridging (Uniswap/Solana)**:
  ```bash
  vortcoin bridge-to-wrapped --from-address <WALLET_ADDRESS> --target-network ethereum --target-wallet-escrow <ETH_ROUTER> --amount 1845000
  ```

---

## 🖥 Hardware Requirements (Miners & Validators)
* **CPU**: 1 Core (Standard Virtual CPU or equivalent)
* **RAM**: 1 GB or higher
* **Storage**: 20 GB SSD / NVMe (Required for lightning-fast localized Sled DB ledger operations)
* **OS**: Ubuntu 22.04 LTS / Ubuntu 24.04 LTS

---

## 📄 License
The VORTCOIN Network L1 Core project is open-source software licensed under the official **MIT License**.

