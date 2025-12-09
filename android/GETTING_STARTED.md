# Getting Started with PredictiveRolls Android

This guide will walk you through building and running the PredictiveRolls Android app from scratch.

## Prerequisites Installation

### 1. Install Android Studio

Download and install Android Studio from: https://developer.android.com/studio

During installation, make sure to install:
- Android SDK
- Android SDK Platform
- Android Virtual Device (for emulator)

### 2. Install Android NDK

1. Open Android Studio
2. Go to **Tools** → **SDK Manager**
3. Click the **SDK Tools** tab
4. Check **NDK (Side by side)**
5. Click **Apply** and wait for installation

Note the NDK location (usually `~/Android/Sdk/ndk/<version>` on Linux/macOS or `C:\Users\<username>\AppData\Local\Android\Sdk\ndk\<version>` on Windows)

### 3. Install Rust

If you haven't already:

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download and run from: https://rustup.rs/

### 4. Install cargo-ndk

```bash
cargo install cargo-ndk
```

### 5. Add Android Rust Targets

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

## Environment Setup

### Linux/macOS

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/<version>
export PATH=$PATH:$ANDROID_HOME/platform-tools
export PATH=$PATH:$ANDROID_HOME/tools
```

Replace `<version>` with your actual NDK version (e.g., `25.2.9519653`).

Then reload:
```bash
source ~/.bashrc  # or ~/.zshrc
```

### Windows (PowerShell)

Add to your PowerShell profile:

```powershell
$env:ANDROID_HOME = "C:\Users\<username>\AppData\Local\Android\Sdk"
$env:ANDROID_NDK_HOME = "$env:ANDROID_HOME\ndk\<version>"
$env:PATH += ";$env:ANDROID_HOME\platform-tools;$env:ANDROID_HOME\tools"
```

Or set permanently via System Environment Variables.

## Building the App

### Method 1: Quick Build (Recommended)

1. Clone the repository:
   ```bash
   git clone https://github.com/sushiomsky/PredictiveRolls.git
   cd PredictiveRolls
   ```

2. Build everything:
   ```bash
   cd android
   ./build.sh
   ```

3. The script will:
   - Verify dependencies
   - Build Rust libraries for all Android architectures
   - Copy libraries to the correct locations

4. Build the APK:
   ```bash
   ./gradlew assembleDebug
   ```

5. Install on device/emulator:
   ```bash
   adb install app/build/outputs/apk/debug/app-debug.apk
   ```

### Method 2: Step-by-Step Build

1. Navigate to the android-lib directory:
   ```bash
   cd android-lib
   ```

2. Build for arm64-v8a (most common for modern phones):
   ```bash
   cargo ndk -t arm64-v8a -o ../android/app/src/main/jniLibs build --release
   ```

3. (Optional) Build for other architectures:
   ```bash
   cargo ndk -t armeabi-v7a -o ../android/app/src/main/jniLibs build --release
   cargo ndk -t x86 -o ../android/app/src/main/jniLibs build --release
   cargo ndk -t x86_64 -o ../android/app/src/main/jniLibs build --release
   ```

4. Build the Android app:
   ```bash
   cd ../android
   ./gradlew assembleDebug
   ```

### Method 3: Using Android Studio

1. First build the Rust libraries using Method 1 or 2

2. Open Android Studio

3. Click **File** → **Open**

4. Navigate to and select the `android` directory

5. Wait for Gradle sync to complete

6. Click **Build** → **Build Bundle(s) / APK(s)** → **Build APK(s)**

7. Once built, click **locate** in the notification to find the APK

## Running the App

### On a Physical Device

1. Enable Developer Options on your Android device:
   - Go to **Settings** → **About Phone**
   - Tap **Build Number** 7 times

2. Enable USB Debugging:
   - Go to **Settings** → **Developer Options**
   - Enable **USB Debugging**

3. Connect your device via USB

4. Verify connection:
   ```bash
   adb devices
   ```

5. Install the APK:
   ```bash
   adb install app/build/outputs/apk/debug/app-debug.apk
   ```

6. Launch the app from your device's app drawer

### On an Emulator

1. Open Android Studio

2. Click **Tools** → **Device Manager**

3. Click **Create Device**

4. Select a device (e.g., Pixel 5)

5. Select a system image (API 24 or higher)

6. Finish and start the emulator

7. Install the APK:
   ```bash
   adb install app/build/outputs/apk/debug/app-debug.apk
   ```

## First Run Configuration

1. Launch the PredictiveRolls app

2. Read and accept the disclaimer

3. Tap the **Settings** button

4. Configure your settings:
   - **Select Site**: Choose your gambling site
   - **API Key**: Enter your API key from the gambling site
   - **Currency**: Enter the currency code (e.g., BTC)
   - **Strategy**: Select your betting strategy

5. Tap **Save Settings**

6. Return to the main screen

7. Tap **Start Betting** to begin

## Troubleshooting

### "ANDROID_NDK_HOME is not set"

**Solution:**
```bash
# Find your NDK version
ls ~/Android/Sdk/ndk/  # Linux/macOS
dir %LOCALAPPDATA%\Android\Sdk\ndk  # Windows

# Set the environment variable
export ANDROID_NDK_HOME=~/Android/Sdk/ndk/<version>  # Linux/macOS
```

### "cargo-ndk: command not found"

**Solution:**
```bash
cargo install cargo-ndk
```

### "error: linker `aarch64-linux-android-clang` not found"

**Solution:** Make sure `ANDROID_NDK_HOME` is set correctly and the NDK is installed.

### "No connected devices"

**Solution:**
- For physical device: Enable USB debugging and reconnect
- For emulator: Start an emulator from Android Studio

### Build fails with "Could not find gradle wrapper"

**Solution:**
```bash
cd android
chmod +x gradlew
./gradlew wrapper
```

### "Unable to load native library"

**Solution:** The Rust libraries weren't built. Run:
```bash
cd android
./build.sh
```

## Next Steps

- Check out the [Android README](README.md) for detailed documentation
- Read about [security considerations](README.md#security)
- Learn about [betting strategies](../README.md#available-strategies)
- Explore the source code in `android/app/src/main/java/com/predictiverolls/`

## Getting Help

If you encounter issues:

1. Check this guide thoroughly
2. Review the [main README](../README.md)
3. Search existing [GitHub issues](https://github.com/sushiomsky/PredictiveRolls/issues)
4. Open a new issue with:
   - Your OS and version
   - Android Studio version
   - NDK version
   - Rust version
   - Complete error messages
   - Steps to reproduce

## Development Tips

- Use `./gradlew clean` to clean build artifacts
- Use `adb logcat` to view Android logs
- Filter logs: `adb logcat | grep PredictiveRolls`
- Use Android Studio's debugger for Java code
- Add logging in Rust code with `log::info!()`, `log::debug!()`, etc.

Happy betting! (But remember: only for educational purposes! 🎓)
