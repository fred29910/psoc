#!/bin/bash
# PSOC Icon Generation Script
# This script generates icons in various formats and sizes from the SVG source

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ICONS_DIR="$PROJECT_ROOT/resources/icons"
SVG_SOURCE="$ICONS_DIR/psoc.svg"

echo "🎨 PSOC Icon Generation Script"

# Check if SVG source exists
if [ ! -f "$SVG_SOURCE" ]; then
    echo "❌ SVG source not found at $SVG_SOURCE"
    exit 1
fi

# Check for required tools
check_tool() {
    if ! command -v "$1" &> /dev/null; then
        echo "❌ $1 not found. Please install it first."
        echo "   Ubuntu/Debian: sudo apt-get install $2"
        echo "   macOS: brew install $3"
        return 1
    fi
    return 0
}

# Check for ImageMagick or Inkscape
HAS_IMAGEMAGICK=false
HAS_INKSCAPE=false

if command -v convert &> /dev/null; then
    HAS_IMAGEMAGICK=true
    echo "✅ ImageMagick found"
elif command -v magick &> /dev/null; then
    HAS_IMAGEMAGICK=true
    echo "✅ ImageMagick found (magick command)"
fi

if command -v inkscape &> /dev/null; then
    HAS_INKSCAPE=true
    echo "✅ Inkscape found"
fi

if [ "$HAS_IMAGEMAGICK" = false ] && [ "$HAS_INKSCAPE" = false ]; then
    echo "❌ Neither ImageMagick nor Inkscape found."
    echo "   Please install one of them:"
    echo "   Ubuntu/Debian: sudo apt-get install imagemagick"
    echo "   macOS: brew install imagemagick"
    echo "   Or for Inkscape:"
    echo "   Ubuntu/Debian: sudo apt-get install inkscape"
    echo "   macOS: brew install inkscape"
    exit 1
fi

# Function to convert SVG to PNG using available tool
svg_to_png() {
    local size=$1
    local output=$2
    
    if [ "$HAS_INKSCAPE" = true ]; then
        inkscape --export-type=png --export-width="$size" --export-height="$size" --export-filename="$output" "$SVG_SOURCE"
    elif [ "$HAS_IMAGEMAGICK" = true ]; then
        if command -v convert &> /dev/null; then
            convert -background transparent -size "${size}x${size}" "$SVG_SOURCE" "$output"
        else
            magick -background transparent -size "${size}x${size}" "$SVG_SOURCE" "$output"
        fi
    fi
}

# Generate PNG icons in various sizes
echo "📱 Generating PNG icons..."

# Standard sizes
SIZES=(16 24 32 48 64 96 128 256 512 1024)

for size in "${SIZES[@]}"; do
    output_file="$ICONS_DIR/psoc-${size}.png"
    echo "  Generating ${size}x${size} PNG..."
    svg_to_png "$size" "$output_file"
done

# Create main icon (256x256)
cp "$ICONS_DIR/psoc-256.png" "$ICONS_DIR/psoc.png"
echo "✅ Main PNG icon created: psoc.png"

# Generate ICO file for Windows (if ImageMagick is available)
if [ "$HAS_IMAGEMAGICK" = true ]; then
    echo "🪟 Generating Windows ICO file..."
    
    # Create ICO with multiple sizes
    if command -v convert &> /dev/null; then
        convert "$ICONS_DIR/psoc-16.png" "$ICONS_DIR/psoc-24.png" "$ICONS_DIR/psoc-32.png" "$ICONS_DIR/psoc-48.png" "$ICONS_DIR/psoc-64.png" "$ICONS_DIR/psoc-128.png" "$ICONS_DIR/psoc-256.png" "$ICONS_DIR/psoc.ico"
    else
        magick "$ICONS_DIR/psoc-16.png" "$ICONS_DIR/psoc-24.png" "$ICONS_DIR/psoc-32.png" "$ICONS_DIR/psoc-48.png" "$ICONS_DIR/psoc-64.png" "$ICONS_DIR/psoc-128.png" "$ICONS_DIR/psoc-256.png" "$ICONS_DIR/psoc.ico"
    fi
    
    echo "✅ Windows ICO file created: psoc.ico"
fi

# Generate ICNS file for macOS (if iconutil is available on macOS)
if [[ "$OSTYPE" == "darwin"* ]] && command -v iconutil &> /dev/null; then
    echo "🍎 Generating macOS ICNS file..."
    
    # Create iconset directory
    ICONSET_DIR="$ICONS_DIR/psoc.iconset"
    rm -rf "$ICONSET_DIR"
    mkdir -p "$ICONSET_DIR"
    
    # Copy icons with proper naming for iconset
    cp "$ICONS_DIR/psoc-16.png" "$ICONSET_DIR/icon_16x16.png"
    cp "$ICONS_DIR/psoc-32.png" "$ICONSET_DIR/icon_16x16@2x.png"
    cp "$ICONS_DIR/psoc-32.png" "$ICONSET_DIR/icon_32x32.png"
    cp "$ICONS_DIR/psoc-64.png" "$ICONSET_DIR/icon_32x32@2x.png"
    cp "$ICONS_DIR/psoc-128.png" "$ICONSET_DIR/icon_128x128.png"
    cp "$ICONS_DIR/psoc-256.png" "$ICONSET_DIR/icon_128x128@2x.png"
    cp "$ICONS_DIR/psoc-256.png" "$ICONSET_DIR/icon_256x256.png"
    cp "$ICONS_DIR/psoc-512.png" "$ICONSET_DIR/icon_256x256@2x.png"
    cp "$ICONS_DIR/psoc-512.png" "$ICONSET_DIR/icon_512x512.png"
    cp "$ICONS_DIR/psoc-1024.png" "$ICONSET_DIR/icon_512x512@2x.png"
    
    # Generate ICNS
    iconutil -c icns "$ICONSET_DIR" -o "$ICONS_DIR/psoc.icns"
    
    # Clean up iconset directory
    rm -rf "$ICONSET_DIR"
    
    echo "✅ macOS ICNS file created: psoc.icns"
fi

# Create favicon for web
echo "🌐 Generating favicon..."
cp "$ICONS_DIR/psoc-32.png" "$ICONS_DIR/favicon-32x32.png"
cp "$ICONS_DIR/psoc-16.png" "$ICONS_DIR/favicon-16x16.png"

if [ "$HAS_IMAGEMAGICK" = true ]; then
    if command -v convert &> /dev/null; then
        convert "$ICONS_DIR/psoc-16.png" "$ICONS_DIR/psoc-32.png" "$ICONS_DIR/favicon.ico"
    else
        magick "$ICONS_DIR/psoc-16.png" "$ICONS_DIR/psoc-32.png" "$ICONS_DIR/favicon.ico"
    fi
    echo "✅ Favicon created: favicon.ico"
fi

# Generate app store icons (if needed)
echo "📱 Generating app store icons..."
svg_to_png 180 "$ICONS_DIR/app-icon-180.png"  # iOS App Store
svg_to_png 1024 "$ICONS_DIR/app-icon-1024.png"  # iOS App Store large

echo "🎉 Icon generation completed!"
echo "Generated icons in: $ICONS_DIR"
echo ""
echo "Available icons:"
ls -la "$ICONS_DIR"/*.png "$ICONS_DIR"/*.ico "$ICONS_DIR"/*.icns 2>/dev/null || true
