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
pub struct MetaData {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<crate::models::Entries>,
}

impl MetaData {
    pub fn new() -> MetaData {
        MetaData {
            id: None,
            entries: None,
        }
    }
}


