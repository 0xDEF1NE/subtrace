#!/bin/bash

DESTINATION="/usr/share/subnerium" 

if [ "$(id -u)" != "0" ]; then
    echo "This script must be run as root."
    exit 1
fi

# Check if Python 3 is installed
if command -v python3 &>/dev/null; then
    echo "Python 3 is already installed."
else
    echo "Installing Python 3..."
    # Check the current operating system
    
    if command -v apt-get &>/dev/null; then
        # Install Python 3 on Debian/Ubuntu
        sudo apt-get install python3
    elif command -v pacman &>/dev/null; then
        # Install Python 3 on Arch
        sudo pacman -Sy python
    else
        echo "Unsupported operating system."
        exit 1
    fi
fi

# Check if pip3 is installed
if command -v pip3 &>/dev/null; then
    echo "pip3 is already installed."
else
    echo "Installing pip3..."

    if command -v apt-get &>/dev/null; then
        # Install pip3 on Debian/Ubuntu
        sudo apt-get install python3-pip
    elif command -v pacman &>/dev/null; then
        # Install pip3 on Arch
        sudo pacman -Sy python-pip
    else
        echo "Unsupported operating system."
        exit 1
    fi
fi


mkdir -p $DESTINATION
chmod +x main.py
cp -r * $DESTINATION
ln -s "$DESTINATION/main.py" /usr/bin/nerium

echo "Installation complete!"
