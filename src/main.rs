#![recursion_limit = "256"]

pub mod config;
pub mod currency;
pub mod data;
pub mod dataset;
pub mod inference;
pub mod model;
pub mod sites;
pub mod strategies;
pub mod training;
pub mod util;

use burn::{
    backend::{wgpu::WgpuDevice, Vulkan},
    prelude::*,
    record::{CompactRecorder, Recorder},
};
use colored::Colorize;
use log::{error, info, warn};
use model::Model;
use training::TrainingConfig;

use crate::config::SiteConfig;
#[allow(unused_imports)]
use crate::sites::{crypto_games::CryptoGames, duck_dice::DuckDiceIo, free_bitco_in::FreeBitcoIn};
use crate::sites::{BetError, BetResult, Site};
use crate::{config::TomlConfig, model::ModelConfig};

struct Game<B: Backend> {
    confidence: f32,
    site: Box<dyn Site>,
    model: Model<B>,
    device: B::Device,
    prediction: f32,
    initialized: bool,
}

impl<B: Backend> Game<B> {
    async fn bet(&mut self) -> Result<(), BetError> {
        if !self.initialized {
            B::seed(42);
            self.initialized = true;
        }
        let bet_result = match self.site.do_bet(self.prediction, self.confidence).await {
            Ok(res) => res,
            Err(err) => match err {
                BetError::EmptyReply => return Ok(()),
                _ => return Err(err),
            },
        };

        if bet_result.result {
            self.site.on_win(&bet_result);
            self.print_res(&bet_result, true);
        } else {
            self.site.on_lose(&bet_result);
            self.print_res(&bet_result, false);
        }

        let history = self.site.get_history();
        let history_size = self.site.get_history_size();
        // Get server seed hash next roll and convert it to a tensor of shape (-1, 256).
        if history.len() >= history_size {
            let inputs_hash = history
                .iter()
                .flat_map(|itm| {
                    let mut vals = util::hex_string_to_binary_vec::<B>(&itm.hash_next_roll);
                    vals.resize(util::HASH_NEXT_ROLL_SIZE, 0f32.elem::<B::FloatElem>());

                    vals.append(&mut util::hex_string_to_binary_vec::<B>(
                        &itm.hash_previous_roll,
                    ));
                    vals.resize(util::HASH_PREVIOUS_ROLL_SIZE, 0f32.elem::<B::FloatElem>());

                    vals.append(&mut util::hex_string_to_binary_vec::<B>(&itm.client_seed));
                    vals.resize(util::CLIENT_SEED_SIZE, 0f32.elem::<B::FloatElem>());

                    vals.append(
                        &mut (0..32)
                            .map(|i| ((itm.nonce >> i) & 1).elem::<B::FloatElem>())
                            .collect::<Vec<B::FloatElem>>(),
                    );
                    vals.resize(util::FINAL_FEATURE_SIZE, 0f32.elem::<B::FloatElem>());

                    vals
                })
                .collect::<Vec<B::FloatElem>>();

            let hash_data = TensorData::new(
                inputs_hash,
                [
                    history.len() / history_size,
                    history_size,
                    4,
                    util::HASH_NEXT_ROLL_SIZE,
                ],
            );
            let hash_data: Tensor<B, 4> =
                Tensor::from(hash_data.convert::<B::FloatElem>()).to_device(&self.device);

            let output = self.model.forward(data::BetBatch {
                inputs: hash_data,
                targets: Tensor::zeros(Shape::new([1, 1]), &self.device),
            });
            let predicted_output = output
                .clone()
                .argmax(1)
                .into_data()
                .to_vec::<i32>()
                .unwrap();
            let predicted_output = predicted_output[0];
            let confidence = output.clone().into_data().to_vec::<f32>().unwrap()
                [predicted_output as usize]
                * 100.;
            // let predicted = (predicted_output[0] + 1.) * 10000. / 2.;
            // let predicted = (((predicted - 4500.) / (5500. - 4500.)) * (10000. - 0.)) + 0.;

            self.confidence = confidence;
            self.prediction = predicted_output as f32 * 100.;
        }

        Ok(())
    }

    fn print_res(&self, bet_result: &BetResult, win: bool) {
        let profit_str = &format!("Profit: {:.8}", self.site.get_profit());
        let profit_str = if self.site.get_profit() > 0. {
            profit_str.green()
        } else {
            profit_str.red()
        };

        let golden_roll = if bet_result.number > 9900 || bet_result.number < 100 {
            (&format!("{: <5}", bet_result.number)).yellow()
        } else {
            format!("{: <5}", bet_result.number).normal()
        };

        let output_str = &format!(
            "#{: >6} || Balance: {:0>.8} || Roll: {: <5} || Multiplier: {: <6.2} || Wagered: {:.8} || Predicted: {: <5.0} || Confidence: {: <2.2} || {}",
            self.site.get_rolls(),
            self.site.get_balance(),
            golden_roll,
            self.site.get_current_multiplier(),
            self.site.get_current_bet(),
            self.prediction,
            self.confidence,
            profit_str,
        );
        let output_str = if win {
            output_str.green()
        } else {
            output_str.red()
        };

        println!("{output_str}");
    }
}

#[tokio::main]
async fn main() -> Result<(), BetError> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting PredictiveRolls application");

    // Read configuration
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    info!("Loading configuration from: {}", config_path);

    let config_contents = tokio::fs::read_to_string(&config_path).await.map_err(|e| {
        error!("Failed to read config file {}: {}", config_path, e);
        BetError::Failed
    })?;

    let game_config: TomlConfig = toml::from_str(&config_contents).map_err(|e| {
        error!("Failed to parse config.toml: {}", e);
        BetError::ConfigError(format!("Parse error: {}", e))
    })?;

    // Validate configuration
    game_config.validate().map_err(|e| {
        error!("Configuration validation failed: {}", e);
        BetError::ConfigError(e)
    })?;

    info!("Configuration validated successfully");

    // Initialize the configured site
    let site: Box<dyn Site> = if game_config.duck_dice.enabled {
        info!("Using DuckDice site");
        Box::new(
            DuckDiceIo::default()
                .with_api_key(game_config.duck_dice.api_key.clone())
                .with_currency(game_config.duck_dice.currency.clone())
                .with_strategy(game_config.duck_dice.strategy),
        )
    } else {
        warn!("No site enabled in configuration");
        return Err(BetError::Failed);
    };

    type MyBackend = Vulkan<f32, i32>;

    info!("Initializing GPU device");
    let device = WgpuDevice::default();

    // Get model artifact directory from environment or use default
    let artifact_dir = std::env::var("MODEL_DIR").unwrap_or_else(|_| "./artifacts".to_string());
    info!("Loading model from: {}", artifact_dir);

    let _config = TrainingConfig::load(format!("{artifact_dir}/config.json")).map_err(|e| {
        error!("Failed to load model config: {}", e);
        BetError::Failed
    })?;

    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .map_err(|e| {
            error!("Failed to load trained model: {}", e);
            BetError::Failed
        })?;

    info!("Model loaded successfully");
    let model = ModelConfig::new().init(&device).load_record(record);

    let mut game = Game::<MyBackend> {
        confidence: 0.,
        site,
        model,
        device,
        prediction: 0.,
        initialized: false,
    };

    info!("Logging into site");
    game.site.login().await?;
    info!("Login successful, starting betting loop");

    loop {
        match game.bet().await {
            Ok(_) => {}
            Err(e) => {
                error!("Bet failed: {:?}", e);
                return Err(e);
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
