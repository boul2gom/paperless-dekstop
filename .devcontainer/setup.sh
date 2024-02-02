apt-get update
apt-get upgrade -y
apt-get install -y nala

nala install -y curl git jq sudo zsh vim gnupg gnupg2 openssl build-essential ca-certificates

## Install common components
rustup component add rustfmt
rustup component add clippy

cargo install cargo-nextest
cargo install cargo-update
cargo install cargo-expand
cargo install cargo-readme
cargo install cargo-watch
cargo install cargo-edit

## Install Node.js and pnpm
NODE_MAJOR=20
curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list

nala update
nala install -y nodejs
curl -fsSL https://get.pnpm.io/install.sh | sh -

## Setup and install oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
cp -R /root/.oh-my-zsh /home/$USERNAME
cp /root/.zshrc /home/$USERNAME
sed -i -e "s/\/root\/.oh-my-zsh/\/home\/$USERNAME\/.oh-my-zsh/g" /home/$USERNAME/.zshrc
chown -R $USER_UID:$USER_GID /home/$USERNAME/.oh-my-zsh /home/$USERNAME/.zshrc