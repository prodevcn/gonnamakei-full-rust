#################################################################################
# This file is executed inside the docker container that will compile the code. #
#################################################################################

# Fetch packages.
cargo fetch

# Fix for an error in aragors. Remove when fixed.
sed -i -E "s/_phantom/phantom/g" /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/arangors-0.5.0/src/aql.rs

# Install some dependencies.
apt-get update
apt-get install pkg-config libudev-dev -y
pkg-config --libs --cflags libudev
cargo build --release