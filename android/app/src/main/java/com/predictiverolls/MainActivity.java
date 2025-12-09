package com.predictiverolls;

import android.app.AlertDialog;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.widget.Button;
import android.widget.TextView;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;
import androidx.security.crypto.EncryptedSharedPreferences;
import androidx.security.crypto.MasterKeys;

import java.io.IOException;
import java.security.GeneralSecurityException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class MainActivity extends AppCompatActivity {
    private static final String PREFS_NAME = "PredictiveRollsPrefs";
    
    private TextView predictionValue;
    private TextView confidenceValue;
    private TextView balanceValue;
    private TextView winRateValue;
    private TextView logTextView;
    private Button settingsButton;
    private Button startStopButton;
    
    private boolean isRunning = false;
    private ExecutorService executorService;
    private Handler mainHandler;
    private SharedPreferences prefs;
    
    static {
        System.loadLibrary("predictive_rolls_android");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        
        // Initialize views
        predictionValue = findViewById(R.id.predictionValue);
        confidenceValue = findViewById(R.id.confidenceValue);
        balanceValue = findViewById(R.id.balanceValue);
        winRateValue = findViewById(R.id.winRateValue);
        logTextView = findViewById(R.id.logTextView);
        settingsButton = findViewById(R.id.settingsButton);
        startStopButton = findViewById(R.id.startStopButton);
        
        // Initialize executor and handler
        executorService = Executors.newSingleThreadExecutor();
        mainHandler = new Handler(Looper.getMainLooper());
        
        // Initialize encrypted preferences
        try {
            String masterKeyAlias = MasterKeys.getOrCreate(MasterKeys.AES256_GCM_SPEC);
            prefs = EncryptedSharedPreferences.create(
                PREFS_NAME,
                masterKeyAlias,
                this,
                EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
                EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
            );
        } catch (GeneralSecurityException | IOException e) {
            e.printStackTrace();
            // Fallback to regular SharedPreferences
            prefs = getSharedPreferences(PREFS_NAME, MODE_PRIVATE);
        }
        
        // Show disclaimer on first launch
        if (!prefs.getBoolean("disclaimer_accepted", false)) {
            showDisclaimer();
        }
        
        // Set up button listeners
        settingsButton.setOnClickListener(v -> {
            Intent intent = new Intent(MainActivity.this, SettingsActivity.class);
            startActivity(intent);
        });
        
        startStopButton.setOnClickListener(v -> {
            if (!isRunning) {
                startBetting();
            } else {
                stopBetting();
            }
        });
        
        // Initialize native library
        PredictiveRollsNative.initialize();
        appendLog("PredictiveRolls initialized");
    }
    
    private void showDisclaimer() {
        new AlertDialog.Builder(this)
            .setTitle(R.string.disclaimer_title)
            .setMessage(R.string.disclaimer_message)
            .setCancelable(false)
            .setPositiveButton(R.string.accept, (dialog, which) -> {
                prefs.edit().putBoolean("disclaimer_accepted", true).apply();
            })
            .setNegativeButton(R.string.decline, (dialog, which) -> {
                finish();
            })
            .show();
    }
    
    private void startBetting() {
        String apiKey = prefs.getString("api_key", "");
        String site = prefs.getString("site", "duck_dice");
        String currency = prefs.getString("currency", "BTC");
        String strategy = prefs.getString("strategy", "None");
        
        if (apiKey.isEmpty()) {
            Toast.makeText(this, "Please configure API key in settings", Toast.LENGTH_LONG).show();
            return;
        }
        
        isRunning = true;
        startStopButton.setText(R.string.stop_betting);
        startStopButton.setEnabled(false);
        
        appendLog("Starting betting session...");
        appendLog("Site: " + site);
        appendLog("Currency: " + currency);
        appendLog("Strategy: " + strategy);
        
        executorService.execute(() -> {
            try {
                // Configure the native library
                PredictiveRollsNative.configure(site, apiKey, currency, strategy);
                
                mainHandler.post(() -> {
                    startStopButton.setEnabled(true);
                    appendLog("Betting session started");
                });
                
                // Start betting loop
                while (isRunning) {
                    try {
                        float prediction = PredictiveRollsNative.getPrediction();
                        float confidence = PredictiveRollsNative.getConfidence();
                        
                        mainHandler.post(() -> {
                            predictionValue.setText(String.format("%.2f", prediction));
                            confidenceValue.setText(String.format("%.2f%%", confidence * 100));
                        });
                        
                        boolean betResult = PredictiveRollsNative.placeBet(prediction, confidence);
                        
                        String balance = PredictiveRollsNative.getBalance();
                        float winRate = PredictiveRollsNative.getWinRate();
                        
                        mainHandler.post(() -> {
                            balanceValue.setText(balance + " " + currency);
                            winRateValue.setText(String.format("%.2f%%", winRate * 100));
                            
                            String result = betResult ? "WIN" : "LOSS";
                            appendLog(String.format("Bet: %.2f | Confidence: %.2f%% | Result: %s", 
                                prediction, confidence * 100, result));
                        });
                        
                        Thread.sleep(5000); // Wait between bets
                        
                    } catch (InterruptedException e) {
                        break;
                    }
                }
                
            } catch (Exception e) {
                mainHandler.post(() -> {
                    appendLog("Error: " + e.getMessage());
                    Toast.makeText(MainActivity.this, "Error: " + e.getMessage(), Toast.LENGTH_LONG).show();
                    stopBetting();
                });
            }
        });
    }
    
    private void stopBetting() {
        isRunning = false;
        startStopButton.setText(R.string.start_betting);
        appendLog("Betting session stopped");
    }
    
    private void appendLog(String message) {
        String currentLog = logTextView.getText().toString();
        String timestamp = new java.text.SimpleDateFormat("HH:mm:ss").format(new java.util.Date());
        logTextView.setText(currentLog + "\n[" + timestamp + "] " + message);
        
        // Auto-scroll to bottom
        logTextView.post(() -> {
            int scrollAmount = logTextView.getLayout().getLineTop(logTextView.getLineCount()) - logTextView.getHeight();
            if (scrollAmount > 0) {
                logTextView.scrollTo(0, scrollAmount);
            }
        });
    }
    
    @Override
    protected void onDestroy() {
        super.onDestroy();
        stopBetting();
        executorService.shutdown();
        PredictiveRollsNative.cleanup();
    }
}
