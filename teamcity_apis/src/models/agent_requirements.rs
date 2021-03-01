/*
 * TeamCity REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2018.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentRequirements {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "agent-requirement", skip_serializing_if = "Option::is_none")]
    pub agent_requirement: Option<Vec<crate::models::AgentRequirement>>,
}

impl AgentRequirements {
    pub fn new() -> AgentRequirements {
        AgentRequirements {
            count: None,
            agent_requirement: None,
        }
    }
}


