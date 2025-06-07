#!/bin/bash
# PSOC Linux Packaging Script
# This script creates Linux packages (AppImage, .deb, .rpm, tarball)

set -e

# Configuration
VERSION="${1:-0.8.6}"
CONFIGURATION="${2:-release}"
SKIP_BUILD="${3:-false}"

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/target/release"
PACKAGE_DIR="$PROJECT_ROOT/packages/linux"
RESOURCES_DIR="$PROJECT_ROOT/resources"

echo "🐧 PSOC Linux Packaging Script"
echo "Version: $VERSION"
echo "Configuration: $CONFIGURATION"

# Create package directory
mkdir -p "$PACKAGE_DIR"

# Build the application
if [ "$SKIP_BUILD" != "true" ]; then
    echo "🔨 Building PSOC for Linux..."
    cd "$PROJECT_ROOT"
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        echo "❌ Cargo not found. Please install Rust toolchain."
        exit 1
    fi
    
    # Build release version
    cargo build --release --target x86_64-unknown-linux-gnu
    
    echo "✅ Build completed successfully!"
fi

# Verify executable exists
EXECUTABLE_PATH="$BUILD_DIR/psoc"
if [ ! -f "$EXECUTABLE_PATH" ]; then
    echo "❌ Executable not found at $EXECUTABLE_PATH"
    exit 1
fi

# Strip the binary to reduce size
strip "$EXECUTABLE_PATH"

# Create desktop file
echo "📄 Creating desktop file..."
DESKTOP_FILE="$RESOURCES_DIR/desktop/psoc.desktop"
mkdir -p "$(dirname "$DESKTOP_FILE")"

cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Name=PSOC Image Editor
Comment=Professional Simple Open-source image editor
Exec=psoc %F
Icon=psoc
Terminal=false
Type=Application
Categories=Graphics;Photography;RasterGraphics;
MimeType=image/png;image/jpeg;image/tiff;image/bmp;image/gif;application/x-psoc;
StartupNotify=true
StartupWMClass=psoc
Keywords=image;editor;graphics;photo;picture;
EOF

# Create AppImage
echo "📦 Creating AppImage..."
create_appimage() {
    local appdir="$PACKAGE_DIR/PSOC.AppDir"
    
    # Clean and create AppDir structure
    rm -rf "$appdir"
    mkdir -p "$appdir/usr/bin"
    mkdir -p "$appdir/usr/share/applications"
    mkdir -p "$appdir/usr/share/icons/hicolor/256x256/apps"
    mkdir -p "$appdir/usr/share/pixmaps"
    
    # Copy executable
    cp "$EXECUTABLE_PATH" "$appdir/usr/bin/psoc"
    chmod +x "$appdir/usr/bin/psoc"
    
    # Copy desktop file
    cp "$DESKTOP_FILE" "$appdir/usr/share/applications/"
    cp "$DESKTOP_FILE" "$appdir/"
    
    # Copy icon
    if [ -f "$RESOURCES_DIR/icons/psoc.png" ]; then
        cp "$RESOURCES_DIR/icons/psoc.png" "$appdir/usr/share/icons/hicolor/256x256/apps/"
        cp "$RESOURCES_DIR/icons/psoc.png" "$appdir/usr/share/pixmaps/"
        cp "$RESOURCES_DIR/icons/psoc.png" "$appdir/"
    fi
    
    # Create AppRun script
    cat > "$appdir/AppRun" << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin/:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib/:${LD_LIBRARY_PATH}"
exec "${HERE}/usr/bin/psoc" "$@"
EOF
    chmod +x "$appdir/AppRun"
    
    # Download appimagetool if not available
    APPIMAGETOOL="$PACKAGE_DIR/appimagetool"
    if [ ! -f "$APPIMAGETOOL" ]; then
        echo "📥 Downloading appimagetool..."
        wget -O "$APPIMAGETOOL" "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
        chmod +x "$APPIMAGETOOL"
    fi
    
    # Create AppImage
    cd "$PACKAGE_DIR"
    ARCH=x86_64 "$APPIMAGETOOL" "PSOC.AppDir" "psoc-$VERSION-linux.AppImage"
    
    echo "✅ AppImage created: $PACKAGE_DIR/psoc-$VERSION-linux.AppImage"
}

# Create tarball
echo "📦 Creating tarball..."
create_tarball() {
    local tarball_dir="$PACKAGE_DIR/psoc-$VERSION-linux"
    
    # Clean and create directory
    rm -rf "$tarball_dir"
    mkdir -p "$tarball_dir/bin"
    mkdir -p "$tarball_dir/share/applications"
    mkdir -p "$tarball_dir/share/icons"
    
    # Copy files
    cp "$EXECUTABLE_PATH" "$tarball_dir/bin/"
    cp "$DESKTOP_FILE" "$tarball_dir/share/applications/"
    
    if [ -f "$RESOURCES_DIR/icons/psoc.png" ]; then
        cp "$RESOURCES_DIR/icons/psoc.png" "$tarball_dir/share/icons/"
    fi
    
    # Copy documentation
    for doc in README.md LICENSE-MIT LICENSE-APACHE CHANGELOG.md; do
        if [ -f "$PROJECT_ROOT/$doc" ]; then
            cp "$PROJECT_ROOT/$doc" "$tarball_dir/"
        fi
    done
    
    # Create install script
    cat > "$tarball_dir/install.sh" << 'EOF'
#!/bin/bash
# PSOC Installation Script

set -e

PREFIX="${1:-/usr/local}"
echo "Installing PSOC to $PREFIX"

# Create directories
sudo mkdir -p "$PREFIX/bin"
sudo mkdir -p "$PREFIX/share/applications"
sudo mkdir -p "$PREFIX/share/icons/hicolor/256x256/apps"

# Copy files
sudo cp bin/psoc "$PREFIX/bin/"
sudo chmod +x "$PREFIX/bin/psoc"
sudo cp share/applications/psoc.desktop "$PREFIX/share/applications/"

if [ -f share/icons/psoc.png ]; then
    sudo cp share/icons/psoc.png "$PREFIX/share/icons/hicolor/256x256/apps/"
fi

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    sudo update-desktop-database "$PREFIX/share/applications"
fi

echo "PSOC installed successfully!"
echo "You can now run 'psoc' from the command line or find it in your applications menu."
EOF
    chmod +x "$tarball_dir/install.sh"
    
    # Create tarball
    cd "$PACKAGE_DIR"
    tar -czf "psoc-$VERSION-linux.tar.gz" "psoc-$VERSION-linux"
    
    echo "✅ Tarball created: $PACKAGE_DIR/psoc-$VERSION-linux.tar.gz"
}

# Create .deb package
echo "📦 Creating .deb package..."
create_deb() {
    local deb_dir="$PACKAGE_DIR/psoc-deb"
    local control_dir="$deb_dir/DEBIAN"
    
    # Clean and create directory structure
    rm -rf "$deb_dir"
    mkdir -p "$control_dir"
    mkdir -p "$deb_dir/usr/bin"
    mkdir -p "$deb_dir/usr/share/applications"
    mkdir -p "$deb_dir/usr/share/icons/hicolor/256x256/apps"
    
    # Copy files
    cp "$EXECUTABLE_PATH" "$deb_dir/usr/bin/"
    cp "$DESKTOP_FILE" "$deb_dir/usr/share/applications/"
    
    if [ -f "$RESOURCES_DIR/icons/psoc.png" ]; then
        cp "$RESOURCES_DIR/icons/psoc.png" "$deb_dir/usr/share/icons/hicolor/256x256/apps/"
    fi
    
    # Create control file
    cat > "$control_dir/control" << EOF
Package: psoc
Version: $VERSION
Section: graphics
Priority: optional
Architecture: amd64
Depends: libc6, libgtk-3-0, libxcb-render0, libxcb-shape0, libxcb-xfixes0, libxkbcommon0
Maintainer: PSOC Development Team <dev@psoc.project>
Description: Professional Simple Open-source image editor
 PSOC is a Photoshop-like image editor built with Rust, providing
 professional image editing capabilities with a modern interface.
 .
 Features include layers, blend modes, adjustments, filters, and more.
EOF
    
    # Create postinst script
    cat > "$control_dir/postinst" << 'EOF'
#!/bin/bash
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database /usr/share/applications
fi
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -t /usr/share/icons/hicolor
fi
EOF
    chmod +x "$control_dir/postinst"
    
    # Build .deb package
    cd "$PACKAGE_DIR"
    dpkg-deb --build psoc-deb "psoc_${VERSION}_amd64.deb"
    
    echo "✅ .deb package created: $PACKAGE_DIR/psoc_${VERSION}_amd64.deb"
}

# Execute packaging functions
if command -v wget &> /dev/null; then
    create_appimage
else
    echo "⚠️  wget not found, skipping AppImage creation"
fi

create_tarball

if command -v dpkg-deb &> /dev/null; then
    create_deb
else
    echo "⚠️  dpkg-deb not found, skipping .deb package creation"
fi

echo "🎉 Linux packaging completed!"
echo "Packages created in: $PACKAGE_DIR"
