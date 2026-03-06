#!/bin/bash

# BullShift Trading Platform - Build Script
# This script builds the complete BullShift platform for Linux/macOS

set -e

echo "🚀 Building BullShift Trading Platform..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check Rust
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install Rust first."
        exit 1
    fi
    
    # Check Flutter
    if ! command -v flutter &> /dev/null; then
        print_error "Flutter is not installed. Please install Flutter first."
        exit 1
    fi
    
    # Check platform dependencies
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if ! pkg-config --exists libsecret-1; then
            print_warning "libsecret-1 is not installed. Installing..."
            sudo apt-get update && sudo apt-get install -y libsecret-1-dev
        fi
    fi
    
    print_status "Prerequisites check passed!"
}

# Build Rust backend
build_rust() {
    print_status "Building Rust backend..."
    
    cd rust
    
    # Build in release mode for performance
    cargo build --release

    print_status "Rust backend built successfully!"
    
    cd ..
}

# Setup Flutter dependencies
setup_flutter() {
    print_status "Setting up Flutter dependencies..."
    
    cd flutter
    
    # Get dependencies
    flutter pub get
    
    # Generate code if needed
    if [ -f "pubspec.yaml" ] && grep -q "objectbox" pubspec.yaml; then
        print_status "Running ObjectBox code generation..."
        dart run build_runner build --delete-conflicting-outputs
    fi
    
    cd ..
}

# Build Flutter application
build_flutter() {
    print_status "Building Flutter application..."
    
    cd flutter
    
    # Detect platform
    if [[ "$OSTYPE" == "darwin"* ]]; then
        print_status "Building for macOS..."
        flutter build macos --release
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        print_status "Building for Linux..."
        flutter build linux --release
    else
        print_error "Unsupported platform: $OSTYPE"
        exit 1
    fi
    
    print_status "Flutter application built successfully!"
    
    cd ..
}

# Run tests
run_tests() {
    print_status "Running tests..."
    
    # Rust tests
    print_status "Running Rust tests..."
    cd rust && cargo test && cd ..
    
    # Flutter tests
    print_status "Running Flutter tests..."
    cd flutter && flutter test && cd ..
    
    print_status "All tests passed!"
}

# Create distribution package
create_package() {
    print_status "Creating distribution package..."
    
    local package_name="bullshift-$(date +%Y%m%d)"
    local package_dir="dist/$package_name"
    
    # Create package directory
    mkdir -p "$package_dir"
    
    # Copy Flutter build
    if [[ "$OSTYPE" == "darwin"* ]]; then
        cp -r flutter/build/macos/Build/Products/Release/bullshift.app "$package_dir/"
    else
        cp -r flutter/build/linux/x64/release/bundle "$package_dir/bullshift"
    fi
    
    # Copy Rust library
    if [[ "$OSTYPE" == "darwin"* ]]; then
        cp rust/target/release/libbullshift_core.dylib "$package_dir/"
    else
        cp rust/target/release/libbullshift_core.so "$package_dir/"
    fi
    
    # Copy documentation
    cp README.md "$package_dir/"
    cp -r docs "$package_dir/"
    
    # Create run script
    cat > "$package_dir/run.sh" << 'EOF'
#!/bin/bash
# BullShift Trading Platform Launcher

# Get script directory
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Set library path
export LD_LIBRARY_PATH="$DIR:$LD_LIBRARY_PATH"

# Run the application
if [ -f "$DIR/bullshift.app/Contents/MacOS/bullshift" ]; then
    # macOS
    "$DIR/bullshift.app/Contents/MacOS/bullshift"
elif [ -f "$DIR/bullshift/bullshift" ]; then
    # Linux
    "$DIR/bullshift/bullshift"
else
    echo "Error: Could not find BullShift executable"
    exit 1
fi
EOF
    
    chmod +x "$package_dir/run.sh"
    
    # Create archive
    cd dist
    tar -czf "$package_name.tar.gz" "$package_name"
    cd ..
    
    print_status "Package created: dist/$package_name.tar.gz"
}

# Main build process
main() {
    print_status "Starting BullShift build process..."
    
    check_prerequisites
    build_rust
    setup_flutter
    build_flutter
    run_tests
    create_package
    
    print_status "🎉 BullShift build completed successfully!"
    print_status "Run the application with: ./dist/$package_name/run.sh"
}

# Handle command line arguments
case "${1:-}" in
    "clean")
        print_status "Cleaning build artifacts..."
        rm -rf rust/target
        rm -rf flutter/build
        rm -rf dist
        print_status "Clean completed!"
        ;;
    "test")
        run_tests
        ;;
    "package")
        create_package
        ;;
    "")
        main
        ;;
    *)
        echo "Usage: $0 [clean|test|package]"
        echo "  clean   - Remove all build artifacts"
        echo "  test    - Run tests only"
        echo "  package - Create distribution package"
        echo "  (no args) - Full build process"
        exit 1
        ;;
esac