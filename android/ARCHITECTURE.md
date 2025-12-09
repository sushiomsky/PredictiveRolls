# PredictiveRolls Android Architecture

This document describes the architecture and design decisions for the Android version of PredictiveRolls.

## Overview

The Android version is a native Android app that leverages the existing Rust codebase through JNI (Java Native Interface). This hybrid approach combines:
- **Java/Android UI**: Native Android user interface using Material Design
- **Rust Core**: High-performance ML inference and betting logic
- **JNI Bridge**: Seamless communication between Java and Rust

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Android Application                       │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────────┐         ┌─────────────────┐           │
│  │  MainActivity   │         │ SettingsActivity│           │
│  │                 │         │                 │           │
│  │  - UI Updates   │◄───────►│  - Config UI    │           │
│  │  - Betting Loop │         │  - Secure Store │           │
│  │  - Stats Display│         │                 │           │
│  └────────┬────────┘         └─────────────────┘           │
│           │                                                  │
│           │ JNI Calls                                       │
│           ▼                                                  │
│  ┌──────────────────────────────────────┐                  │
│  │   PredictiveRollsNative (Java)       │                  │
│  │                                       │                  │
│  │  - native initialize()                │                  │
│  │  - native configure()                 │                  │
│  │  - native getPrediction()             │                  │
│  │  - native placeBet()                  │                  │
│  └──────────────────┬────────────────────┘                  │
│                     │                                        │
└─────────────────────┼────────────────────────────────────────┘
                      │ JNI Boundary
┌─────────────────────▼────────────────────────────────────────┐
│              Rust Native Library (JNI)                        │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────┐                   │
│  │   android-lib/src/lib.rs              │                   │
│  │                                       │                   │
│  │  - JNI Entry Points                   │                   │
│  │  - State Management                   │                   │
│  │  - Android Logger                     │                   │
│  └──────────────┬────────────────────────┘                   │
│                 │                                             │
│                 ▼                                             │
│  ┌──────────────────────────────────────┐                   │
│  │   Main Crate (Future Integration)     │                   │
│  │                                       │                   │
│  │  - ML Model (Burn)                    │                   │
│  │  - Site APIs (DuckDice, etc.)        │                   │
│  │  - Betting Strategies                 │                   │
│  └──────────────────────────────────────┘                   │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Components

### 1. Android UI Layer (Java)

#### MainActivity.java
**Responsibilities:**
- Display real-time predictions, confidence, balance, and win rate
- Start/stop betting sessions
- Update UI from background threads using Handler
- Manage app lifecycle
- Display betting log with timestamps

**Key Features:**
- Background thread execution with ExecutorService
- Thread-safe UI updates via Handler.post()
- Encrypted SharedPreferences for secure storage
- Disclaimer dialog on first launch

#### SettingsActivity.java
**Responsibilities:**
- Configure gambling site selection
- Manage API key input
- Select currency and betting strategy
- Persist settings securely

**Key Features:**
- Encrypted storage for sensitive data
- Spinner controls for site and strategy selection
- Input validation
- Parent activity integration

#### PredictiveRollsNative.java
**Responsibilities:**
- Define JNI interface methods
- Provide Java API for native functionality
- Document native method contracts

**Design Pattern:**
- Static native methods (no object instantiation needed)
- Simple primitive types for easy JNI marshalling

### 2. Rust Native Layer (JNI)

#### android-lib/src/lib.rs
**Responsibilities:**
- Implement JNI entry points
- Manage application state (thread-safe with Mutex)
- Provide Android-specific logging
- Bridge between Java and core Rust functionality

**Key Features:**
- Global state management with lazy_static
- Android logger integration
- JNI type conversions (jstring, jfloat, jboolean)
- Error handling for JNI calls

**State Management:**
```rust
struct AppState {
    initialized: bool,
    site: String,
    api_key: String,
    currency: String,
    strategy: String,
    prediction: f32,
    confidence: f32,
    balance: f64,
    total_bets: u32,
    wins: u32,
}
```

### 3. Build System

#### Build Script (build.sh)
**Purpose:** Automate cross-compilation for all Android architectures

**Process:**
1. Verify dependencies (cargo, rustup, NDK)
2. Add Android targets if missing
3. Build for each architecture:
   - arm64-v8a (64-bit ARM)
   - armeabi-v7a (32-bit ARM)
   - x86 (32-bit x86)
   - x86_64 (64-bit x86)
4. Copy libraries to jniLibs directory

#### Gradle Build System
**Configuration:**
- App-level: Dependencies, SDK versions, build types
- Project-level: Plugin versions, repositories
- Gradle Wrapper: Version 8.2 for reproducible builds

## Data Flow

### Betting Flow
```
User taps "Start Betting"
    ↓
MainActivity.startBetting()
    ↓
Configure native library via JNI
    ↓
Background thread loop:
    1. getPrediction() → JNI → Rust
    2. getConfidence() → JNI → Rust
    3. placeBet() → JNI → Rust → (Future: API call)
    4. getBalance() → JNI → Rust
    5. getWinRate() → JNI → Rust
    6. Update UI via Handler.post()
    7. Sleep 5 seconds
    8. Repeat if still running
```

### Configuration Flow
```
User opens Settings
    ↓
Load from EncryptedSharedPreferences
    ↓
Display in UI controls
    ↓
User modifies settings
    ↓
Tap "Save Settings"
    ↓
Validate input
    ↓
Save to EncryptedSharedPreferences
    ↓
Return to MainActivity
```

## Security Considerations

### 1. Secure Storage
- **EncryptedSharedPreferences**: API keys encrypted at rest
- **AES-256-GCM encryption**: Industry-standard encryption
- **Fallback**: Regular SharedPreferences if encryption fails (logs warning)

### 2. Network Security
- **HTTPS Only**: `android:usesCleartextTraffic="false"`
- **Certificate Pinning**: Can be added in future for specific sites
- **Network State Permission**: Check connectivity before API calls

### 3. Code Security
- **ProGuard**: Obfuscates Java code in release builds
- **Native Library Stripping**: Removes debug symbols
- **No Hardcoded Secrets**: All credentials from user input

### 4. App Security
- **No Backup Flag**: Consider adding `android:allowBackup="false"`
- **Screen Security**: Consider FLAG_SECURE for screenshot prevention
- **Root Detection**: Consider adding for production

## Threading Model

### Main Thread (UI Thread)
- View updates
- User interactions
- Handler message processing

### Background Thread (ExecutorService)
- Betting loop
- Native library calls
- API communication (future)
- Heavy computations

### Communication
- Handler.post() for Main ← Background
- Thread-safe state in Rust (Mutex)
- No blocking operations on main thread

## Future Enhancements

### Phase 1: Core Integration
- [ ] Integrate full Burn ML model
- [ ] Connect to actual gambling site APIs
- [ ] Implement all betting strategies
- [ ] Add proper error handling and recovery

### Phase 2: Features
- [ ] Background Service for continuous betting
- [ ] Notification system for wins/losses
- [ ] Historical data charts
- [ ] Multiple account management
- [ ] Export betting history

### Phase 3: Performance
- [ ] Optimize ML inference for mobile GPUs
- [ ] Reduce battery consumption
- [ ] Implement caching strategies
- [ ] Add offline mode support

### Phase 4: User Experience
- [ ] Dark theme support
- [ ] Tablet-optimized UI
- [ ] Widget for home screen
- [ ] Share functionality
- [ ] Localization (multiple languages)

## Testing Strategy

### Unit Tests (Future)
- Test JNI method contracts
- Test state management
- Test UI logic
- Test encryption/decryption

### Integration Tests (Future)
- Test MainActivity → Native flow
- Test SettingsActivity → Storage flow
- Test thread synchronization
- Test error recovery

### Manual Testing
- Test on various Android versions (API 24+)
- Test on different screen sizes
- Test with different network conditions
- Test background/foreground transitions

## Performance Considerations

### Memory
- Rust native library: ~5-10 MB
- Java code: ~2-3 MB
- Total APK size: ~15-20 MB (depends on architectures included)

### CPU
- ML inference: Moderate (depends on model complexity)
- UI updates: Minimal
- JNI overhead: Negligible for our use case

### Battery
- Active betting: High (network + computation)
- Idle: Low (no background services yet)
- Optimization: Use WorkManager for background tasks

### Network
- API calls: 1 per bet (~5 seconds interval)
- Data usage: Low (small JSON payloads)
- Connection pooling: Handled by Rust reqwest

## Debugging Guide

### Java Debugging
```bash
# View all logs
adb logcat

# Filter by app
adb logcat | grep PredictiveRolls

# Clear logs
adb logcat -c

# Save logs to file
adb logcat > logs.txt
```

### Native Debugging
```bash
# View native logs (from android_logger)
adb logcat | grep PredictiveRolls

# Use ndk-stack for crashes
adb logcat | ndk-stack -sym android-lib/target/aarch64-linux-android/release
```

### Common Issues
1. **Library not loaded**: Check jniLibs contains .so files
2. **Method not found**: Verify JNI naming convention
3. **Null pointer**: Check thread synchronization
4. **ClassNotFound**: Verify ProGuard rules

## Code Metrics

- Total lines of code: ~1,046
- Java code: ~450 lines
- Rust code: ~200 lines
- XML resources: ~300 lines
- Build scripts: ~100 lines
- Documentation: ~15,000 words

## Dependencies

### Android Dependencies
- androidx.appcompat:appcompat:1.6.1
- com.google.android.material:material:1.11.0
- androidx.constraintlayout:constraintlayout:2.1.4
- androidx.security:security-crypto:1.1.0-alpha06

### Rust Dependencies
- jni:0.21 (JNI bindings)
- log:0.4 (logging facade)
- android_logger:0.13 (Android logging)
- lazy_static:1.5.0 (static initialization)
- rand:0.9 (random number generation)

## License

This Android implementation follows the same MIT License as the main project.
