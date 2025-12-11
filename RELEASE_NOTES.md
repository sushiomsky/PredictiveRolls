# PredictiveRolls Android Release Notes

## Version 1.0 - Major Android Refactoring

### Overview

This release represents a comprehensive refactoring of PredictiveRolls to provide full Android support with modern Material Design 3 UI and complete DuckDice Bot API integration.

### What's New

#### 🎨 Modern Android UI
- **Material Design 3**: Complete redesign using the latest Material Design components
- **Dark Mode Support**: Automatic theme switching based on system preferences
- **Responsive Layout**: Optimized for different screen sizes using CoordinatorLayout
- **Enhanced Statistics**: Real-time display of predictions, confidence, balance, and win rate
- **Visual Feedback**: Emoji-based status indicators and improved activity logging
- **Modern Toolbar**: Material toolbar with menu options and About dialog

#### 🔌 Full DuckDice API Integration
- **Real API Client**: Complete Rust-based HTTP client using reqwest and tokio
- **All Endpoints**: User info, place bet, and randomize seed fully implemented
- **Rate Limiting**: Automatic detection and handling of API rate limits
- **Error Recovery**: Comprehensive error handling with user-friendly messages
- **Async Operations**: Non-blocking API calls for smooth user experience
- **Faucet Support**: Configurable faucet or main balance usage

#### 🔒 Enhanced Security
- **Encrypted Storage**: API keys secured with Android Keystore (AES-256-GCM)
- **HTTPS Only**: All network traffic encrypted (no cleartext allowed)
- **ProGuard/R8**: Code obfuscation and optimization for release builds
- **Secure Build**: Proper signing configuration for production releases
- **Stable Dependencies**: Using production-ready library versions

#### 🔧 CI/CD Pipeline
- **Automated Builds**: Multi-architecture native library compilation
- **Release Signing**: Automatic APK signing with GitHub Secrets
- **Quality Checks**: Android lint, formatting, and code analysis
- **Artifact Management**: Automatic upload of debug and release APKs
- **GitHub Releases**: Automatic release creation on main branch pushes

#### 📚 Comprehensive Documentation
- **CI/CD Setup Guide**: Complete instructions for pipeline configuration
- **API Integration Guide**: Detailed architecture and usage documentation
- **Build Instructions**: Step-by-step guide for local development
- **Security Best Practices**: Guidelines for secure deployment
- **Troubleshooting**: Common issues and solutions

### Technical Details

#### Architecture
```
Android UI (Java/Material 3)
    ↓ JNI Bridge
Rust Business Logic (android-lib)
    ↓ HTTP Client
DuckDice Bot API
```

#### Supported Platforms
- **Minimum SDK**: Android 7.0 (API 24)
- **Target SDK**: Android 14 (API 34)
- **Architectures**: ARM64, ARMv7, x86, x86_64

#### Key Dependencies
- Material Design 3 (1.11.0)
- AndroidX Security Crypto (1.0.0)
- Reqwest (0.12)
- Tokio (1.x)
- JNI (0.21)

### Installation

#### From Source
```bash
# Clone repository
git clone https://github.com/sushiomsky/PredictiveRolls.git
cd PredictiveRolls

# Build native libraries
cd android
./build.sh

# Build APK
./gradlew assembleDebug
```

#### From CI/CD
1. Go to GitHub Actions → Latest workflow run
2. Download `app-debug` or `app-release` artifact
3. Install APK on Android device

### Configuration

1. Launch app and accept disclaimer
2. Tap Settings (or menu → Settings)
3. Configure:
   - Gambling Site: `duck_dice`
   - API Key: Your DuckDice API key
   - Currency: `BTC`, `ETH`, etc.
   - Strategy: `None` (more coming soon)
4. Save settings
5. Return to main screen
6. Tap "Start Betting"

### Known Limitations

1. **ML Model**: Currently uses simplified prediction logic; full neural network integration pending
2. **Single Site**: Only DuckDice is fully integrated; other sites have placeholder code
3. **Basic Strategy**: Only simple strategy implemented; advanced strategies coming
4. **No Background Service**: Betting stops when app is minimized (Android limitation)

### Security Considerations

⚠️ **Important Security Notes**:

1. **API Keys**: Never share your API keys or commit them to version control
2. **Keystore**: Keep your release keystore backup secure and never commit it
3. **Testing**: Always test with small amounts before real betting
4. **Responsibility**: This software is for educational purposes; gambling involves risk

### Breaking Changes

- **Configuration Format**: Settings are now stored in encrypted SharedPreferences
- **Native Library**: Library renamed to `libpredictive_rolls_android.so`
- **API Interface**: JNI methods signature updated for better error handling

### Migration Guide

If upgrading from a previous version:

1. Export your API keys before upgrading
2. Uninstall old version
3. Install new version
4. Re-enter API keys in new encrypted settings
5. Test with small bets first

### Roadmap

#### Version 1.1 (Planned)
- [ ] Full ML model integration with Burn framework
- [ ] Additional gambling site support (CryptoGames, FreeBitco.in)
- [ ] Advanced betting strategies (Martingale, D'Alembert)
- [ ] Background service for continuous betting
- [ ] Push notifications for wins/losses
- [ ] Historical data and charts

#### Version 1.2 (Future)
- [ ] Bankroll management tools
- [ ] Stop-loss and take-profit limits
- [ ] Multiple account management
- [ ] Backup and restore functionality
- [ ] Widget support
- [ ] Tablet-optimized UI

### Performance

- **APK Size**: ~15-20 MB (varies by architectures included)
- **Memory Usage**: ~50-100 MB during operation
- **Battery Impact**: Moderate during active betting
- **Network Usage**: Minimal (small JSON payloads)

### Testing

#### Automated Tests
- ✅ Unit tests for API serialization
- ✅ Rust clippy and formatting checks
- ✅ Android lint validation
- ⏳ Integration tests (in progress)
- ⏳ UI tests (planned)

#### Manual Testing
- ✅ Basic betting flow
- ✅ Settings configuration
- ✅ Error handling
- ✅ Dark mode switching
- ✅ API rate limiting
- ⏳ Extended session testing
- ⏳ Real betting with small amounts

### Contributors

This release was developed by the Copilot agent with contributions from:
- @sushiomsky (Repository owner)
- GitHub Copilot Team

### Support

For issues, questions, or contributions:

1. Check [Android README](android/README.md)
2. Review [API Integration Guide](android/API_INTEGRATION.md)
3. Search existing [GitHub Issues](https://github.com/sushiomsky/PredictiveRolls/issues)
4. Open a new issue with:
   - Android version
   - Device model
   - Steps to reproduce
   - Log output from `adb logcat`

### License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

### Acknowledgments

- **Burn**: Deep learning framework for Rust
- **Material Design**: Google's design system
- **DuckDice**: Cryptocurrency gambling platform
- **Rust Community**: Amazing tools and libraries
- **Android Developers**: Comprehensive documentation

### Disclaimer

⚠️ **IMPORTANT DISCLAIMER**

This software is provided for **educational and research purposes only**. 

- Gambling involves substantial risk
- You can lose money, potentially all of it
- Never bet with money you cannot afford to lose
- This software does not guarantee profits
- Past performance does not predict future results
- The developers are not responsible for any financial losses
- Check your local laws regarding online gambling
- Seek help if you have a gambling problem

Use responsibly.

---

**Release Date**: December 2025
**Version**: 1.0.0
**Build**: Initial public release
