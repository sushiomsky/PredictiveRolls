use crate::currency::Currency;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub enum TomlStrategies {
    AiFight,
    BlaksRunner,
    MyStrategy,
    #[default]
    None,
}

#[derive(Debug, Deserialize)]
pub struct CryptoGamesConfig {
    pub enabled: bool,
    pub api_key: String,
    pub currency: Currency,
    pub strategy: TomlStrategies,
}

#[derive(Debug, Deserialize)]
pub struct FreeBitcoInConfig {
    pub enabled: bool,
    pub btc_address: String,
    pub password: String,
    pub strategy: TomlStrategies,
}

#[derive(Debug, Deserialize)]
pub struct DuckDiceConfig {
    pub enabled: bool,
    pub api_key: String,
    pub currency: Currency,
    pub strategy: TomlStrategies,
}

#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub crypto_games: CryptoGamesConfig,
    pub freebitcoin: FreeBitcoInConfig,
    pub duck_dice: DuckDiceConfig,
}

impl TomlConfig {
    /// Validates the configuration
    pub fn validate(&self) -> Result<(), String> {
        let mut enabled_count = 0;

        if self.duck_dice.enabled {
            enabled_count += 1;
            if self.duck_dice.api_key.is_empty() {
                return Err("DuckDice API key cannot be empty".to_string());
            }
        }

        if self.crypto_games.enabled {
            enabled_count += 1;
            if self.crypto_games.api_key.is_empty() {
                return Err("CryptoGames API key cannot be empty".to_string());
            }
        }

        if self.freebitcoin.enabled {
            enabled_count += 1;
            if self.freebitcoin.btc_address.is_empty() {
                return Err("FreeBitco.in BTC address cannot be empty".to_string());
            }
            if self.freebitcoin.password.is_empty() {
                return Err("FreeBitco.in password cannot be empty".to_string());
            }
        }

        if enabled_count == 0 {
            return Err("At least one site must be enabled".to_string());
        }

        if enabled_count > 1 {
            return Err("Only one site can be enabled at a time".to_string());
        }

        Ok(())
    }
}

pub trait SiteConfig {
    fn with_api_key(self, _api_key: String) -> Self
    where
        Self: Sized,
    {
        self
    }

    fn with_username(self) -> Self
    where
        Self: Sized,
    {
        self
    }

    fn with_password(self) -> Self
    where
        Self: Sized,
    {
        self
    }

    fn with_currency(self, _currency: Currency) -> Self
    where
        Self: Sized,
    {
        self
    }

    fn with_strategy(self, _strategy: TomlStrategies) -> Self
    where
        Self: Sized,
    {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_no_site_enabled() {
        let config = TomlConfig {
            duck_dice: DuckDiceConfig {
                enabled: false,
                api_key: "test".to_string(),
                currency: Currency::BTC,
                strategy: TomlStrategies::None,
            },
            crypto_games: CryptoGamesConfig {
                enabled: false,
                api_key: "test".to_string(),
                currency: Currency::BTC,
                strategy: TomlStrategies::None,
            },
            freebitcoin: FreeBitcoInConfig {
                enabled: false,
                btc_address: "test".to_string(),
                password: "test".to_string(),
                strategy: TomlStrategies::None,
            },
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_empty_api_key() {
        let config = TomlConfig {
            duck_dice: DuckDiceConfig {
                enabled: true,
                api_key: "".to_string(),
                currency: Currency::BTC,
                strategy: TomlStrategies::None,
            },
            crypto_games: CryptoGamesConfig {
                enabled: false,
                api_key: "test".to_string(),
                currency: Currency::BTC,
                strategy: TomlStrategies::None,
            },
            freebitcoin: FreeBitcoInConfig {
                enabled: false,
                btc_address: "test".to_string(),
                password: "test".to_string(),
                strategy: TomlStrategies::None,
            },
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_valid() {
        let config = TomlConfig {
            duck_dice: DuckDiceConfig {
                enabled: true,
                api_key: "valid_key".to_string(),
                currency: Currency::BTC,
                strategy: TomlStrategies::None,
            },
            crypto_games: CryptoGamesConfig {
                enabled: false,
                api_key: "test".to_string(),
                currency: Currency::BTC,
                strategy: TomlStrategies::None,
            },
            freebitcoin: FreeBitcoInConfig {
                enabled: false,
                btc_address: "test".to_string(),
                password: "test".to_string(),
                strategy: TomlStrategies::None,
            },
        };

        assert!(config.validate().is_ok());
    }
}
