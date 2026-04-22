#!/bin/bash
set -e

echo "Building AsterIDE for macOS, please wait."

if ! command -v cargo-bundle &> /dev/null; then
    echo "cargo-bundle not found. Installing..."
    cargo install cargo-bundle
fi

echo "Building app bundle."
cd crates/editor
cargo bundle --release
cd ../..

if [ ! -d "target/release/bundle/osx/AsterIDE.app" ]; then
    echo "Error: App bundle not found at target/release/bundle/osx/AsterIDE.app"
    exit 1
fi

echo "Creating build directory."
mkdir -p build

echo "Preparing DMG contents."
BUILD_DIR="build/AsterIDE"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

cp -r "target/release/bundle/osx/AsterIDE.app" "$BUILD_DIR/"

ln -s /Applications "$BUILD_DIR/Applications"

echo "Creating Disk Mounter Image."
hdiutil create -volname "AsterIDE" -srcfolder "$BUILD_DIR" -ov -format UDZO "build/AsterIDE.dmg"

echo "Done. DMG created at build/AsterIDE.dmg"
