#!/bin/bash
# PSOC macOS Packaging Script
# This script creates macOS .app bundles and .dmg installers

set -e

# Configuration
VERSION="${1:-0.8.6}"
CONFIGURATION="${2:-release}"
SKIP_BUILD="${3:-false}"
TARGET="${4:-x86_64-apple-darwin}"

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/target/$TARGET/release"
PACKAGE_DIR="$PROJECT_ROOT/packages/macos"
RESOURCES_DIR="$PROJECT_ROOT/resources"

echo "🍎 PSOC macOS Packaging Script"
echo "Version: $VERSION"
echo "Configuration: $CONFIGURATION"

# Create package directory
mkdir -p "$PACKAGE_DIR"

# Build the application
if [ "$SKIP_BUILD" != "true" ]; then
    echo "🔨 Building PSOC for macOS..."
    cd "$PROJECT_ROOT"
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        echo "❌ Cargo not found. Please install Rust toolchain."
        exit 1
    fi
    
    # Build release version
    cargo build --release --target x86_64-apple-darwin
    
    echo "✅ Build completed successfully!"
fi

# Verify executable exists
EXECUTABLE_PATH="$BUILD_DIR/psoc"

if [ ! -f "$EXECUTABLE_PATH" ]; then
    echo "❌ Executable not found at $EXECUTABLE_PATH"
    exit 1
fi

# Create .app bundle
echo "📦 Creating .app bundle..."

APP_NAME="PSOC.app"
APP_DIR="$PACKAGE_DIR/$APP_NAME"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_APP_DIR="$CONTENTS_DIR/Resources"

# Clean and create app directory structure
rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_APP_DIR"

# Copy executable
cp "$EXECUTABLE_PATH" "$MACOS_DIR/psoc"
chmod +x "$MACOS_DIR/psoc"

# Create Info.plist
cat > "$CONTENTS_DIR/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>psoc</string>
    <key>CFBundleIdentifier</key>
    <string>com.psoc.imageEditor</string>
    <key>CFBundleName</key>
    <string>PSOC</string>
    <key>CFBundleDisplayName</key>
    <string>PSOC Image Editor</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>PSOC</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeExtensions</key>
            <array>
                <string>png</string>
                <string>jpg</string>
                <string>jpeg</string>
                <string>tiff</string>
                <string>psoc</string>
            </array>
            <key>CFBundleTypeName</key>
            <string>Image Files</string>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>LSHandlerRank</key>
            <string>Owner</string>
        </dict>
    </array>
</dict>
</plist>
EOF

# Copy application icon if it exists
if [ -f "$RESOURCES_DIR/icons/psoc.icns" ]; then
    cp "$RESOURCES_DIR/icons/psoc.icns" "$RESOURCES_APP_DIR/"
    # Add icon reference to Info.plist
    /usr/libexec/PlistBuddy -c "Add :CFBundleIconFile string psoc.icns" "$CONTENTS_DIR/Info.plist" 2>/dev/null || true
fi

# Copy resources
if [ -d "$RESOURCES_DIR" ]; then
    cp -r "$RESOURCES_DIR"/* "$RESOURCES_APP_DIR/" 2>/dev/null || true
fi

echo "✅ .app bundle created: $APP_DIR"

# Create DMG installer
echo "💿 Creating DMG installer..."

DMG_NAME="psoc-$VERSION-macos.dmg"
DMG_PATH="$PACKAGE_DIR/$DMG_NAME"
TEMP_DMG_PATH="$PACKAGE_DIR/temp.dmg"

# Remove existing DMG
rm -f "$DMG_PATH" "$TEMP_DMG_PATH"

# Create temporary DMG
hdiutil create -srcfolder "$APP_DIR" -volname "PSOC $VERSION" -fs HFS+ -fsargs "-c c=64,a=16,e=16" -format UDRW -size 100m "$TEMP_DMG_PATH"

# Mount the DMG
MOUNT_DIR=$(hdiutil attach -readwrite -noverify -noautoopen "$TEMP_DMG_PATH" | egrep '^/dev/' | sed 1q | awk '{print $3}')

# Create Applications symlink
ln -sf /Applications "$MOUNT_DIR/Applications"

# Set DMG window properties (if osascript is available)
if command -v osascript &> /dev/null && [ -z "$CI" ]; then
    osascript << EOF
tell application "Finder"
    tell disk "PSOC $VERSION"
        open
        set current view of container window to icon view
        set toolbar visible of container window to false
        set statusbar visible of container window to false
        set the bounds of container window to {400, 100, 900, 400}
        set theViewOptions to the icon view options of container window
        set arrangement of theViewOptions to not arranged
        set icon size of theViewOptions to 128
        set position of item "PSOC.app" of container window to {150, 150}
        set position of item "Applications" of container window to {350, 150}
        update without registering applications
        delay 2
        close
    end tell
end tell
EOF
else
    if [ -n "$CI" ]; then
        echo "Skipping DMG appearance customization in CI environment."
    else
        echo "osascript not found, skipping DMG appearance customization."
    fi
fi

# Unmount the DMG
hdiutil detach "$MOUNT_DIR"

# Convert to compressed DMG
hdiutil convert "$TEMP_DMG_PATH" -format UDZO -imagekey zlib-level=9 -o "$DMG_PATH"

# Clean up
rm -f "$TEMP_DMG_PATH"

echo "✅ DMG installer created: $DMG_PATH"

# Create tarball for distribution
echo "📦 Creating tarball..."
TARBALL_NAME="psoc-$VERSION-macos.tar.gz"
TARBALL_PATH="$PACKAGE_DIR/$TARBALL_NAME"

cd "$PACKAGE_DIR"
tar -czf "$TARBALL_NAME" "$APP_NAME"

echo "✅ Tarball created: $TARBALL_PATH"

echo "🎉 macOS packaging completed!"
echo "Packages created in: $PACKAGE_DIR"
