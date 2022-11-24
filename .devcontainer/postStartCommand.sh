#!/bin/sh

sudo apt-get update
sudo apt-get upgrade -y

rustup --version
#rustup toolchain install nightly --component rust-analyzer-preview

rustc --version

#cargo install --locked cargo-outdated

# alias cls="clear"
# alias cc="cargo check"
# alias cb="cargo build"
# alias cr="cargo run"

echo "Done"