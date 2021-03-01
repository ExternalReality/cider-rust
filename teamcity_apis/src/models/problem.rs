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
pub struct Problem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mutes", skip_serializing_if = "Option::is_none")]
    pub mutes: Option<crate::models::Mutes>,
    #[serde(rename = "investigations", skip_serializing_if = "Option::is_none")]
    pub investigations: Option<crate::models::Investigations>,
    #[serde(rename = "problemOccurrences", skip_serializing_if = "Option::is_none")]
    pub problem_occurrences: Option<crate::models::ProblemOccurrences>,
    #[serde(rename = "locator", skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
}

impl Problem {
    pub fn new() -> Problem {
        Problem {
            id: None,
            _type: None,
            identity: None,
            href: None,
            description: None,
            mutes: None,
            investigations: None,
            problem_occurrences: None,
            locator: None,
        }
    }
}


