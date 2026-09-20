#!/bin/bash

# =========================================================================
# VORTCOIN NETWORK - AUTOMATED NODE INSTALLER FOR LINUX VPS & MINERS
# Minimal Hardware Requirement: 1 Core CPU, 1GB RAM, Standard Connection
# =========================================================================

# Preserving original user variables before executing the sudo command
ACTUAL_USER=${SUDO_USER:-$USER}
ACTUAL_HOME=$(eval echo ~$ACTUAL_USER)

clear
echo "================================================================="
echo "VORTCOIN NETWORK (VORT) - INSTALLER NODE GLOBAL MINERS (PoAV)"
echo "================================================================="
echo "Checking your Linux system dependencies..."

# 1. Update the system and install basic components if they are not already present
sudo apt-get update -y && sudo apt-get install -y curl git build-essential libssl-dev pkg-config

# --- P2P Network Risk Mitigation Firewall Automation ---
echo "Configuring Linux network port security gateway..."

# A. Detection and configuration using UFW (Uncomplicated Firewall)
if command -v ufw &> /dev/null; then
    echo "   [UFW Detection] Opening L1 Consensus Port..."
    sudo ufw allow 3690/tcp comment 'VORTCOIN P2P Socket'
    sudo ufw allow 3690/udp comment 'VORTCOIN P2P Socket'
    sudo ufw allow 8545/tcp comment 'VORTCOIN RPC API Gateway'
    # If UFW is active, reload immediately
    sudo ufw status | grep -q "active" && sudo ufw reload
fi

# B. Detection and backup configuration using iptables
if command -v iptables &> /dev/null; then
    echo "   [iptables Detection] Injecting binary packet access rules..."
    sudo iptables -C INPUT -p tcp --dport 3690 -j ACCEPT >/dev/null 2>&1 || sudo iptables -A INPUT -p tcp --dport 3690 -j ACCEPT -m comment --comment "VORTCOIN P2P TCP"
    sudo iptables -C INPUT -p udp --dport 3690 -j ACCEPT >/dev/null 2>&1 || sudo iptables -A INPUT -p udp --dport 3690 -j ACCEPT -m comment --comment "VORTCOIN P2P UDP"
    sudo iptables -C INPUT -p tcp --dport 8545 -j ACCEPT >/dev/null 2>&1 || sudo iptables -A INPUT -p tcp --dport 8545 -j ACCEPT -m comment --comment "VORTCOIN RPC API"
    
    # Ensuring iptables rules are persistent if the saving utility is installed
    if command -v iptables-save &> /dev/null; then
        sudo iptables-save | sudo tee /etc/iptables/rules.v4 > /dev/null 2>&1
    fi
fi
echo "Network ports 3690 (P2P) and 8545 (RPC) have been officially opened."
# -------------------------------------------------------------------------

# 2. Automatically install the Rust compiler (if not already installed)
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Installing Rust Compiler (industry standard)..."
    # Running Rust installation as the original user (not root)
    sudo -u "$ACTUAL_USER" curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sudo -u "$ACTUAL_USER" sh -s -- -y
    # Loads the env path so that the current script session recognizes cargo
    source "$ACTUAL_HOME/.cargo/env"
else
    echo "The Rust compiler is ready for use."
fi

# 3. Downloading the VORTCOIN Core blockchain code from the official repository
echo "Downloading VORTCOIN Core Engine from the global repository..."
cd "$ACTUAL_HOME" || exit
if [ -d "$ACTUAL_HOME/vortcoin_miner_node" ]; then
    echo "Directory vortcoin_miner_node already exists. Pulling latest updates..."
    cd "$ACTUAL_HOME/vortcoin_miner_node" || exit
    sudo -u "$ACTUAL_USER" git pull
else
    # CLone from the official repository
    sudo -u "$ACTUAL_USER" git clone https://github.com/vortcoin/vortcoin.git "$ACTUAL_HOME/vortcoin_miner_node"
    cd "$ACTUAL_HOME/vortcoin_miner_node" || exit
fi

# 4. Compiling Release Binaries (Build Once, Run Forever)
echo "Compiling VORTCOIN Core binary architecture (Please wait)..."
# Ensure the cargo command is called with the full path for safety
sudo -u "$ACTUAL_USER" "$ACTUAL_HOME/.cargo/bin/cargo" build --release

# 5. Starting the PoAV Validator Node in the background using Systemd
echo "Registering the VORTCOIN Node as a Linux background service..."

# Using safe 'cat' redirection to prevent variables within the service file from being expanded into empty text
sudo tee /etc/systemd/system/vortcoin-node.service > /dev/null << EOF
[Unit]
Description=VORTCOIN Network PoAV Validator Node
After=network.target

[Service]
User=$ACTUAL_USER
WorkingDirectory=$ACTUAL_HOME/vortcoin_miner_node
ExecStart=$ACTUAL_HOME/vortcoin_miner_node/target/release/vortcoin-cli node-start
Restart=always
RestartSec=10
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:$ACTUAL_HOME/.cargo/bin"

[Install]
WantedBy=multi-user.target
EOF

# 6. Starting the Node Service in Real-time
echo "Starting the VORTCOIN mining engine..."
sudo systemctl daemon-reload
sudo systemctl enable vortcoin-node.service
sudo systemctl start vortcoin-node.service

# Configuring log retention natively directly in systemd-journald (Safe & Efficient)
echo "Configuring native log rotation thresholds..."
sudo mkdir -p /etc/systemd/journald.conf.d
sudo tee /etc/systemd/journald.conf.d/vortcoin.conf > /dev/null << EOF
[Journal]
SystemMaxUse=100M
SystemMaxFileSize=20M
MaxRetentionSec=7day
EOF

# Restart the log service to apply the configuration
sudo systemctl restart systemd-journald

echo "================================================================="
echo "VORTCOIN NODE SUCCESSFULLY RUNNING IN THE BACKGROUND!"
echo "================================================================="
echo "Your standard CPU is now officially validating the global network."
echo "The coin's base price is calculated fairly based on your node's energy."
echo "To view mining activity logs, type:"
echo "   -> journalctl -u vortcoin-node.service -f"
echo "================================================================="
