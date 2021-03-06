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
pub struct StateField {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
    #[serde(rename = "inherited", skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
}

impl StateField {
    pub fn new() -> StateField {
        StateField {
            value: None,
            inherited: None,
        }
    }
}


