#!/bin/bash

# Exit immediately if any step errors:
set -e

# Go to project root
cd $(git rev-parse --show-toplevel)


# Install Postgres
brew install postgres
# Initialize postgres DB
initdb /usr/local/var/postgres
# Create postgres user
/usr/local/opt/postgres/bin/createuser -s postgres which will just use the latest version.
# Start postgres server manually
pg_ctl -D /usr/local/var/postgres start

# Start postgres at launch
mkdir -p ~/Library/LaunchAgents
ln -sfv /usr/local/opt/postgresql/*.plist ~/Library/LaunchAgents
launchctl load ~/Library/LaunchAgents/homebrew.mxcl.postgresql.plist

# Install Rust Nighhly
# TODO

# Install cargo watcher
cargo install --force cargo-watch