mod duckdice_api;

use duckdice_api::{BetRequest, DuckDiceClient, DuckDiceError};
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jfloat};
use jni::JNIEnv;
use log::{debug, error, info, warn};
use std::sync::Mutex;

// Global state for the Android app
lazy_static::lazy_static! {
    static ref STATE: Mutex<AppState> = Mutex::new(AppState::default());
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("Failed to create tokio runtime");
}

#[derive(Default)]
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
    use_faucet: bool,
    api_client: Option<DuckDiceClient>,
}

impl AppState {
    fn win_rate(&self) -> f32 {
        if self.total_bets == 0 {
            0.0
        } else {
            self.wins as f32 / self.total_bets as f32
        }
    }

    fn initialize_client(&mut self) -> Result<(), DuckDiceError> {
        if self.api_key.is_empty() {
            return Err(DuckDiceError::AuthenticationError);
        }
        self.api_client = Some(DuckDiceClient::new(self.api_key.clone())?);
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, _reserved: *mut std::ffi::c_void) -> jni::sys::jint {
    // Initialize Android logger
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("PredictiveRolls"),
    );
    
    info!("PredictiveRolls native library loaded");
    jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
pub extern "C" fn Java_com_predictiverolls_PredictiveRollsNative_initialize(
    env: JNIEnv,
    _class: JClass,
) {
    info!("Initializing PredictiveRolls native library");
    
    let mut state = STATE.lock().unwrap();
    state.initialized = true;
    
    info!("Native library initialized successfully");
}

#[no_mangle]
pub extern "C" fn Java_com_predictiverolls_PredictiveRollsNative_configure(
    env: JNIEnv,
    _class: JClass,
    site: JString,
    api_key: JString,
    currency: JString,
    strategy: JString,
) {
    let site_str: String = env
        .get_string(site)
        .expect("Couldn't get site string")
        .into();
    let api_key_str: String = env
        .get_string(api_key)
        .expect("Couldn't get API key string")
        .into();
    let currency_str: String = env
        .get_string(currency)
        .expect("Couldn't get currency string")
        .into();
    let strategy_str: String = env
        .get_string(strategy)
        .expect("Couldn't get strategy string")
        .into();

    info!("Configuring: site={}, currency={}, strategy={}", site_str, currency_str, strategy_str);
    
    let mut state = STATE.lock().unwrap();
    state.site = site_str.clone();
    state.api_key = api_key_str;
    state.currency = currency_str;
    state.strategy = strategy_str;
    
    // Initialize API client based on site
    if site_str == "duck_dice" || site_str == "duckdice" {
        match state.initialize_client() {
            Ok(_) => {
                info!("DuckDice API client initialized successfully");
                
                // Fetch initial balance from API
                if let Some(client) = &state.api_client {
                    match RUNTIME.block_on(client.get_user_info()) {
                        Ok(user_info) => {
                            info!("User: {} (Level {})", user_info.username, user_info.level);
                            
                            // Find balance for the configured currency
                            for balance in user_info.balances {
                                if balance.currency == state.currency {
                                    let balance_str = if state.use_faucet {
                                        balance.faucet.as_ref()
                                    } else {
                                        balance.main.as_ref()
                                    };
                                    
                                    if let Some(bal_str) = balance_str {
                                        state.balance = bal_str.parse().unwrap_or(0.0);
                                        info!("Initial balance: {} {}", state.balance, state.currency);
                                    }
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to fetch user info: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to initialize API client: {}", e);
            }
        }
    } else {
        warn!("Site '{}' not yet supported with real API integration", site_str);
        state.balance = 1.0; // Fallback to demo balance
    }
    
    debug!("Configuration complete");
}

#[no_mangle]
pub extern "C" fn Java_com_predictiverolls_PredictiveRollsNative_getPrediction(
    _env: JNIEnv,
    _class: JClass,
) -> jfloat {
    let mut state = STATE.lock().unwrap();
    
    // TODO: Integrate with the full Burn-based ML model from the main crate
    // For now, generate a pseudo-prediction for demonstration purposes
    // Real implementation should:
    // 1. Load the trained model from MODEL_DIR
    // 2. Prepare input data from historical rolls
    // 3. Run inference through the neural network
    // 4. Return the model's prediction
    state.prediction = 50.0 + (rand::random::<f32>() * 10.0 - 5.0);
    
    debug!("Generated prediction: {}", state.prediction);
    state.prediction
}

#[no_mangle]
pub extern "C" fn Java_com_predictiverolls_PredictiveRollsNative_getConfidence(
    _env: JNIEnv,
    _class: JClass,
) -> jfloat {
    let mut state = STATE.lock().unwrap();
    
    // TODO: Calculate actual confidence from ML model output
    // Generate confidence value for demonstration
    state.confidence = 0.5 + rand::random::<f32>() * 0.3;
    
    debug!("Generated confidence: {}", state.confidence);
    state.confidence
}

#[no_mangle]
pub extern "C" fn Java_com_predictiverolls_PredictiveRollsNative_placeBet(
    _env: JNIEnv,
    _class: JClass,
    prediction: jfloat,
    confidence: jfloat,
) -> jboolean {
    let mut state = STATE.lock().unwrap();
    
    state.total_bets += 1;
    
    // Use real DuckDice API if client is initialized
    if let Some(client) = &state.api_client {
        // Determine bet parameters based on prediction and confidence
        let is_high = prediction > 50.0;
        let chance = if confidence > 0.7 {
            50.0 // High confidence: 50% chance
        } else if confidence > 0.5 {
            40.0 // Medium confidence: 40% chance
        } else {
            30.0 // Low confidence: 30% chance
        };
        
        // Calculate bet amount (simple strategy: bet more with higher confidence)
        let bet_amount = if confidence > 0.7 {
            0.00000100 // Higher bet
        } else {
            0.00000050 // Lower bet
        };
        
        let bet_request = BetRequest {
            symbol: state.currency.clone(),
            chance,
            is_high,
            amount: bet_amount,
            faucet: if state.use_faucet { Some(true) } else { None },
        };
        
        match RUNTIME.block_on(client.place_bet(bet_request)) {
            Ok(response) => {
                let won = response.bet.result;
                
                if won {
                    state.wins += 1;
                    info!("BET WON! Number: {}, Profit: {} {}", 
                        response.bet.number, response.bet.profit, state.currency);
                } else {
                    info!("Bet lost. Number: {}, Loss: {} {}", 
                        response.bet.number, response.bet.bet_amount, state.currency);
                }
                
                // Update balance from API response
                if let Ok(new_balance) = response.user.balance.parse::<f64>() {
                    state.balance = new_balance;
                }
                
                return if won { 1 } else { 0 };
            }
            Err(e) => {
                error!("Bet failed: {}", e);
                
                // Handle rate limiting
                if let DuckDiceError::RateLimitError(seconds) = e {
                    warn!("Rate limited, waiting {} seconds", seconds);
                    // In a real app, we should pause betting and notify the user
                }
                
                // Return false on error
                return 0;
            }
        }
    }
    
    // Fallback to simulation if no API client
    warn!("No API client initialized, using simulation mode");
    let won = rand::random::<f32>() < confidence;
    
    if won {
        state.wins += 1;
        state.balance += 0.01;
        info!("SIM: Bet WON: prediction={}, confidence={}", prediction, confidence);
    } else {
        state.balance -= 0.01;
        info!("SIM: Bet LOST: prediction={}, confidence={}", prediction, confidence);
    }
    
    if won { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn Java_com_predictiverolls_PredictiveRollsNative_getBalance(
    env: JNIEnv,
    _class: JClass,
) -> jni::sys::jstring {
    let mut state = STATE.lock().unwrap();
    
    // Optionally fetch fresh balance from API
    if let Some(client) = &state.api_client {
        if let Ok(user_info) = RUNTIME.block_on(client.get_user_info()) {
            for balance in user_info.balances {
                if balance.currency == state.currency {
                    let balance_str = if state.use_faucet {
                        balance.faucet.as_ref()
                    } else {
                        balance.main.as_ref()
                    };
                    
                    if let Some(bal_str) = balance_str {
                        state.balance = bal_str.parse().unwrap_or(state.balance);
                    }
                    break;
                }
            }
        }
    }
    
    let balance_str = format!("{:.8}", state.balance);
    
    env.new_string(balance_str)
        .expect("Couldn't create java string")
        .into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_predictiverolls_PredictiveRollsNative_getWinRate(
    _env: JNIEnv,
    _class: JClass,
) -> jfloat {
    let state = STATE.lock().unwrap();
    state.win_rate()
}

#[no_mangle]
pub extern "C" fn Java_com_predictiverolls_PredictiveRollsNative_cleanup(
    _env: JNIEnv,
    _class: JClass,
) {
    info!("Cleaning up native library");
    
    let mut state = STATE.lock().unwrap();
    *state = AppState::default();
    
    info!("Cleanup complete");
}
