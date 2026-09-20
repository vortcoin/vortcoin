#!/bin/bash

# =========================================================================
# VORTCOIN NETWORK - CLI ENVIRONMENT SHORTCUT HELPER
# Function: Creates a 'vortcoin' shortcut so you don't have to type 'cargo run --'
# =========================================================================

# Determining the binary location (prioritizing the release version if available)
if [ -f "./target/release/vortcoin-cli" ]; then
    VORTCOIN_CMD="./target/release/vortcoin-cli"
else
    VORTCOIN_CMD="cargo run --"
fi

# Command wrapper function
vortcoin() {
    $VORTCOIN_CMD "$@"
}

# Exporting a function so that it is accessible to the current terminal session
export -f vortcoin

echo "================================================================="
echo "VORTCOIN L1 ENVIRONMENT SHORTCUT ACTIVATED!"
echo "================================================================="
echo "You can now type the 'vortcoin' command directly into the terminal."
echo "Example:"
echo "   -> vortcoin wallet-create"
echo "   -> vortcoin balance --address <WALLET_ADDRESS>"
echo "   -> vortcoin transfer --from <SENDER_ADDRESS> --to <RECEIVER_ADDRESS> --amount <AMOUNT>"
echo "   -> vortcoin node-start --miner-address <WALLET_ADDRESS>"
echo "================================================================="

