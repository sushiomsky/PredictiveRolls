# DuckDice Bot API Integration Guide

This document explains how the PredictiveRolls Android app integrates with the DuckDice Bot API.

## Overview

The Android app uses a hybrid architecture combining:
- **Java/Android UI Layer**: Native Android interface with Material Design 3
- **Rust API Client**: High-performance HTTP client with async operations
- **JNI Bridge**: Seamless communication between Java and Rust

## Architecture

```
┌─────────────────────────────────────────┐
│         Android App (Java)              │
│  - MainActivity                         │
│  - SettingsActivity                     │
│  - Material Design 3 UI                 │
└──────────────┬──────────────────────────┘
               │ JNI Calls
┌──────────────▼──────────────────────────┐
│      JNI Bridge (Rust)                  │
│  - android-lib/src/lib.rs               │
│  - State management                     │
│  - Tokio async runtime                  │
└──────────────┬──────────────────────────┘
               │ API Calls
┌──────────────▼──────────────────────────┐
│    DuckDice API Client (Rust)           │
│  - android-lib/src/duckdice_api.rs      │
│  - HTTP client (reqwest)                │
│  - JSON serialization (serde)           │
└──────────────┬──────────────────────────┘
               │ HTTPS
┌──────────────▼──────────────────────────┐
│      DuckDice Bot API                   │
│  https://duckdice.io/api/               │
└─────────────────────────────────────────┘
```

## API Endpoints Used

### 1. User Info
**Endpoint**: `GET /api/bot/user-info?api_key={key}`

**Purpose**: Fetch user account information and balances

**Response**:
```json
{
  "hash": "user_hash",
  "username": "username",
  "created_at": 1234567890,
  "level": 42,
  "balances": [
    {
      "currency": "BTC",
      "main": "0.00100000",
      "faucet": "0.00001000",
      "affiliate": "0.00000000"
    }
  ]
}
```

**Usage in App**:
- Called during configuration to fetch initial balance
- Called periodically to update balance display
- Determines available currencies for betting

### 2. Place Bet
**Endpoint**: `POST /api/play?api_key={key}`

**Purpose**: Place a bet on the dice game

**Request**:
```json
{
  "symbol": "BTC",
  "chance": 50.0,
  "isHigh": true,
  "amount": 0.00000100,
  "faucet": true
}
```

**Response**:
```json
{
  "bet": {
    "hash": "bet_hash",
    "symbol": "BTC",
    "choice": "high",
    "result": true,
    "number": 75,
    "chance": 50.0,
    "payout": 1.98,
    "betAmount": "0.00000100",
    "winAmount": "0.00000198",
    "profit": "0.00000098",
    "nonce": 12345
  },
  "user": {
    "hash": "user_hash",
    "username": "username",
    "balance": "0.00101098"
  }
}
```

**Usage in App**:
- Called for each bet placed
- Updates balance from response
- Logs win/loss results
- Updates statistics

### 3. Randomize Seed
**Endpoint**: `POST /api/randomize?api_key={key}`

**Purpose**: Change the client seed for provably fair gaming

**Request**:
```json
{
  "clientSeed": "random_seed_string"
}
```

**Usage in App**:
- Called when resetting after losses
- Maintains provably fair guarantee
- Generates new random seed

## Rate Limiting

The DuckDice API implements rate limiting to prevent abuse.

### Handling Rate Limits

**Response Code**: `429 Too Many Requests`
**Header**: `Retry-After: {seconds}`

The API client automatically:
1. Detects `429` response
2. Extracts `Retry-After` header
3. Returns `RateLimitError` with wait time
4. Pauses betting temporarily

**Implementation**:
```rust
fn handle_rate_limit(&self, response: &reqwest::Response) -> Result<(), DuckDiceError> {
    if response.status().as_u16() == 429 {
        if let Some(retry_after) = response.headers().get("retry-after") {
            if let Ok(seconds) = retry_after.to_str()?.parse::<u64>() {
                return Err(DuckDiceError::RateLimitError(seconds));
            }
        }
        return Err(DuckDiceError::RateLimitError(60)); // Default
    }
    Ok(())
}
```

### Best Practices

1. **Minimum Delay**: Wait at least 5 seconds between bets
2. **Exponential Backoff**: Increase delay after repeated rate limits
3. **Burst Prevention**: Don't place multiple bets simultaneously
4. **Monitor Headers**: Check rate limit headers in responses

## Error Handling

### Error Types

```rust
pub enum DuckDiceError {
    NetworkError(String),      // Connection issues
    ApiError(String),           // API returned error
    JsonError(String),          // JSON parsing failed
    AuthenticationError,        // Invalid API key
    RateLimitError(u64),       // Rate limited (seconds to wait)
}
```

### Error Recovery

**Network Errors**:
- Retry with exponential backoff
- Check internet connectivity
- Notify user of connection issues

**API Errors**:
- Log detailed error message
- Display user-friendly error
- Check API key validity

**Authentication Errors**:
- Prompt user to re-enter API key
- Stop betting session
- Clear invalid credentials

**Rate Limit Errors**:
- Pause betting for specified duration
- Display countdown timer
- Resume automatically after wait

## Security Considerations

### 1. API Key Storage

**Android Keystore**:
```java
String masterKeyAlias = MasterKeys.getOrCreate(MasterKeys.AES256_GCM_SPEC);
SharedPreferences prefs = EncryptedSharedPreferences.create(
    "PredictiveRollsPrefs",
    masterKeyAlias,
    context,
    EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
    EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
);
```

**Features**:
- AES-256-GCM encryption
- Hardware-backed security (when available)
- Automatic key rotation
- Secure deletion on app uninstall

### 2. Network Security

**HTTPS Only**:
```xml
<application
    android:usesCleartextTraffic="false">
```

**Certificate Pinning** (Future Enhancement):
```rust
// TODO: Implement certificate pinning
let client = reqwest::Client::builder()
    .add_root_certificate(cert)
    .build()?;
```

### 3. Code Obfuscation

**ProGuard/R8 Rules**:
```proguard
# Keep JNI interface
-keep class com.predictiverolls.PredictiveRollsNative { *; }

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}
```

## Betting Strategy Integration

### Strategy Parameters

The app determines betting parameters based on ML predictions:

```rust
// Determine bet direction
let is_high = prediction > 50.0;

// Adjust chance based on confidence
let chance = if confidence > 0.7 {
    50.0  // High confidence: 50% chance
} else if confidence > 0.5 {
    40.0  // Medium confidence: 40% chance
} else {
    30.0  // Low confidence: 30% chance
};

// Calculate bet amount (more aggressive with high confidence)
let bet_amount = if confidence > 0.7 {
    0.00000100  // Higher bet
} else {
    0.00000050  // Lower bet
};
```

### Future Enhancements

1. **Multiple Strategies**: Support for different betting algorithms
2. **Bankroll Management**: Advanced balance protection
3. **Stop-Loss/Take-Profit**: Automatic session limits
4. **Progressive Betting**: Martingale, D'Alembert, etc.

## Performance Optimization

### Async Operations

All API calls run asynchronously:
```rust
// Tokio runtime in JNI
lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = 
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");
}

// Async API call from sync JNI context
let result = RUNTIME.block_on(client.place_bet(bet_request));
```

### Connection Pooling

Reqwest automatically pools connections:
- Reuses TCP connections
- Reduces handshake overhead
- Improves latency

### Memory Management

- Limited history size (configurable)
- Periodic state cleanup
- Efficient JSON parsing with serde

## Testing the Integration

### Manual Testing

1. **Configure API Key**:
   - Go to Settings
   - Enter valid DuckDice API key
   - Select currency (BTC, ETH, etc.)
   - Choose strategy

2. **Start Betting**:
   - Tap "Start Betting"
   - Monitor activity log
   - Check balance updates
   - Observe win/loss statistics

3. **Error Scenarios**:
   - Test with invalid API key
   - Test with no internet connection
   - Test with rate limiting
   - Test app suspend/resume

### Automated Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bet_request_serialization() {
        let bet = BetRequest {
            symbol: "BTC".to_string(),
            chance: 50.0,
            is_high: true,
            amount: 0.00000100,
            faucet: Some(true),
        };
        
        let json = serde_json::to_string(&bet).unwrap();
        assert!(json.contains("\"symbol\":\"BTC\""));
    }
    
    #[tokio::test]
    async fn test_user_info() {
        let client = DuckDiceClient::new("test_key".to_string()).unwrap();
        // Add API mock for testing
    }
}
```

## Monitoring and Debugging

### Logging

**Android Logs**:
```bash
adb logcat | grep PredictiveRolls
```

**Log Levels**:
- `DEBUG`: API calls, predictions, confidence
- `INFO`: Bet results, balance updates
- `WARN`: Rate limits, retries
- `ERROR`: API failures, exceptions

### Network Debugging

**Charles Proxy / Wireshark**:
1. Configure proxy in Android settings
2. Install SSL certificate
3. Monitor HTTPS traffic to duckdice.io
4. Inspect requests and responses

**Common Issues**:
- 401 Unauthorized: Invalid API key
- 429 Too Many Requests: Rate limited
- 500 Server Error: DuckDice API issue
- Timeout: Network or server slow

## API Documentation

For complete DuckDice Bot API documentation:
- Official Docs: https://duckdice.io/bot-api
- API Forum: Check DuckDice community forums
- Support: Contact DuckDice support for API issues

## Contributing

To contribute API integration improvements:

1. Test thoroughly with small amounts
2. Add proper error handling
3. Document new endpoints
4. Update this guide
5. Submit pull request

## Security Disclosure

If you discover security vulnerabilities in the API integration:

1. **Do not** open a public issue
2. Email: security contact (TBD)
3. Include detailed description
4. Wait for acknowledgment before disclosure

## License

This API integration follows the project's MIT License. See [LICENSE](../LICENSE) for details.
