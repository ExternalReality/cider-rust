/*
 * TeamCity REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2018.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `add_group`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `add_role_at_scope_to_group`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddRoleAtScopeToGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `add_role_to_group`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddRoleToGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_group`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGroupError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_all_groups`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllGroupsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_group_parent_groups`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGroupParentGroupsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_group_properties`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGroupPropertiesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_group_property`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGroupPropertyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_group_role_at_scope`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGroupRoleAtScopeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_group_roles`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGroupRolesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_user_group_of_group`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserGroupOfGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `remove_group_property`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveGroupPropertyError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `remove_role_at_scope_from_group`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveRoleAtScopeFromGroupError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_group_parent_groups`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGroupParentGroupsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_group_property`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGroupPropertyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_group_roles`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGroupRolesError {
    UnknownValue(serde_json::Value),
}


pub async fn add_group(configuration: &configuration::Configuration, fields: Option<&str>, body: Option<crate::models::Group>) -> Result<crate::models::Group, Error<AddGroupError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddGroupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn add_role_at_scope_to_group(configuration: &configuration::Configuration, group_locator: &str, role_id: &str, scope: &str) -> Result<crate::models::Role, Error<AddRoleAtScopeToGroupError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/roles/{roleId}/{scope}", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator), roleId=crate::apis::urlencode(role_id), scope=crate::apis::urlencode(scope));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddRoleAtScopeToGroupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn add_role_to_group(configuration: &configuration::Configuration, group_locator: &str, body: Option<crate::models::Role>) -> Result<crate::models::Role, Error<AddRoleToGroupError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/roles", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddRoleToGroupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_group(configuration: &configuration::Configuration, group_locator: &str) -> Result<(), Error<DeleteGroupError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteGroupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_all_groups(configuration: &configuration::Configuration, fields: Option<&str>) -> Result<crate::models::Groups, Error<GetAllGroupsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAllGroupsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_group_parent_groups(configuration: &configuration::Configuration, group_locator: &str, fields: Option<&str>) -> Result<crate::models::Groups, Error<GetGroupParentGroupsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/parent-groups", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetGroupParentGroupsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_group_properties(configuration: &configuration::Configuration, group_locator: &str, fields: Option<&str>) -> Result<crate::models::Properties, Error<GetGroupPropertiesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/properties", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetGroupPropertiesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_group_property(configuration: &configuration::Configuration, group_locator: &str, name: &str) -> Result<String, Error<GetGroupPropertyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/properties/{name}", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator), name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetGroupPropertyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_group_role_at_scope(configuration: &configuration::Configuration, group_locator: &str, role_id: &str, scope: &str) -> Result<crate::models::Role, Error<GetGroupRoleAtScopeError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/roles/{roleId}/{scope}", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator), roleId=crate::apis::urlencode(role_id), scope=crate::apis::urlencode(scope));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetGroupRoleAtScopeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_group_roles(configuration: &configuration::Configuration, group_locator: &str) -> Result<crate::models::Roles, Error<GetGroupRolesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/roles", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetGroupRolesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_user_group_of_group(configuration: &configuration::Configuration, group_locator: &str, fields: Option<&str>) -> Result<crate::models::Group, Error<GetUserGroupOfGroupError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserGroupOfGroupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn remove_group_property(configuration: &configuration::Configuration, group_locator: &str, name: &str) -> Result<(), Error<RemoveGroupPropertyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/properties/{name}", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator), name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RemoveGroupPropertyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn remove_role_at_scope_from_group(configuration: &configuration::Configuration, group_locator: &str, role_id: &str, scope: &str) -> Result<(), Error<RemoveRoleAtScopeFromGroupError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/roles/{roleId}/{scope}", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator), roleId=crate::apis::urlencode(role_id), scope=crate::apis::urlencode(scope));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RemoveRoleAtScopeFromGroupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn set_group_parent_groups(configuration: &configuration::Configuration, group_locator: &str, fields: Option<&str>, body: Option<crate::models::Groups>) -> Result<crate::models::Groups, Error<SetGroupParentGroupsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/parent-groups", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetGroupParentGroupsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn set_group_property(configuration: &configuration::Configuration, group_locator: &str, name: &str, body: Option<&str>) -> Result<String, Error<SetGroupPropertyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/properties/{name}", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator), name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetGroupPropertyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn set_group_roles(configuration: &configuration::Configuration, group_locator: &str, body: Option<crate::models::Roles>) -> Result<crate::models::Roles, Error<SetGroupRolesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/app/rest/userGroups/{groupLocator}/roles", configuration.base_path, groupLocator=crate::apis::urlencode(group_locator));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetGroupRolesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

