#!/bin/bash

echo "Setting up Real-Time Queue & Server Load Simulator..."

# Build Rust components
echo "Building Rust engine..."
cd rust-engine
cargo build --release
cd ..

# Install Ruby dependencies
echo "Installing Ruby dependencies..."
bundle install

# Make simulator executable
chmod +x simulator.rb

echo "Setup complete! Run './simulator.rb' to start the simulation."
