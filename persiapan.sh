sudo apt update && sudo apt upgrade -y
sudo apt install build-essential
sudo apt install pkg-config libssl-dev 
sudo apt install ca-certificates
sudo update-ca-certificates
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env