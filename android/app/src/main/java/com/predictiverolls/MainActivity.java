package com.predictiverolls;

import android.app.AlertDialog;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.view.Menu;
import android.view.MenuItem;
import android.widget.TextView;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;
import androidx.security.crypto.EncryptedSharedPreferences;
import androidx.security.crypto.MasterKeys;

import com.google.android.material.appbar.MaterialToolbar;
import com.google.android.material.button.MaterialButton;

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
    private TextView totalBetsValue;
    private TextView logTextView;
    private MaterialButton settingsButton;
    private MaterialButton startStopButton;
    private MaterialToolbar toolbar;
    
    private boolean isRunning = false;
    private int totalBets = 0;
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
        
        // Set up toolbar
        toolbar = findViewById(R.id.toolbar);
        setSupportActionBar(toolbar);
        
        // Initialize views
        predictionValue = findViewById(R.id.predictionValue);
        confidenceValue = findViewById(R.id.confidenceValue);
        balanceValue = findViewById(R.id.balanceValue);
        winRateValue = findViewById(R.id.winRateValue);
        totalBetsValue = findViewById(R.id.totalBetsValue);
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
        appendLog("✓ PredictiveRolls initialized");
        appendLog("  Configure settings to get started");
    }
    
    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        getMenuInflater().inflate(R.menu.main_menu, menu);
        return true;
    }
    
    @Override
    public boolean onOptionsItemSelected(MenuItem item) {
        int id = item.getItemId();
        
        if (id == R.id.action_settings) {
            Intent intent = new Intent(MainActivity.this, SettingsActivity.class);
            startActivity(intent);
            return true;
        } else if (id == R.id.action_about) {
            showAboutDialog();
            return true;
        }
        
        return super.onOptionsItemSelected(item);
    }
    
    private void showAboutDialog() {
        new AlertDialog.Builder(this)
            .setTitle("About PredictiveRolls")
            .setMessage("PredictiveRolls Android\nVersion 1.0\n\n" +
                "A machine learning-based predictive dice rolling application.\n\n" +
                "⚠️ For educational and research purposes only.\n" +
                "Gambling involves risk. Never gamble with money you cannot afford to lose.")
            .setPositiveButton("OK", null)
            .show();
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
            Toast.makeText(this, "⚠️ Please configure API key in settings", Toast.LENGTH_LONG).show();
            return;
        }
        
        isRunning = true;
        startStopButton.setText(R.string.stop_betting);
        startStopButton.setIcon(getDrawable(android.R.drawable.ic_media_pause));
        startStopButton.setEnabled(false);
        settingsButton.setEnabled(false);
        
        appendLog("▶ Starting betting session...");
        appendLog("  Site: " + site);
        appendLog("  Currency: " + currency);
        appendLog("  Strategy: " + strategy);
        
        executorService.execute(() -> {
            try {
                // Configure the native library
                PredictiveRollsNative.configure(site, apiKey, currency, strategy);
                
                mainHandler.post(() -> {
                    startStopButton.setEnabled(true);
                    appendLog("✓ Betting session started");
                });
                
                // Start betting loop
                while (isRunning) {
                    try {
                        float prediction = PredictiveRollsNative.getPrediction();
                        float confidence = PredictiveRollsNative.getConfidence();
                        
                        totalBets++;
                        
                        mainHandler.post(() -> {
                            predictionValue.setText(String.format("%.2f", prediction));
                            confidenceValue.setText(String.format("%.1f%%", confidence * 100));
                            totalBetsValue.setText(String.valueOf(totalBets));
                        });
                        
                        boolean betResult = PredictiveRollsNative.placeBet(prediction, confidence);
                        
                        String balance = PredictiveRollsNative.getBalance();
                        float winRate = PredictiveRollsNative.getWinRate();
                        
                        final String result = betResult ? "WIN ✓" : "LOSS ✗";
                        final String emoji = betResult ? "🎉" : "📉";
                        
                        mainHandler.post(() -> {
                            balanceValue.setText(balance + " " + currency);
                            winRateValue.setText(String.format("%.1f%%", winRate * 100));
                            
                            appendLog(String.format("%s Bet #%d | Pred: %.2f | Conf: %.1f%% | %s", 
                                emoji, totalBets, prediction, confidence * 100, result));
                        });
                        
                        Thread.sleep(5000); // Wait between bets
                        
                    } catch (InterruptedException e) {
                        break;
                    }
                }
                
            } catch (Exception e) {
                mainHandler.post(() -> {
                    appendLog("❌ Error: " + e.getMessage());
                    Toast.makeText(MainActivity.this, "Error: " + e.getMessage(), Toast.LENGTH_LONG).show();
                    stopBetting();
                });
            }
        });
    }
    
    private void stopBetting() {
        isRunning = false;
        startStopButton.setText(R.string.start_betting);
        startStopButton.setIcon(getDrawable(android.R.drawable.ic_media_play));
        settingsButton.setEnabled(true);
        appendLog("■ Betting session stopped");
        appendLog("  Total bets placed: " + totalBets);
    }
    
    private void appendLog(String message) {
        String currentLog = logTextView.getText().toString();
        String timestamp = new java.text.SimpleDateFormat("HH:mm:ss").format(new java.util.Date());
        logTextView.setText(currentLog + "\n[" + timestamp + "] " + message);
        
        // Auto-scroll to bottom
        logTextView.post(() -> {
            if (logTextView.getLayout() != null) {
                int scrollAmount = logTextView.getLayout().getLineTop(logTextView.getLineCount()) - logTextView.getHeight();
                if (scrollAmount > 0) {
                    logTextView.scrollTo(0, scrollAmount);
                }
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
