use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jfloat};
use jni::JNIEnv;
use log::{debug, error, info};
use std::sync::Mutex;

// Global state for the Android app
lazy_static::lazy_static! {
    static ref STATE: Mutex<AppState> = Mutex::new(AppState::default());
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
}

impl AppState {
    fn win_rate(&self) -> f32 {
        if self.total_bets == 0 {
            0.0
        } else {
            self.wins as f32 / self.total_bets as f32
        }
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
    state.site = site_str;
    state.api_key = api_key_str;
    state.currency = currency_str;
    state.strategy = strategy_str;
    state.balance = 1.0; // Initial balance for demo
    
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
    
    // TODO: Integrate with actual gambling site APIs from the main crate
    // Real implementation should:
    // 1. Use the configured site (DuckDice, CryptoGames, FreeBitco.in)
    // 2. Make authenticated API call with the prediction
    // 3. Handle the response and update balance from API
    // 4. Apply the configured strategy (AiFight, BlaksRunner, etc.)
    // For now, simulate bet result for demonstration
    let won = rand::random::<f32>() < confidence;
    
    if won {
        state.wins += 1;
        state.balance += 0.01; // Demo win amount
        info!("Bet WON: prediction={}, confidence={}", prediction, confidence);
    } else {
        state.balance -= 0.01; // Demo loss amount
        info!("Bet LOST: prediction={}, confidence={}", prediction, confidence);
    }
    
    if won { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn Java_com_predictiverolls_PredictiveRollsNative_getBalance(
    env: JNIEnv,
    _class: JClass,
) -> jni::sys::jstring {
    let state = STATE.lock().unwrap();
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
