# PredictiveRolls Android

Android version of the PredictiveRolls machine learning-based predictive dice rolling application.

## ⚠️ Disclaimer

This software is for educational and research purposes only. Gambling involves risk, and you should never gamble with money you cannot afford to lose. The developers are not responsible for any financial losses incurred while using this software.

## Prerequisites

To build and run the Android version, you need:

### Android Development
- Android Studio (Arctic Fox or later)
- Android SDK (API level 24 or higher)
- Android NDK (r25 or later)
- Gradle 8.2 or later

### Rust Development
- Rust 1.70 or later
- cargo-ndk for building Rust libraries for Android:
  ```bash
  cargo install cargo-ndk
  ```
- Android targets for Rust:
  ```bash
  rustup target add aarch64-linux-android
  rustup target add armv7-linux-androideabi
  rustup target add i686-linux-android
  rustup target add x86_64-linux-android
  ```

## Environment Setup

1. Set the `ANDROID_NDK_HOME` environment variable:
   ```bash
   export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/<version>
   ```

   Or on Windows:
   ```cmd
   set ANDROID_NDK_HOME=C:\Users\<username>\AppData\Local\Android\Sdk\ndk\<version>
   ```

2. Verify NDK installation:
   ```bash
   echo $ANDROID_NDK_HOME
   ```

## Building

### Method 1: Using the Build Script (Linux/macOS)

1. Navigate to the android directory:
   ```bash
   cd android
   ```

2. Run the build script:
   ```bash
   ./build.sh
   ```

3. Build the Android APK:
   ```bash
   ./gradlew assembleDebug
   ```

### Method 2: Manual Build

1. Build Rust native libraries:
   ```bash
   cd android-lib
   
   # Build for each architecture
   cargo ndk -t arm64-v8a -o ../android/app/src/main/jniLibs build --release
   cargo ndk -t armeabi-v7a -o ../android/app/src/main/jniLibs build --release
   cargo ndk -t x86 -o ../android/app/src/main/jniLibs build --release
   cargo ndk -t x86_64 -o ../android/app/src/main/jniLibs build --release
   ```

2. Build Android APK:
   ```bash
   cd android
   ./gradlew assembleDebug
   ```

### Method 3: Using Android Studio

1. Open the `android` directory in Android Studio
2. Build the Rust libraries first using Method 1 or 2
3. Click "Build" → "Build Bundle(s) / APK(s)" → "Build APK(s)"

## Installation

### Debug APK
After building, the APK will be located at:
```
android/app/build/outputs/apk/debug/app-debug.apk
```

Install on a connected device or emulator:
```bash
adb install app/build/outputs/apk/debug/app-debug.apk
```

### Release Build

For production release:
1. Generate a signing key
2. Configure signing in `app/build.gradle`
3. Build release APK:
   ```bash
   ./gradlew assembleRelease
   ```

## Configuration

1. Launch the app on your Android device
2. Accept the disclaimer
3. Tap "Settings" button
4. Configure:
   - **Gambling Site**: Choose from DuckDice, CryptoGames, or FreeBitco.in
   - **API Key**: Enter your site API key (stored securely)
   - **Currency**: Enter currency code (e.g., BTC, ETH)
   - **Strategy**: Select betting strategy

5. Tap "Save Settings"

## Usage

1. After configuration, return to the main screen
2. Tap "Start Betting" to begin
3. The app will display:
   - Current prediction
   - Confidence level
   - Current balance
   - Win rate
   - Betting log

4. Tap "Stop Betting" to pause the session

## Project Structure

```
android/
├── app/
│   ├── src/main/
│   │   ├── java/com/predictiverolls/    # Java source code
│   │   │   ├── MainActivity.java         # Main app activity
│   │   │   ├── SettingsActivity.java    # Settings screen
│   │   │   └── PredictiveRollsNative.java # JNI interface
│   │   ├── res/                          # Android resources
│   │   │   ├── layout/                   # UI layouts
│   │   │   ├── values/                   # Strings, colors
│   │   │   └── drawable/                 # Icons
│   │   ├── jniLibs/                      # Native libraries (generated)
│   │   └── AndroidManifest.xml           # App manifest
│   ├── build.gradle                      # App build configuration
│   └── proguard-rules.pro               # ProGuard rules
├── build.gradle                          # Project build configuration
├── settings.gradle                       # Project settings
├── gradle.properties                     # Gradle properties
├── build.sh                             # Build script
└── README.md                            # This file

android-lib/
├── src/
│   └── lib.rs                           # Rust JNI implementation
└── Cargo.toml                           # Rust dependencies
```

## Features

- 🤖 **Native Rust Core**: Uses JNI for high-performance ML inference
- 🔒 **Secure Storage**: API keys stored using Android EncryptedSharedPreferences
- 📱 **Material Design**: Modern Android UI with Material Components
- 🎯 **Multiple Sites**: Support for DuckDice, CryptoGames, FreeBitco.in
- 📊 **Real-time Stats**: Live display of predictions, confidence, balance, and win rate
- 🛡️ **Network Security**: HTTPS-only communication with gambling sites

## Security

- **API keys** are stored using Android's EncryptedSharedPreferences
- The app uses **HTTPS only** for network communication
- **No cleartext traffic** is allowed (configured in AndroidManifest.xml)
- Native libraries are **stripped** in release builds to prevent reverse engineering

## Troubleshooting

### Build Errors

**Error: "ANDROID_NDK_HOME not set"**
- Solution: Install Android NDK and set the environment variable

**Error: "cargo-ndk not found"**
- Solution: Install cargo-ndk:
  ```bash
  cargo install cargo-ndk
  ```

**Error: "target not found"**
- Solution: Add the missing Android target:
  ```bash
  rustup target add aarch64-linux-android
  ```

### Runtime Errors

**Error: "Unable to load native library"**
- Solution: Ensure native libraries were built and copied to jniLibs/

**Error: "API key not configured"**
- Solution: Go to Settings and configure your API key

## Performance

The Android version uses:
- **Native Rust code** for ML inference (via JNI)
- **Optimized release builds** with LTO and size optimization
- **Background threads** for betting operations
- **Minimal UI updates** to conserve battery

## Known Limitations

1. **ML Model**: The current implementation uses a simplified prediction model for demonstration. For production use, integrate the full Burn-based neural network model.

2. **GPU Acceleration**: Mobile GPUs differ from desktop. The Vulkan backend may need adjustments for optimal Android performance.

3. **Background Execution**: Android limits background execution. Long-running betting sessions may be interrupted.

## Future Enhancements

- [ ] Full ML model integration with Burn framework
- [ ] Optimized mobile GPU inference
- [ ] Background service for continuous betting
- [ ] Notification support
- [ ] Historical data and charts
- [ ] Multiple account management
- [ ] Backup and restore settings

## Contributing

Please read [CONTRIBUTING.md](../CONTRIBUTING.md) for details on contributing to the Android version.

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Support

For Android-specific issues:
1. Check this README
2. Review existing GitHub issues
3. Open a new issue with the "android" label
