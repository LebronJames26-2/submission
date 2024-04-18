sudo apt-get update -y && sudo apt-get upgrade -y && sudo apt-get install -y pkg-config build-essential libudev-dev
cargo install --git https://github.com/project-serum/anchor avm --locked --force
avm install latest
avm use latest

echo "Anchor is now installed!"
