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
pub struct BuildTriggeringOptions {
    #[serde(rename = "cleanSources", skip_serializing_if = "Option::is_none")]
    pub clean_sources: Option<bool>,
    #[serde(rename = "cleanSourcesInAllDependencies", skip_serializing_if = "Option::is_none")]
    pub clean_sources_in_all_dependencies: Option<bool>,
    #[serde(rename = "rebuildAllDependencies", skip_serializing_if = "Option::is_none")]
    pub rebuild_all_dependencies: Option<bool>,
    #[serde(rename = "rebuildFailedOrIncompleteDependencies", skip_serializing_if = "Option::is_none")]
    pub rebuild_failed_or_incomplete_dependencies: Option<bool>,
    #[serde(rename = "queueAtTop", skip_serializing_if = "Option::is_none")]
    pub queue_at_top: Option<bool>,
    #[serde(rename = "freezeSettings", skip_serializing_if = "Option::is_none")]
    pub freeze_settings: Option<bool>,
    #[serde(rename = "tagDependencies", skip_serializing_if = "Option::is_none")]
    pub tag_dependencies: Option<bool>,
    #[serde(rename = "rebuildDependencies", skip_serializing_if = "Option::is_none")]
    pub rebuild_dependencies: Option<crate::models::BuildTypes>,
}

impl BuildTriggeringOptions {
    pub fn new() -> BuildTriggeringOptions {
        BuildTriggeringOptions {
            clean_sources: None,
            clean_sources_in_all_dependencies: None,
            rebuild_all_dependencies: None,
            rebuild_failed_or_incomplete_dependencies: None,
            queue_at_top: None,
            freeze_settings: None,
            tag_dependencies: None,
            rebuild_dependencies: None,
        }
    }
}


