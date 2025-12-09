package com.predictiverolls;

import android.content.SharedPreferences;
import android.os.Bundle;
import android.widget.ArrayAdapter;
import android.widget.Button;
import android.widget.EditText;
import android.widget.Spinner;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;
import androidx.security.crypto.EncryptedSharedPreferences;
import androidx.security.crypto.MasterKeys;

import java.io.IOException;
import java.security.GeneralSecurityException;

public class SettingsActivity extends AppCompatActivity {
    private static final String PREFS_NAME = "PredictiveRollsPrefs";
    
    private Spinner siteSpinner;
    private EditText apiKeyInput;
    private EditText currencyInput;
    private Spinner strategySpinner;
    private Button saveButton;
    private SharedPreferences prefs;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_settings);
        
        // Enable back button
        if (getSupportActionBar() != null) {
            getSupportActionBar().setDisplayHomeAsUpEnabled(true);
        }
        
        // Initialize views
        siteSpinner = findViewById(R.id.siteSpinner);
        apiKeyInput = findViewById(R.id.apiKeyInput);
        currencyInput = findViewById(R.id.currencyInput);
        strategySpinner = findViewById(R.id.strategySpinner);
        saveButton = findViewById(R.id.saveButton);
        
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
            prefs = getSharedPreferences(PREFS_NAME, MODE_PRIVATE);
        }
        
        // Set up site spinner
        String[] sites = {
            getString(R.string.site_duck_dice),
            getString(R.string.site_crypto_games),
            getString(R.string.site_free_bitco)
        };
        ArrayAdapter<String> siteAdapter = new ArrayAdapter<>(this,
            android.R.layout.simple_spinner_item, sites);
        siteAdapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item);
        siteSpinner.setAdapter(siteAdapter);
        
        // Set up strategy spinner
        String[] strategies = {
            getString(R.string.strategy_none),
            getString(R.string.strategy_ai_fight),
            getString(R.string.strategy_blaks_runner),
            getString(R.string.strategy_my_strategy)
        };
        ArrayAdapter<String> strategyAdapter = new ArrayAdapter<>(this,
            android.R.layout.simple_spinner_item, strategies);
        strategyAdapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item);
        strategySpinner.setAdapter(strategyAdapter);
        
        // Load saved settings
        loadSettings();
        
        // Set up save button
        saveButton.setOnClickListener(v -> saveSettings());
    }
    
    private void loadSettings() {
        String site = prefs.getString("site", "duck_dice");
        String apiKey = prefs.getString("api_key", "");
        String currency = prefs.getString("currency", "BTC");
        String strategy = prefs.getString("strategy", "None");
        
        // Set site spinner selection
        if (site.equals("duck_dice")) {
            siteSpinner.setSelection(0);
        } else if (site.equals("crypto_games")) {
            siteSpinner.setSelection(1);
        } else if (site.equals("free_bitco_in")) {
            siteSpinner.setSelection(2);
        }
        
        // Set strategy spinner selection
        if (strategy.equals("None")) {
            strategySpinner.setSelection(0);
        } else if (strategy.equals("AiFight")) {
            strategySpinner.setSelection(1);
        } else if (strategy.equals("BlaksRunner")) {
            strategySpinner.setSelection(2);
        } else if (strategy.equals("MyStrategy")) {
            strategySpinner.setSelection(3);
        }
        
        apiKeyInput.setText(apiKey);
        currencyInput.setText(currency);
    }
    
    private void saveSettings() {
        String site;
        int sitePosition = siteSpinner.getSelectedItemPosition();
        if (sitePosition == 0) {
            site = "duck_dice";
        } else if (sitePosition == 1) {
            site = "crypto_games";
        } else {
            site = "free_bitco_in";
        }
        
        String strategy;
        int strategyPosition = strategySpinner.getSelectedItemPosition();
        if (strategyPosition == 0) {
            strategy = "None";
        } else if (strategyPosition == 1) {
            strategy = "AiFight";
        } else if (strategyPosition == 2) {
            strategy = "BlaksRunner";
        } else {
            strategy = "MyStrategy";
        }
        
        String apiKey = apiKeyInput.getText().toString();
        String currency = currencyInput.getText().toString();
        
        prefs.edit()
            .putString("site", site)
            .putString("api_key", apiKey)
            .putString("currency", currency)
            .putString("strategy", strategy)
            .apply();
        
        Toast.makeText(this, "Settings saved", Toast.LENGTH_SHORT).show();
        finish();
    }
    
    @Override
    public boolean onSupportNavigateUp() {
        finish();
        return true;
    }
}
