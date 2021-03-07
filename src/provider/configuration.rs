use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::error::Error;

use crate::provider::ProviderType;

#[derive(Deserialize, Serialize, Debug)]
pub struct ProviderConfig {
    pub provider: ProviderType,
    pub url: String,
    pub api_token: Option<String>,
}

pub fn load_provider_configs(providers: Vec<ProviderType>) -> Vec<ProviderConfig> {
    let mut cfgs: Vec<ProviderConfig> = vec![];
    for p in providers {
        cfgs.push(load_provider_config(p).unwrap());
    }
    cfgs
}

pub fn load_provider_config(provider: ProviderType) -> Result<ProviderConfig, Box<dyn Error>> {
    let mut filename = format!("./cider_config/{:?}.json", provider);
    filename = filename.to_lowercase();
    let file = File::open(filename).expect("failed opening file");
    let mut cfg: ProviderConfig = serde_yaml::from_reader(file)?;
    let token_var = format!("{:?}_token", provider).to_uppercase();
    let token = env::var(token_var)?;
    cfg.api_token = Some(token);
    cfg.provider = provider;
    Ok(cfg)
}
