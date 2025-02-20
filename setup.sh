#!/bin/bash

# Ensure script is run as root
if [ "$(id -u)" != "0" ]; then
   echo "This script must be run as root (use sudo)." 
   exit 1
fi

# Install necessary packages based on the package manager available
if [ -x "$(command -v apt-get)" ]; then
    echo "Detected Debian-based system. Installing dependencies using apt-get."
    sudo apt-get update
    sudo apt-get install -y git python3-pip llvm cmake curl

elif [ -x "$(command -v yum)" ]; then
    echo "Detected Red Hat-based system. Installing dependencies using yum."
    sudo yum install -y git python3-pip llvm cmake curl

elif [ -x "$(command -v pacman)" ]; then
    echo "Detected Arch-based system. Installing dependencies using pacman."
    sudo pacman -Sy --needed git python-pip llvm cmake curl

elif [ -x "$(command -v brew)" ]; then
    echo "Detected macOS. Installing dependencies using Homebrew."
    brew install git python3 llvm cmake curl

else
    echo "Unsupported OS or package manager. Please install dependencies manually."
    exit 1
fi

# Install Python dependencies
echo "Installing Python dependencies..."
pip3 install mako

# Build and run the project using Cargo
echo "Building the project..."
cargo run
