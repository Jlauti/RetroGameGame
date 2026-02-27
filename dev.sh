#!/usr/bin/env bash

# Ensure we're in the repository root
cd "$(git rev-parse --show-toplevel)" || exit 1

OPTIONS=(
    "Run Game (cargo run)"
    "Run Built Binary (target/debug/retro-game-game)"
    "Local Compile (cargo build)"
    "Remote Compile (build on 10.0.0.10)"
    "Remote Check (check on 10.0.0.10)"
    "Check (cargo check)"
    "Test (cargo test)"
    "Format (cargo fmt)"
    "Lint (cargo clippy)"
    "Exit"
)

NUM_OPTIONS=${#OPTIONS[@]}
SELECTED=0

draw_menu() {
    clear
    echo -e "\033[1;36m"
    cat << "EOF"
  _____      _             _____                     _____                     
 |  __ \    | |           / ____|                   / ____|                    
 | |__) |___| |_ _ __ ___| |  __  __ _ _ __ ___  ___| |  __  __ _ _ __ ___  ___ 
 |  _  // _ \ __| '__/ _ \ | |_ |/ _` | '_ ` _ \/ _ \ | |_ |/ _` | '_ ` _ \/ _ \
 | | \ \  __/ |_| | | (_) | |__| | (_| | | | | | |  __/ |__| | (_| | | | | | |  __/
 |_|  \_\___|\__|_|  \___/ \_____|\__,_|_| |_| |_|\___|\_____|\__,_|_| |_| |_|\___|

EOF
    echo -e "\033[0m"
    echo -e "\033[1;33mUse UP/DOWN arrows to navigate, ENTER to select.\033[0m"
    echo ""

    for i in "${!OPTIONS[@]}"; do
        if [ "$i" -eq "$SELECTED" ]; then
            echo -e "  \033[1;32m> ${OPTIONS[$i]}\033[0m"
        else
            echo "    ${OPTIONS[$i]}"
        fi
    done
}

run_action() {
    clear
    case $SELECTED in
        0)
            echo -e "\033[1;34mRunning game (cargo run)...\033[0m"
            cargo run
            ;;
        1)
            echo -e "\033[1;34mRunning built binary...\033[0m"
            if [ -x "./target/debug/retro-game-game" ]; then
                ./target/debug/retro-game-game
            else
                echo -e "\033[1;31mError: Binary not found or not executable. Compile it first.\033[0m"
            fi
            ;;
        2)
            echo -e "\033[1;34mCompiling locally (cargo build)...\033[0m"
            cargo build
            ;;
        3)
            echo -e "\033[1;34mCompiling remotely (build on 10.0.0.10)...\033[0m"
            # Ensure proper execution and return to script context
            env SSH_ID_FILE=~/.ssh/ghost_proxmox_ed25519 ./scripts/remote_compiler/remote-cargo.sh root@10.0.0.10 build
            ;;
        4)
            echo -e "\033[1;34mChecking remotely (check on 10.0.0.10)...\033[0m"
            env SSH_ID_FILE=~/.ssh/ghost_proxmox_ed25519 ./scripts/remote_compiler/remote-cargo.sh root@10.0.0.10 check
            ;;
        5)
            echo -e "\033[1;34mRunning cargo check...\033[0m"
            cargo check
            ;;
        6)
            echo -e "\033[1;34mRunning cargo test...\033[0m"
            cargo test
            ;;
        7)
            echo -e "\033[1;34mRunning cargo fmt...\033[0m"
            cargo fmt
            ;;
        8)
            echo -e "\033[1;34mRunning cargo clippy...\033[0m"
            cargo clippy
            ;;
        9)
            clear
            exit 0
            ;;
    esac

    echo ""
    read -rsn1 -p "Press any key to return to the menu..."
}

while true; do
    draw_menu
    
    # Read arrow keys (escape sequence)
    read -rsn1 key
    if [[ $key == $'\x1b' ]]; then
        read -rsn2 key # read standard 2 character escape sequence
        if [[ $key == "[A" ]]; then # Up
            ((SELECTED--))
            if [ "$SELECTED" -lt 0 ]; then
                SELECTED=$((NUM_OPTIONS - 1))
            fi
        elif [[ $key == "[B" ]]; then # Down
            ((SELECTED++))
            if [ "$SELECTED" -ge "$NUM_OPTIONS" ]; then
                SELECTED=0
            fi
        fi
    elif [[ $key == "" ]]; then # Enter key
        run_action
    fi
done
