#!/bin/bash
# PSOC Main Packaging Script
# This script orchestrates the packaging process for all platforms

set -e

# Configuration
VERSION="${1:-0.8.6}"
PLATFORMS="${2:-all}"
SKIP_BUILD="${3:-false}"
SKIP_ICONS="${4:-false}"

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRIPTS_DIR="$PROJECT_ROOT/scripts"

echo "📦 PSOC Main Packaging Script"
echo "Version: $VERSION"
echo "Platforms: $PLATFORMS"
echo "Skip build: $SKIP_BUILD"
echo "Skip icons: $SKIP_ICONS"

# Function to detect current platform
detect_platform() {
    case "$OSTYPE" in
        linux*)   echo "linux" ;;
        darwin*)  echo "macos" ;;
        msys*|cygwin*|mingw*) echo "windows" ;;
        *)        echo "unknown" ;;
    esac
}

CURRENT_PLATFORM=$(detect_platform)
echo "Current platform: $CURRENT_PLATFORM"

# Generate icons if not skipped
if [ "$SKIP_ICONS" != "true" ]; then
    echo "🎨 Generating application icons..."
    if [ -f "$SCRIPTS_DIR/generate_icons.sh" ]; then
        "$SCRIPTS_DIR/generate_icons.sh"
    else
        echo "⚠️  Icon generation script not found, skipping..."
    fi
fi

# Create packages directory
PACKAGES_DIR="$PROJECT_ROOT/packages"
mkdir -p "$PACKAGES_DIR"

# Function to package for a specific platform
package_platform() {
    local platform=$1
    local script_path="$SCRIPTS_DIR/package/${platform}.sh"
    
    echo ""
    echo "📦 Packaging for $platform..."
    
    if [ ! -f "$script_path" ]; then
        echo "❌ Packaging script not found for $platform: $script_path"
        return 1
    fi
    
    case "$platform" in
        "linux")
            bash "$script_path" "$VERSION" "release" "$SKIP_BUILD"
            ;;
        "macos")
            bash "$script_path" "$VERSION" "release" "$SKIP_BUILD"
            ;;
        "windows")
            if command -v powershell &> /dev/null; then
                powershell -ExecutionPolicy Bypass -File "$SCRIPTS_DIR/package/windows.ps1" -Version "$VERSION" -Configuration "release" -SkipBuild:$SKIP_BUILD
            elif command -v pwsh &> /dev/null; then
                pwsh -ExecutionPolicy Bypass -File "$SCRIPTS_DIR/package/windows.ps1" -Version "$VERSION" -Configuration "release" -SkipBuild:$SKIP_BUILD
            else
                echo "❌ PowerShell not found. Cannot package for Windows on this system."
                return 1
            fi
            ;;
        *)
            echo "❌ Unknown platform: $platform"
            return 1
            ;;
    esac
}

# Package based on requested platforms
case "$PLATFORMS" in
    "all")
        echo "📦 Packaging for all platforms..."
        
        # Always try to package for current platform first
        package_platform "$CURRENT_PLATFORM"
        
        # Try to package for other platforms if tools are available
        for platform in linux macos windows; do
            if [ "$platform" != "$CURRENT_PLATFORM" ]; then
                echo ""
                echo "🔄 Attempting cross-platform packaging for $platform..."
                if package_platform "$platform"; then
                    echo "✅ Successfully packaged for $platform"
                else
                    echo "⚠️  Failed to package for $platform (tools may not be available)"
                fi
            fi
        done
        ;;
    "current")
        package_platform "$CURRENT_PLATFORM"
        ;;
    *)
        # Split comma-separated platforms
        IFS=',' read -ra PLATFORM_ARRAY <<< "$PLATFORMS"
        for platform in "${PLATFORM_ARRAY[@]}"; do
            platform=$(echo "$platform" | xargs)  # Trim whitespace
            package_platform "$platform"
        done
        ;;
esac

# Generate checksums
echo ""
echo "🔐 Generating checksums..."
cd "$PACKAGES_DIR"

# Find all package files
PACKAGE_FILES=()
for dir in linux macos windows; do
    if [ -d "$dir" ]; then
        while IFS= read -r -d '' file; do
            PACKAGE_FILES+=("$file")
        done < <(find "$dir" -type f \( -name "*.tar.gz" -o -name "*.zip" -o -name "*.dmg" -o -name "*.AppImage" -o -name "*.deb" -o -name "*.rpm" -o -name "*.msi" \) -print0)
    fi
done

if [ ${#PACKAGE_FILES[@]} -gt 0 ]; then
    # Generate SHA256 checksums
    CHECKSUM_FILE="$PACKAGES_DIR/checksums-$VERSION.txt"
    echo "# PSOC $VERSION - Package Checksums" > "$CHECKSUM_FILE"
    echo "# Generated on $(date)" >> "$CHECKSUM_FILE"
    echo "" >> "$CHECKSUM_FILE"
    
    for file in "${PACKAGE_FILES[@]}"; do
        if command -v sha256sum &> /dev/null; then
            sha256sum "$file" >> "$CHECKSUM_FILE"
        elif command -v shasum &> /dev/null; then
            shasum -a 256 "$file" >> "$CHECKSUM_FILE"
        fi
    done
    
    echo "✅ Checksums generated: $CHECKSUM_FILE"
else
    echo "⚠️  No package files found for checksum generation"
fi

# Create release notes template
echo ""
echo "📝 Creating release notes template..."
RELEASE_NOTES="$PACKAGES_DIR/release-notes-$VERSION.md"

cat > "$RELEASE_NOTES" << EOF
# PSOC $VERSION Release Notes

## What's New

### Features
- Application packaging and distribution system
- Cross-platform installers (Windows MSI, macOS DMG, Linux AppImage/DEB)
- Professional application icons and branding
- Automated build and release pipeline

### Improvements
- Enhanced build system with platform-specific optimizations
- Streamlined installation process across all platforms
- Better integration with system package managers

### Bug Fixes
- Various stability improvements
- Performance optimizations

## Download

### Windows
- **Installer**: \`psoc-$VERSION-windows-installer.msi\`
- **Portable**: \`psoc-$VERSION-windows-portable.zip\`

### macOS
- **DMG Installer**: \`psoc-$VERSION-macos.dmg\`
- **Tarball**: \`psoc-$VERSION-macos.tar.gz\`

### Linux
- **AppImage**: \`psoc-$VERSION-linux.AppImage\`
- **Debian Package**: \`psoc_${VERSION}_amd64.deb\`
- **Tarball**: \`psoc-$VERSION-linux.tar.gz\`

## Installation

### Windows
1. Download the MSI installer
2. Run the installer and follow the setup wizard
3. Launch PSOC from the Start menu

### macOS
1. Download the DMG file
2. Open the DMG and drag PSOC to Applications
3. Launch PSOC from Launchpad or Applications folder

### Linux
1. Download the AppImage for universal compatibility
2. Make it executable: \`chmod +x psoc-$VERSION-linux.AppImage\`
3. Run: \`./psoc-$VERSION-linux.AppImage\`

Or install the DEB package:
\`\`\`bash
sudo dpkg -i psoc_${VERSION}_amd64.deb
sudo apt-get install -f  # Fix dependencies if needed
\`\`\`

## System Requirements

- **Windows**: Windows 10 or later (64-bit)
- **macOS**: macOS 10.15 (Catalina) or later
- **Linux**: Modern Linux distribution with GTK 3.0+

## Checksums

See \`checksums-$VERSION.txt\` for SHA256 checksums of all packages.

---

For more information, visit: https://github.com/YOUR_USERNAME/psoc
EOF

echo "✅ Release notes template created: $RELEASE_NOTES"

# Summary
echo ""
echo "🎉 Packaging completed!"
echo ""
echo "📊 Summary:"
echo "  Version: $VERSION"
echo "  Packages directory: $PACKAGES_DIR"

if [ -d "$PACKAGES_DIR" ]; then
    echo "  Generated packages:"
    find "$PACKAGES_DIR" -type f \( -name "*.tar.gz" -o -name "*.zip" -o -name "*.dmg" -o -name "*.AppImage" -o -name "*.deb" -o -name "*.rpm" -o -name "*.msi" \) -exec basename {} \; | sort | sed 's/^/    - /'
fi

echo ""
echo "📋 Next steps:"
echo "  1. Test the packages on target platforms"
echo "  2. Update release notes with specific changes"
echo "  3. Create GitHub release with packages"
echo "  4. Update documentation and website"
