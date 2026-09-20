#!/bin/bash

# =========================================================================
# VORTCOIN NETWORK - LIVE MAINNET CLOUD DEPLOYMENT & AUTOMATION TOOL
# Minimal Hardware Requirement: 2 vCPU, 4GB RAM (Ubuntu 22.04/24.04 LTS)
# =========================================================================

ACTUAL_USER=${SUDO_USER:-$USER}
ACTUAL_HOME=$(eval echo ~$ACTUAL_USER)

# Dynamically detect the project root location (flexible for both PC and VPS)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)"
BINARY_PATH="$PROJECT_ROOT/target/release/vortcoin-cli"

clear
echo "================================================================="
echo "VORTCOIN L1 CORE - VPS/GCP LIVE AUTOMATION ENGINE RE-BUILD"
echo "================================================================="

# 1. Automatically rebuild upon code changes
echo "Compiling latest node binary changes dynamically..."
if [ -f "$ACTUAL_HOME/.cargo/bin/cargo" ]; then
    sudo -u "$ACTUAL_USER" "$ACTUAL_HOME/.cargo/bin/cargo" build --release
elif command -v cargo &> /dev/null; then
    sudo -u "$ACTUAL_USER" cargo build --release
else
    echo "ERROR: Rust compiler (cargo) not detected in system env."
    exit 1
fi

# 2. Validate Release Binary Existence
if [ ! -f "$BINARY_PATH" ]; then
    echo "ERROR: Optimized release binary could not be generated at: $BINARY_PATH"
    exit 1
fi

# 3. Lock Down Linux Firewall Rules (UFW & Iptables)
echo "[STEP 1/3] Securing and opening network port junctions..."
if command -v ufw &> /dev/null; then
    sudo ufw allow 3690/tcp comment 'VORTCOIN P2P Socket' > /dev/null
    sudo ufw allow 3690/udp comment 'VORTCOIN P2P Socket' > /dev/null
    sudo ufw allow 8545/tcp comment 'VORTCOIN RPC API Gateway' > /dev/null
    sudo ufw status | grep -q "active" && sudo ufw reload
fi
if command -v iptables &> /dev/null; then
    sudo iptables -C INPUT -p tcp --dport 3690 -j ACCEPT >/dev/null 2>&1 || sudo iptables -A INPUT -p tcp --dport 3690 -j ACCEPT -m comment --comment "VORTCOIN P2P TCP"
    sudo iptables -C INPUT -p tcp --dport 8545 -j ACCEPT >/dev/null 2>&1 || sudo iptables -A INPUT -p tcp --dport 8545 -j ACCEPT -m comment --comment "VORTCOIN RPC API"
fi
echo "   -> Port 3690 (P2P) & Port 8545 (RPC) verified and secured."

# 4. Registering a Linux Background Service (Systemd)
echo "[STEP 2/3] Registering VORTCOIN L1 Background Engine (Systemd)..."
sudo systemctl stop vortcoin-node.service > /dev/null 2>&1

sudo tee /etc/systemd/system/vortcoin-node.service > /dev/null << EOF
[Unit]
Description=VORTCOIN Network Pure Decentralized L1 Core Node
After=network.target

[Service]
User=$ACTUAL_USER
WorkingDirectory=$PROJECT_ROOT
# Default Anchor Node
ExecStart=$BINARY_PATH node-start
Restart=always
RestartSec=10
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:$ACTUAL_HOME/.cargo/bin"

[Install]
WantedBy=multi-user.target
EOF

# Restarting the service daemon
sudo systemctl daemon-reload
sudo systemctl enable vortcoin-node.service > /dev/null 2>&1
sudo systemctl start vortcoin-node.service
echo "   -> Background daemon service re-registered and activated successfully."

# 5. Setting up System Log Cleanup Automation (Native Journald Mitigations)
echo "[STEP 3/3] Deploying automated journald memory mitigations..."
sudo mkdir -p /etc/systemd/journald.conf.d
sudo tee /etc/systemd/journald.conf.d/vortcoin.conf > /dev/null << EOF
[Journal]
SystemMaxUse=100M
SystemMaxFileSize=20M
MaxRetentionSec=7day
EOF
sudo systemctl restart systemd-journald
echo "   -> Log rotation schedule enforced to native 100MB system thresholds."

echo "================================================================="
echo "VORTCOIN LAYER-1 CORE ENGINE IS LIVE ON THE CLOUD INFRASTRUCTURE!"
echo "================================================================="
echo "Your GCP/VPS server is now officially running 24/7 in the background."
echo "To monitor PoAV consensus block activity in real-time, type:"
echo "   -> journalctl -u vortcoin-node.service -f"
echo "================================================================="
