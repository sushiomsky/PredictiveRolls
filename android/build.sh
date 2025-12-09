#!/bin/bash

# Build script for Android version of PredictiveRolls
# This script builds the Rust native library for all Android architectures
# and copies them to the correct locations in the Android project

set -e

echo "Building PredictiveRolls for Android..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found. Please install Rust."
    exit 1
fi

# Check if rustup is installed
if ! command -v rustup &> /dev/null; then
    echo "Error: rustup not found. Please install Rust."
    exit 1
fi

# Add Android targets if not already added
echo "Adding Android targets..."
rustup target add aarch64-linux-android || true
rustup target add armv7-linux-androideabi || true
rustup target add i686-linux-android || true
rustup target add x86_64-linux-android || true

# Set up NDK path
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME environment variable not set."
    echo "Please install Android NDK and set ANDROID_NDK_HOME."
    exit 1
fi

echo "Using Android NDK: $ANDROID_NDK_HOME"

# Navigate to android-lib directory
cd "$(dirname "$0")/../android-lib"

# Build for each architecture
echo "Building for arm64-v8a..."
cargo ndk -t arm64-v8a -o ../android/app/src/main/jniLibs build --release

echo "Building for armeabi-v7a..."
cargo ndk -t armeabi-v7a -o ../android/app/src/main/jniLibs build --release

echo "Building for x86..."
cargo ndk -t x86 -o ../android/app/src/main/jniLibs build --release

echo "Building for x86_64..."
cargo ndk -t x86_64 -o ../android/app/src/main/jniLibs build --release

echo ""
echo "✓ Build complete!"
echo ""
echo "Native libraries have been copied to android/app/src/main/jniLibs/"
echo ""
echo "To build the Android APK:"
echo "  cd android"
echo "  ./gradlew assembleDebug"
echo ""
