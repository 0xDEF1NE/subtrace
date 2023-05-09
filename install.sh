#!/bin/bash

DESTINATION="/usr/share/subnerium" 

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
        echo "[ERROR] Unsupported operating system."
        exit 1
    fi
fi

# Check if pip3 is installed
if command -v pip3 &>/dev/null; then
    echo "[INFO] pip3 is already installed."
else
    echo "Installing pip3..."

    if command -v apt-get &>/dev/null; then
        # Install pip3 on Debian/Ubuntu
        sudo apt-get install python3-pip
    elif command -v pacman &>/dev/null; then
        # Install pip3 on Arch
        sudo pacman -Sy python-pip
    else
        echo "[ERROR] Unsupported operating system."
        exit 1
    fi
fi

pip3 install -r requirements.txt

if [ -d "$DESTINATION" ]; then
    echo "[INFO] The '$DESTINATION' folder already exists."
else
    sudo mkdir -p $DESTINATION
fi

if [ -d "$HOME/.config/subnerium" ]; then
    echo "[INFO] The '$HOME/.config/subnerium' folder already exists."
else
    mkdir -p "$HOME/.config/subnerium"
fi

if [ -f "$HOME/.config/subnerium/apikeys.yaml" ]; then
    echo "[INFO] The '$HOME/.config/subnerium/apikeys.yaml' file already exists."
else
    echo "Moving apikeys.yaml to $HOME/.config/subnerium/apikeys.yaml"
    cp apikeys.yaml $HOME/.config/subnerium/apikeys.yaml
fi

chmod +x main.py

cp -r templates/ $HOME/.config/subnerium/templates/
sudo cp -r * $DESTINATION

if [ -h "$DESTINATION/main.py" ]; then
    echo "/usr/bin/subnerium Symbolic link exists"
else
    sudo ln -s "$DESTINATION/main.py" /usr/bin/subnerium
fi

echo "Installation complete!"
echo "Run: subnerium --help"
