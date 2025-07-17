#!/bin/bash

# Build script for Litra Control Linux packages
# This script builds all Linux distribution packages

set -e

echo "🚀 Building Litra Control for Linux..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    echo "Checking dependencies..."
    
    if ! command -v pnpm &> /dev/null; then
        print_error "pnpm is required but not installed"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo is required but not installed"
        exit 1
    fi
    
    print_status "All dependencies are installed"
}

# Install npm dependencies
install_deps() {
    echo "Installing dependencies..."
    pnpm install
    print_status "Dependencies installed"
}

# Build the application
build_app() {
    echo "Building application..."
    
    # Build for Linux targets
    pnpm tauri build --target x86_64-unknown-linux-gnu
    
    print_status "Application built successfully"
}

# Display build results
show_results() {
    echo ""
    echo "🎉 Build completed successfully!"
    echo ""
    echo "Generated packages:"
    
    BUILD_DIR="src-tauri/target/x86_64-unknown-linux-gnu/release/bundle"
    
    if [ -d "$BUILD_DIR" ]; then
        # List all generated files
        find "$BUILD_DIR" -type f \( -name "*.deb" -o -name "*.rpm" -o -name "*.AppImage" \) | while read file; do
            size=$(du -h "$file" | cut -f1)
            print_status "$(basename "$file") (${size})"
        done
    else
        print_warning "Build directory not found: $BUILD_DIR"
    fi
    
    echo ""
    echo "📦 Package locations:"
    echo "  • DEB packages: $BUILD_DIR/deb/"
    echo "  • RPM packages: $BUILD_DIR/rpm/"
    echo "  • AppImage: $BUILD_DIR/appimage/"
    echo ""
    echo "📋 Next steps:"
    echo "  1. Test the packages on target systems"
    echo "  2. Upload to GitHub Releases"
    echo "  3. Update package repositories"
}

# Main execution
main() {
    echo "Building Litra Control v$(grep '"version"' src-tauri/tauri.conf.json | cut -d'"' -f4)"
    echo "Target: Linux x86_64"
    echo ""
    
    check_dependencies
    install_deps
    build_app
    show_results
}

# Run main function
main "$@"