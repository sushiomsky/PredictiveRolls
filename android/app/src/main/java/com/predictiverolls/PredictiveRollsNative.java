package com.predictiverolls;

/**
 * JNI interface to the Rust native library.
 * This class provides methods to interact with the PredictiveRolls core functionality.
 */
public class PredictiveRollsNative {
    
    /**
     * Initialize the native library and load the ML model.
     */
    public static native void initialize();
    
    /**
     * Configure the betting session.
     * 
     * @param site The gambling site to use (e.g., "duck_dice", "crypto_games", "free_bitco_in")
     * @param apiKey The API key for authentication
     * @param currency The currency to use (e.g., "BTC", "ETH")
     * @param strategy The betting strategy (e.g., "None", "AiFight", "BlaksRunner", "MyStrategy")
     */
    public static native void configure(String site, String apiKey, String currency, String strategy);
    
    /**
     * Get the current prediction from the ML model.
     * 
     * @return The predicted dice roll value (0.0 to 100.0)
     */
    public static native float getPrediction();
    
    /**
     * Get the confidence level of the current prediction.
     * 
     * @return The confidence level (0.0 to 1.0)
     */
    public static native float getConfidence();
    
    /**
     * Place a bet with the given prediction and confidence.
     * 
     * @param prediction The predicted value
     * @param confidence The confidence level
     * @return true if the bet was won, false otherwise
     */
    public static native boolean placeBet(float prediction, float confidence);
    
    /**
     * Get the current balance.
     * 
     * @return The current balance as a string
     */
    public static native String getBalance();
    
    /**
     * Get the current win rate.
     * 
     * @return The win rate (0.0 to 1.0)
     */
    public static native float getWinRate();
    
    /**
     * Clean up resources and shut down the native library.
     */
    public static native void cleanup();
}
