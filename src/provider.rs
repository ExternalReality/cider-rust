pub mod configuration;
pub mod teamcity;
pub mod gitlab;
use async_trait::async_trait;

use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::model::project::{Project, Pipeline};

#[derive(Serialize, Deserialize, EnumString, Debug)]
#[non_exhaustive]
pub enum ProviderType {
    TeamCity,
    GitLab,
}

#[async_trait]
pub trait Provider {
    async fn projects(&self) -> Vec<Project>;
    async fn pipelines(&self) -> Vec<Pipeline>;
}