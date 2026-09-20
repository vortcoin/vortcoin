# Contributing to VORTCOIN Core Engine

Thank you for your interest in contributing to VORTCOIN Layer-1! As an open-source, community-driven blockchain network, your pull requests (PRs) and issue reporting keep the core architecture robust, fast, and secure.

## ⚖️ Code of Conduct
By participating in this repository, you agree to maintain a professional, collaborative, and constructive environment. Focus on technical facts, safety-first engineering, and respect toward fellow blockchain developers.

## 🚀 How to Contribute

### 1. Reporting Bugs & Issues
* Before opening a new issue, search the current repository database to see if a similar bug has already been tracked.
* Provide a clear description of the bug, including your environment configurations (OS, Rust version, hardware specs).
* Include terminal logs and step-by-step instructions to reliably reproduce the error.

### 2. Submitting Pull Requests (PRs)
To ensure your changes can be safely merged into the `main` branch, please follow this development lifecycle:

1. **Fork & Branch**: Fork this repository into your personal space and create a descriptively named branch:
   ```bash
   git checkout -b feature/optimize-poav-handshake
   ```
2. **Rust Coding Standards**: 
   * Ensure your code is thoroughly formatted using `cargo fmt`.
   * Check for performance overheads, logic edge cases, and compliance with the 3-6-9 Tesla Alignment constraints.
3. **Run Suite Tests**: All localized unit and integration tests must pass cleanly before submission:
   * Run `cargo test` to execute native suites.
4. **Sign Your Commits**: Due to the decentralized and critical financial nature of a Layer-1 project, we highly encourage or require all commits to be digitally signed with a **GPG/SSH Key** verified on your GitHub profile.
5. **Open the PR**: Submit your PR targeting our `main` branch. Provide a comprehensive summary outlining *what* was modified and *why* it improves the core ledger engine.

## 🔐 Vulnerability & Bug Bounty Reporting
**Do NOT open a public GitHub Issue for critical security exploits or economic vectors** (e.g., smart contract bugs, network halos, double-spend vectors). Please escalate these vulnerabilities privately via the security reporting mechanism outlined in our `SECURITY.md` file to safeguard on-chain assets.
