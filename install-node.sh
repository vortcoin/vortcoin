#!/bin/bash
# =========================================================================
# 🌀 VORTCOIN NETWORK L1 - AUTOMATED RETAIL MINER INSTALLER
# Supported Architectures: Linux x86_64, macOS AMD64/ARM64
# =========================================================================

set -e

echo "================================================================="
echo "🌀 INITIALIZING VORTCOIN L1 DECENTRALIZED MINER DEPLOYMENT"
echo "================================================================="

# 1. Detect Host Operating System and Architecture Layout
OS_TYPE="$(uname -s)"
ARCH_TYPE="$(uname -m)"
INSTALL_DIR="/usr/local/bin"
TARGET_DIR="$HOME/.vortcoin"

echo "[SYSTEM] Operating System Detected: $OS_TYPE ($ARCH_TYPE)"

# 2. Establish Local Application Directory Boundaries
mkdir -p "$TARGET_DIR"
cd "$TARGET_DIR"

# 3. Create the Sacred Fair Launch Genesis Manifest locally
cat << 'EOF' > genesis.json
{
  "network_name": "vortcoin_pure_decentralization_matrix",
  "genesis_time": 1787336900,
  "era": 1,
  "allocations": {},
  "protocol_constants": {
    "max_supply_nano": 36900000000000000,
    "burn_tax_rate": 0.369
  }
}
EOF
echo "[CONFIG] Localized genesis.json manifest synchronized successfully."

# 4. Resolve Cloud Binary Compilation Targets
BINARY_URL="https://github.com/vortcoin/vortcoin.git"

if [ "$OS_TYPE" = "Darwin" ]; then
    BINARY_URL="https://github.com/vortcoin/vortcoin.git"
fi

echo "[NETWORK] Fetching compiled high-performance L1 core biner payload..."
# Download binary natively utilizing curl stream parameters
if ! curl -L -sSf "$BINARY_URL" -o "vortcoin-cli"; then
    echo "ERROR: Failed to download compiled toolchain. Ensure network pathways are open."
    exit 1
fi

chmod +x vortcoin-cli

# 5. Elevate Installation to Global Binary Execution Directories
echo "[DEPLOYMENT] Exposing toolchain to localized path environments..."
if [ -w "$INSTALL_DIR" ]; then
    cp vortcoin-cli "$INSTALL_DIR/"
else
    sudo cp vortcoin-cli "$INSTALL_DIR/"
fi

echo "================================================================="
echo "VORTCOIN CORE NODE INSTALLATION COMPLETED SUCCESSFULLY!"
echo "================================================================="
echo " Toolchain Location : $INSTALL_DIR/vortcoin-cli"
echo " Configuration Path : $TARGET_DIR/genesis.json"
echo "-----------------------------------------------------------------"
echo " To generate a secure 24-word Quantum-Safe wallet, execute:"
echo "   -> vortcoin-cli wallet-create"
echo ""
echo " To launch your intensive PoAV mining loop processing engine, execute:"
echo "   -> vortcoin-cli node-start --miner-address YOUR_WALLET_ADDRESS"
echo "================================================================="
