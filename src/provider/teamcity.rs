use crate::provider::configuration::load_provider_config;
use crate::provider::{Provider, ProviderType};
use async_trait::async_trait;
use openapi::apis::configuration::Configuration;
use openapi::apis::project_api;

pub struct TeamCity;

#[async_trait]
impl Provider for TeamCity {
    fn new() -> TeamCity {
        TeamCity
    }

    async fn projects(&self) -> () {
        let cfg = load_provider_config(ProviderType::TeamCity);
        let mut c: Configuration = Configuration::new();
        c.bearer_access_token = cfg.api_token;
        let _res = project_api::get_all_projects(&c, None, None);
        ()
    }
}
