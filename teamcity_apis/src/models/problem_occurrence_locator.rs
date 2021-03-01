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
pub struct ProblemOccurrenceLocator {
    #[serde(rename = "affectedProject", skip_serializing_if = "Option::is_none")]
    pub affected_project: Option<String>,
    #[serde(rename = "build", skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "currentlyFailing", skip_serializing_if = "Option::is_none")]
    pub currently_failing: Option<bool>,
    #[serde(rename = "currentlyInvestigated", skip_serializing_if = "Option::is_none")]
    pub currently_investigated: Option<bool>,
    #[serde(rename = "currentlyMuted", skip_serializing_if = "Option::is_none")]
    pub currently_muted: Option<bool>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(rename = "lookupLimit", skip_serializing_if = "Option::is_none")]
    pub lookup_limit: Option<i32>,
    #[serde(rename = "muted", skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(rename = "problem", skip_serializing_if = "Option::is_none")]
    pub problem: Option<String>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl ProblemOccurrenceLocator {
    pub fn new() -> ProblemOccurrenceLocator {
        ProblemOccurrenceLocator {
            affected_project: None,
            build: None,
            count: None,
            currently_failing: None,
            currently_investigated: None,
            currently_muted: None,
            identity: None,
            item: None,
            lookup_limit: None,
            muted: None,
            problem: None,
            start: None,
            _type: None,
        }
    }
}


