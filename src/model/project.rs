use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Pipeline {
    pub name: String,
    pub project: String,
    pub uuid: String,
    pub provider: String,
}
