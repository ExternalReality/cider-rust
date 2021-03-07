pub mod configuration;
pub mod teamcity;
use async_trait::async_trait;

use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Serialize, Deserialize, EnumString, Debug)]
pub enum ProviderType {
    TeamCity,
    GitLab,
}

#[async_trait]
pub trait Provider {
    fn new() -> Self;
    async fn projects(&self) -> ();
}
