/// DuckDice Bot API Client
/// 
/// This module provides a client for interacting with the DuckDice Bot API
/// as documented at https://duckdice.io/bot-api

use log::{debug, error, info};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Error types for DuckDice API operations
#[derive(Debug)]
pub enum DuckDiceError {
    NetworkError(String),
    ApiError(String),
    JsonError(String),
    AuthenticationError,
    RateLimitError(u64), // seconds to wait
}

impl std::fmt::Display for DuckDiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DuckDiceError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            DuckDiceError::ApiError(msg) => write!(f, "API error: {}", msg),
            DuckDiceError::JsonError(msg) => write!(f, "JSON error: {}", msg),
            DuckDiceError::AuthenticationError => write!(f, "Authentication failed"),
            DuckDiceError::RateLimitError(secs) => write!(f, "Rate limited, retry after {} seconds", secs),
        }
    }
}

impl std::error::Error for DuckDiceError {}

impl From<reqwest::Error> for DuckDiceError {
    fn from(err: reqwest::Error) -> Self {
        DuckDiceError::NetworkError(err.to_string())
    }
}

impl From<serde_json::Error> for DuckDiceError {
    fn from(err: serde_json::Error) -> Self {
        DuckDiceError::JsonError(err.to_string())
    }
}

/// User information response
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub hash: String,
    pub username: String,
    pub created_at: u64,
    pub level: u32,
    pub balances: Vec<Balance>,
}

/// Balance information
#[derive(Debug, Deserialize, Clone)]
pub struct Balance {
    pub currency: String,
    pub main: Option<String>,
    pub faucet: Option<String>,
    pub affiliate: Option<String>,
}

/// Bet request payload
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BetRequest {
    pub symbol: String,
    pub chance: f64,
    pub is_high: bool,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faucet: Option<bool>,
}

/// Bet response
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BetResponse {
    pub bet: BetInfo,
    pub user: UserInBet,
}

/// Bet information
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BetInfo {
    pub hash: String,
    pub symbol: String,
    pub choice: String,
    pub result: bool,
    pub number: u32,
    pub chance: f64,
    pub payout: f64,
    pub bet_amount: String,
    pub win_amount: String,
    pub profit: String,
    pub nonce: u64,
}

/// User info in bet response
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInBet {
    pub hash: String,
    pub username: String,
    pub balance: String,
}

/// DuckDice Bot API Client
pub struct DuckDiceClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl DuckDiceClient {
    /// Create a new DuckDice API client
    pub fn new(api_key: String) -> Result<Self, DuckDiceError> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "PredictiveRolls-Android/1.0".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            api_key,
            base_url: "https://duckdice.io/api".to_string(),
        })
    }

    /// Get user information
    pub async fn get_user_info(&self) -> Result<UserInfo, DuckDiceError> {
        let url = format!("{}/bot/user-info?api_key={}", self.base_url, self.api_key);
        
        debug!("Fetching user info from DuckDice");
        let response = self.client
            .get(&url)
            .send()
            .await?;

        self.handle_rate_limit(&response)?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("User info request failed: {} - {}", status, body);
            return Err(DuckDiceError::ApiError(format!("Status: {}, Body: {}", status, body)));
        }

        let user_info: UserInfo = response.json().await?;
        debug!("User info retrieved: {} (level {})", user_info.username, user_info.level);
        Ok(user_info)
    }

    /// Place a bet
    pub async fn place_bet(&self, bet: BetRequest) -> Result<BetResponse, DuckDiceError> {
        let url = format!("{}/play?api_key={}", self.base_url, self.api_key);
        
        info!("Placing bet: {} {} @ {} chance ({})", 
            bet.amount, bet.symbol, bet.chance, 
            if bet.is_high { "HIGH" } else { "LOW" });

        let response = self.client
            .post(&url)
            .json(&bet)
            .send()
            .await?;

        self.handle_rate_limit(&response)?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Bet request failed: {} - {}", status, body);
            return Err(DuckDiceError::ApiError(format!("Status: {}, Body: {}", status, body)));
        }

        let bet_response: BetResponse = response.json().await?;
        
        if bet_response.bet.result {
            info!("BET WON! Profit: {} {}", bet_response.bet.profit, bet_response.bet.symbol);
        } else {
            info!("Bet lost. Result: {}", bet_response.bet.number);
        }

        Ok(bet_response)
    }

    /// Randomize client seed
    pub async fn randomize_seed(&self, client_seed: String) -> Result<(), DuckDiceError> {
        let url = format!("{}/randomize?api_key={}", self.base_url, self.api_key);
        
        debug!("Randomizing seed");
        let payload = serde_json::json!({
            "clientSeed": client_seed
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        self.handle_rate_limit(&response)?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Seed randomization failed: {} - {}", status, body);
            return Err(DuckDiceError::ApiError(format!("Status: {}, Body: {}", status, body)));
        }

        info!("Seed randomized successfully");
        Ok(())
    }

    /// Handle rate limiting from response headers
    fn handle_rate_limit(&self, response: &reqwest::Response) -> Result<(), DuckDiceError> {
        if response.status().as_u16() == 429 {
            // Rate limited
            if let Some(retry_after) = response.headers().get("retry-after") {
                if let Ok(seconds_str) = retry_after.to_str() {
                    if let Ok(seconds) = seconds_str.parse::<u64>() {
                        return Err(DuckDiceError::RateLimitError(seconds));
                    }
                }
            }
            return Err(DuckDiceError::RateLimitError(60)); // Default to 60 seconds
        }
        Ok(())
    }
}

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
        assert!(json.contains("\"chance\":50"));
    }
}
