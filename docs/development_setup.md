# Development Environment Setup

## 🚀 Quick Start

This guide helps you set up the complete development environment for BullShift.

## Prerequisites

### Required Tools

#### 1. Flutter SDK (4.0+)
```bash
# Install Flutter
# macOS
brew install --cask flutter

# Linux (Ubuntu/Debian)
sudo snap install flutter --classic

# Windows
# Download from https://flutter.dev/docs/get-started/install/windows

# Verify installation
flutter --version
flutter doctor
```

#### 2. Rust Toolchain (1.70+)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### 3. Platform Dependencies

**macOS:**
```bash
# Xcode Command Line Tools
xcode-select --install

# Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Additional dependencies
brew install cmake pkg-config
```

**Linux (Ubuntu/Debian):**
```bash
# Update package manager
sudo apt update

# Install required packages
sudo apt install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libsecret-1-dev

# For Flutter
sudo apt install -y clang cmake ninja-build pkg-config libgtk-3-dev liblzma-dev
```

**Windows:**
```bash
# Install Microsoft Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/

# Install CMake
# Download from: https://cmake.org/download/

# Install Git for Windows
# Download from: https://git-scm.com/download/win
```

## Development Setup

### 1. Clone Repository
```bash
git clone <repository-url>
cd bullshift
```

### 2. Build Rust Backend
```bash
cd rust
cargo build --release
```

### 3. Setup Flutter
```bash
cd ../flutter
flutter pub get

# Verify dependencies
flutter doctor -v
```

### 4. Run Application
```bash
# Linux
flutter run -d linux

# macOS
flutter run -d macos

# Windows
flutter run -d windows

# Mobile (if configured)
flutter run -d ios
flutter run -d android
```

## IDE Configuration

### VS Code (Recommended)

**Extensions:**
- Dart
- Flutter
- Rust
- CMake Tools
- GitLens

**Settings:**
```json
{
    "dart.flutterSdkPath": "/path/to/flutter",
    "rust-analyzer.cargo.features": "all"
}
```

### JetBrains Fleet/IDEA

**Plugins:**
- Dart
- Flutter
- Rust

## Testing

### Rust Tests
```bash
cd rust
cargo test
cargo test --release  # For performance tests
```

### Flutter Tests
```bash
cd flutter
flutter test
flutter test --coverage  # Generate coverage report
```

## Build Process

### Development Build
```bash
# Fast development build
cd rust && cargo build
cd ../flutter && flutter build debug
```

### Production Build
```bash
# Optimized production build
cd rust && cargo build --release
cd ../flutter && flutter build release
```

### Package Distribution
```bash
# Create distributable packages
cd flutter
flutter build linux --release
flutter build macos --release
flutter build windows --release
flutter build apk --release
flutter build ios --release
```

## Troubleshooting

### Common Issues

**Flutter Doctor Issues:**
```bash
# Run verbose doctor
flutter doctor -v

# Accept Android licenses (if needed)
flutter doctor --android-licenses
```

**Rust Compilation Issues:**
```bash
# Update toolchain
rustup update

# Clean build cache
cargo clean && cargo build --release
```

**Missing Dependencies:**
```bash
# Linux: Install missing packages
sudo apt install libsecret-1-dev libwebkit2gtk-4.0-dev

# macOS: Install via Homebrew
brew install pkg-config cmake

# Windows: Install Visual Studio Build Tools
```

### Platform-Specific Notes

**macOS Apple Silicon:**
```bash
# Install Rosetta 2 for Intel compatibility
sudo softwareupdate --install-rosetta --agree-to-license

# Ensure Flutter supports ARM64
flutter config --enable-macos-desktop
```

**Linux Wayland:**
```bash
# For Wayland support
export GDK_BACKEND=wayland
flutter run -d linux
```

**Windows PowerShell:**
```powershell
# Set execution policy for scripts
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

## Development Workflow

### 1. Daily Development
```bash
# Pull latest changes
git pull origin main

# Update dependencies
cd rust && cargo update
cd ../flutter && flutter pub upgrade

# Run tests
cargo test && flutter test
```

### 2. Code Quality
```bash
# Rust formatting
cd rust && cargo fmt
cd rust && cargo clippy

# Flutter analysis
cd flutter && flutter analyze
```

### 3. Before Commit
```bash
# Full test suite
cargo test && flutter test

# Code formatting
cargo fmt && flutter format .

# Static analysis
cargo clippy && flutter analyze
```

## Performance Tips

### Rust Development
```bash
# Use faster linker
# Add to ~/.cargo/config.toml:
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### Flutter Development
```bash
# Enable Hot Reload/Restart
# Use --release flag for performance testing
flutter run --profile release

# Track performance
flutter run --profile release --trace-startup
```

## Next Steps

After setup:
1. Review [Security Audit](docs/security-audit.md)
2. Check [Code Quality Guide](docs/code-quality.md)
3. Run initial build to verify everything works
4. Explore the [Development Status](README.md#development-status)

## Support

**Environment Issues:**
- Check Flutter Doctor: `flutter doctor -v`
- Verify Rust installation: `rustc --version && cargo --version`
- Platform-specific issues in Troubleshooting section

**Code Issues:**
- Review [Refactoring Summary](refactoring-summary.md) for recent improvements
- Check existing GitHub issues
- Create new issue with environment details

---

**Ready to start development! 🚀**