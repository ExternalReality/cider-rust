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
pub struct ProblemOccurrence {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "muted", skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(rename = "currentlyInvestigated", skip_serializing_if = "Option::is_none")]
    pub currently_investigated: Option<bool>,
    #[serde(rename = "currentlyMuted", skip_serializing_if = "Option::is_none")]
    pub currently_muted: Option<bool>,
    #[serde(rename = "logAnchor", skip_serializing_if = "Option::is_none")]
    pub log_anchor: Option<String>,
    #[serde(rename = "newFailure", skip_serializing_if = "Option::is_none")]
    pub new_failure: Option<bool>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "additionalData", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<String>,
    #[serde(rename = "problem", skip_serializing_if = "Option::is_none")]
    pub problem: Option<crate::models::Problem>,
    #[serde(rename = "mute", skip_serializing_if = "Option::is_none")]
    pub mute: Option<crate::models::Mute>,
    #[serde(rename = "build", skip_serializing_if = "Option::is_none")]
    pub build: Option<crate::models::Build>,
}

impl ProblemOccurrence {
    pub fn new() -> ProblemOccurrence {
        ProblemOccurrence {
            id: None,
            _type: None,
            identity: None,
            href: None,
            muted: None,
            currently_investigated: None,
            currently_muted: None,
            log_anchor: None,
            new_failure: None,
            details: None,
            additional_data: None,
            problem: None,
            mute: None,
            build: None,
        }
    }
}


