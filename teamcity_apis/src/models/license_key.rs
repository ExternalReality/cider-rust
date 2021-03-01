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
pub struct LicenseKey {
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(rename = "obsolete", skip_serializing_if = "Option::is_none")]
    pub obsolete: Option<bool>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "maintenanceEndDate", skip_serializing_if = "Option::is_none")]
    pub maintenance_end_date: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "servers", skip_serializing_if = "Option::is_none")]
    pub servers: Option<i32>,
    #[serde(rename = "agents", skip_serializing_if = "Option::is_none")]
    pub agents: Option<i32>,
    #[serde(rename = "unlimitedAgents", skip_serializing_if = "Option::is_none")]
    pub unlimited_agents: Option<bool>,
    #[serde(rename = "buildTypes", skip_serializing_if = "Option::is_none")]
    pub build_types: Option<i32>,
    #[serde(rename = "unlimitedBuildTypes", skip_serializing_if = "Option::is_none")]
    pub unlimited_build_types: Option<bool>,
    #[serde(rename = "errorDetails", skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "rawType", skip_serializing_if = "Option::is_none")]
    pub raw_type: Option<String>,
}

impl LicenseKey {
    pub fn new() -> LicenseKey {
        LicenseKey {
            valid: None,
            active: None,
            expired: None,
            obsolete: None,
            expiration_date: None,
            maintenance_end_date: None,
            _type: None,
            servers: None,
            agents: None,
            unlimited_agents: None,
            build_types: None,
            unlimited_build_types: None,
            error_details: None,
            key: None,
            raw_type: None,
        }
    }
}


