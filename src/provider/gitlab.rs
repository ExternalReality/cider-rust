use crate::model::project::Project;
use crate::provider::{Provider, Pipeline};
use async_trait::async_trait;

pub struct GitLab;

#[async_trait]
impl Provider for GitLab {
    async fn projects(&self) -> Vec<Project> {
        vec!()
    }

    async fn pipelines(&self) -> Vec<Pipeline> {
        vec!()
    }
}

impl GitLab {
    pub fn new() -> GitLab {
        GitLab
    }
}