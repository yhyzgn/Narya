#!/bin/bash

# Narya Build Dependencies Installer for Fedora
# Designed for Tauri v2 development

echo "Installing Tauri v2 dependencies for Fedora..."

sudo dnf install -y \
    webkit2gtk4.1-devel \
    gtk3-devel \
    libsoup3-devel \
    libayatana-appindicator-gtk3-devel \
    librsvg2-devel \
    openssl-devel \
    gcc \
    gcc-c++ \
    make \
    cmake \
    perl-FindBin \
    perl-File-Compare

if [ $? -eq 0 ]; then
    echo "Dependencies installed successfully."
else
    echo "Error installing dependencies. Please check the output above."
    exit 1
fi
