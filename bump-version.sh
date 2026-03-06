#!/bin/bash
# BullShift Version Bump Script
# Usage: ./bump-version.sh <new_version>
# Example: ./bump-version.sh 2026.3.5

set -e

if [ -z "$1" ]; then
    echo "Current version: $(cat VERSION)"
    echo "Usage: $0 <new_version>"
    echo "Example: $0 2026.3.5"
    exit 1
fi

NEW_VERSION="$1"
OLD_VERSION=$(cat VERSION | tr -d '[:space:]')

echo "Bumping version: $OLD_VERSION -> $NEW_VERSION"

# Update VERSION file
echo "$NEW_VERSION" > VERSION

# Update Cargo.toml
sed -i "s/^version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" rust/Cargo.toml

# Update pubspec.yaml
sed -i "s/^version: $OLD_VERSION/version: $NEW_VERSION/" flutter/pubspec.yaml

# Update README badge
sed -i "s/Version-${OLD_VERSION}/Version-${NEW_VERSION}/" README.md

echo "Updated files:"
echo "  VERSION"
echo "  rust/Cargo.toml"
echo "  flutter/pubspec.yaml"
echo "  README.md"
echo ""
echo "Version bumped to $NEW_VERSION"
echo "Don't forget to update CHANGELOG.md!"
