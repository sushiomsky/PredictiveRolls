# CI/CD Setup Guide for PredictiveRolls Android

This guide explains how to set up the CI/CD pipeline for building, testing, and releasing the PredictiveRolls Android application.

## Overview

The CI/CD pipeline consists of:
1. **Rust Native Library Building** - Cross-compiles Rust code for all Android architectures
2. **Android Debug Build** - Creates debug APK for testing
3. **Android Release Build** - Creates signed, optimized release APK
4. **Testing** - Runs unit and instrumentation tests
5. **Linting** - Performs code quality checks

## Prerequisites

- GitHub repository with Actions enabled
- Android SDK and NDK (handled automatically in CI)
- Rust toolchain (handled automatically in CI)
- Signing keystore for release builds

## Generating a Release Keystore

### Step 1: Generate the Keystore

Use the Java keytool to generate a keystore:

```bash
keytool -genkey -v -keystore release-keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias predictive-rolls
```

You'll be prompted to enter:
- **Keystore password**: Choose a strong password
- **Key password**: Can be the same as keystore password
- **Name and organization details**: Fill these appropriately

### Step 2: Encode Keystore to Base64

For GitHub Secrets, encode the keystore file:

```bash
base64 release-keystore.jks > keystore-base64.txt
```

### Step 3: Configure GitHub Secrets

Go to your GitHub repository → Settings → Secrets and variables → Actions, and add:

1. **KEYSTORE_BASE64**: Content of `keystore-base64.txt`
2. **KEYSTORE_PASSWORD**: The keystore password you chose
3. **KEY_ALIAS**: `predictive-rolls` (or the alias you used)
4. **KEY_PASSWORD**: The key password you chose

## Local Development Setup

### Building Locally

#### Prerequisites for Local Builds

1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Android targets:
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

3. Install cargo-ndk:
```bash
cargo install cargo-ndk
```

4. Set up Android NDK:
```bash
# Install via Android Studio SDK Manager or
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/<version>
```

#### Build Native Libraries

```bash
cd android-lib

# Build for all architectures
cargo ndk -t arm64-v8a -o ../android/app/src/main/jniLibs build --release
cargo ndk -t armeabi-v7a -o ../android/app/src/main/jniLibs build --release
cargo ndk -t x86 -o ../android/app/src/main/jniLibs build --release
cargo ndk -t x86_64 -o ../android/app/src/main/jniLibs build --release

# Or use the build script
cd ..
./android/build.sh
```

#### Build Android APK

Debug build:
```bash
cd android
./gradlew assembleDebug
```

Release build (requires signing configuration):
```bash
cd android
./gradlew assembleRelease
```

The APK will be in `android/app/build/outputs/apk/`

### Local Signing Configuration

Create a file `android/local.properties` (this file is gitignored):

```properties
KEYSTORE_FILE=release-keystore.jks
KEYSTORE_PASSWORD=your_keystore_password
KEY_ALIAS=predictive-rolls
KEY_PASSWORD=your_key_password
```

Place your `release-keystore.jks` in the `android/app/` directory.

## CI/CD Workflow

### Triggered On

- Push to `main` or `develop` branches
- Pull requests to `main` or `develop`
- Only when changes are made to:
  - `android/**`
  - `android-lib/**`
  - `.github/workflows/android.yml`

### Build Artifacts

#### Debug Builds
- Available for every PR and push
- Unsigned, debuggable APK
- Retained for 30 days
- Download from Actions → Workflow Run → Artifacts

#### Release Builds
- Only built on pushes to `main` or `develop`
- Signed and optimized APK (if keystore is configured)
- Code obfuscation with ProGuard/R8
- Resource shrinking enabled
- Retained for 90 days

#### Lint Reports
- Generated for every build
- Available as artifact: `lint-results`
- HTML report with detailed findings

#### Test Results
- Unit test reports
- Available as artifact: `test-results`

### Automatic Releases

When pushing to the `main` branch:
1. A GitHub Release is automatically created
2. Release is tagged as `v<run_number>`
3. Release APK is attached to the release
4. Can be downloaded by end users

## Build Variants

### Debug
- **Application ID**: `com.predictiverolls.debug`
- **Debuggable**: Yes
- **Minification**: No
- **Use Case**: Development and testing

### Release
- **Application ID**: `com.predictiverolls`
- **Debuggable**: No
- **Minification**: Yes (ProGuard/R8)
- **Resource Shrinking**: Yes
- **Use Case**: Production distribution

## Optimization

### APK Size Optimization

The release build includes:
- ProGuard/R8 code shrinking
- Resource shrinking
- Removal of unused code
- Debug log removal
- Native library stripping (configured in Rust)

### Build Performance

The CI uses caching for:
- Gradle dependencies
- Cargo registry and build artifacts
- Native libraries (across jobs)

## Troubleshooting

### Build Fails: "NDK not found"

The CI automatically sets up NDK. For local builds:
```bash
# Install via Android Studio SDK Manager
# Or set ANDROID_NDK_HOME environment variable
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/<version>
```

### Build Fails: "cargo-ndk not found"

Install cargo-ndk:
```bash
cargo install cargo-ndk
```

### Build Fails: "target not found"

Add missing Android targets:
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### Release Build Not Signed

Ensure GitHub Secrets are correctly configured:
1. Verify `KEYSTORE_BASE64` is the base64-encoded keystore
2. Verify passwords and alias match the keystore
3. Check workflow logs for signing errors

### Native Library Missing at Runtime

Ensure native libraries are in the correct directories:
```
android/app/src/main/jniLibs/
├── arm64-v8a/
│   └── libpredictive_rolls_android.so
├── armeabi-v7a/
│   └── libpredictive_rolls_android.so
├── x86/
│   └── libpredictive_rolls_android.so
└── x86_64/
    └── libpredictive_rolls_android.so
```

### ProGuard Issues

If the release build crashes:
1. Check ProGuard rules in `proguard-rules.pro`
2. Add keep rules for classes that are accessed via reflection
3. Use mapping file to deobfuscate stack traces: `android/app/build/outputs/mapping/release/mapping.txt`

## Security Best Practices

1. **Never commit the keystore file** - It's in `.gitignore`
2. **Never commit passwords** - Use GitHub Secrets or local.properties
3. **Rotate keys periodically** - Especially if compromised
4. **Use different keystores** - Separate for development and production
5. **Backup your keystore** - Store securely, you can't republish without it

## Monitoring and Maintenance

### Regular Tasks

1. **Update dependencies** monthly:
   ```bash
   cd android
   ./gradlew dependencies --write-locks
   ```

2. **Check for security vulnerabilities**:
   ```bash
   ./gradlew dependencyCheckAnalyze
   ```

3. **Review lint warnings** after each build

4. **Monitor APK size** - Target < 20MB

5. **Test on multiple Android versions** - API 24 to latest

## Additional Resources

- [Android App Signing](https://developer.android.com/studio/publish/app-signing)
- [ProGuard Rules](https://www.guardsquare.com/manual/configuration/usage)
- [GitHub Actions for Android](https://github.com/marketplace?type=actions&query=android)
- [Rust NDK Build](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)

## Support

For issues with the CI/CD pipeline:
1. Check the [workflow file](.github/workflows/android.yml)
2. Review [build logs](https://github.com/sushiomsky/PredictiveRolls/actions)
3. Open an issue on GitHub with relevant logs
