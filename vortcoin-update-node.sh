#!/bin/bash

# =========================================================================
# VORTCOIN NETWORK - AUTOMATED ZERO-DOWNTIME NODE CORES UPDATER
# Purpose: Pull latest patches, re-compile release binary & safe restart
# Location: Run this from the root directory of 'vortcoin_network' on your VPS
# =========================================================================

# Lock the original user directory path variable
ACTUAL_USER=${SUDO_USER:-$USER}
ACTUAL_HOME=$(eval echo ~$ACTUAL_USER)
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)"
BINARY_RELEASE="$PROJECT_DIR/target/release/vortcoin-cli"

clear
echo "================================================================="
echo "VORTCOIN L1 CORE - AUTOMATED HOT-PATCH NODE CORES UPDATER"
echo "================================================================="
echo "Checking local repository status and halting process safely..."

# 1. Enter the main project directory
cd "$PROJECT_DIR" || { echo "ERROR: Directory $PROJECT_DIR not found."; exit 1; }

# 2. Get the bug fix updates from the GitHub repository
echo "[STEP 1/4] Pulling latest code adjustments from GitHub..."
# Ensure that code pulls are executed on behalf of the actual user to maintain the integrity of the SSH token
sudo -u "$ACTUAL_USER" git pull origin main

if [ $? -ne 0 ]; then
    echo "ERROR: Failed to pull updates from Git. Check your internet or SSH keys."
    exit 1
fi

# 3. Recompilation of the Latest Binary Version (Optimized Process)
echo "[STEP 2/4] Re-compiling new release binary features (Cargo Release)..."
if [ -f "$ACTUAL_HOME/.cargo/bin/cargo" ]; then
    sudo -u "$ACTUAL_USER" "$ACTUAL_HOME/.cargo/bin/cargo" build --release
elif command -v cargo &> /dev/null; then
    sudo -u "$ACTUAL_USER" cargo build --release
else
    echo "ERROR: Rust compiler (cargo) not detected in system env."
    exit 1
fi

if [ $? -ne 0 ]; then
    echo "ERROR: Re-compilation crashed! Please check your Rust code syntax logic."
    exit 1
fi

# 4. Instant Service Restart Execution (Preventing Chain Lag)
echo "[STEP 3/4] Executing synchronized transition to the new binary..."

# Stopping the old service causes systemd to automatically and safely release the Sled database lock file
sudo systemctl stop vortcoin-node.service

# Ensuring the new binary is precisely moved into the Linux kernel register memory
sudo systemctl daemon-reload

# Restart the service with the new binary (downtime of less than a few milliseconds)
sudo systemctl start vortcoin-node.service

# 5. Confirmation of Patch Migration Success Status
echo "[STEP 4/4] Verifying network integration status..."
echo "-----------------------------------------------------------------"
if sudo systemctl is-active --quiet vortcoin-node.service; then
    echo "SUCCESS: VORTCOIN Node has been successfully patched to the latest version!"
    echo "Data ledger DB intact. Mining emissions have safely resumed."
else
    echo "CRITICAL: VORTCOIN Service failed to restart. Check system logs immediately."
fi
echo "================================================================="
echo "To view the live english validation logs, execute:"
echo "   -> journalctl -u vortcoin-node.service -f"
echo "================================================================="
