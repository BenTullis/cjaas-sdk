/*
 * Customer Journey as a Service (CJaaS)
 *
 * Something amazing, something special - the Customer Journey as a Service (CJaaS) is a core data layer to enable Journeys across products built upon serverless multi-cloud architecture, to be available as a SaaS service for applications inside and outside of Cisco. [**Cisco Experimental - Not For Production Use**]
 *
 * The version of the OpenAPI document: 0.5.0
 * Contact: cjaas-earlyaccess@cisco.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `clear_tape`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClearTapeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `clear_tape_by_person`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClearTapeByPersonError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `data_sink`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataSinkError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `data_sink_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataSinkGetError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `identities`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdentitiesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `identities_alias`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdentitiesAliasError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `identities_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdentitiesDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `journeys`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JourneysError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `journeys_by_person`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JourneysByPersonError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `keys`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeysError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `keys_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeysDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `keys_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeysListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `online_orchestration_trigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OnlineOrchestrationTriggerError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_builder`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileBuilderError {
    Status400(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `real_time_sse`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealTimeSseError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `real_time_sse_person`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealTimeSsePersonError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `settings`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SettingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_settings`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSettingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `webex_walkin_sse`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebexWalkinSseError {
    UnknownValue(serde_json::Value),
}


/// Delete All Events Collected for Org
pub async fn clear_tape(configuration: &configuration::Configuration, authorization: &str) -> Result<String, Error<ClearTapeError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/ClearTape", configuration.base_path);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ClearTapeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete Events on Specific Person
pub async fn clear_tape_by_person(configuration: &configuration::Configuration, authorization: &str, person: &str) -> Result<String, Error<ClearTapeByPersonError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/ClearTape/{person}", configuration.base_path, person=crate::apis::urlencode(person));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ClearTapeByPersonError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Data Ingestion is based on Cloud Events specification for describing event data in a common way. Data Sink accepts data in the form of POST or GET with support for both Header based authorization and as-well via Query string
pub async fn data_sink(configuration: &configuration::Configuration, authorization: &str, body: crate::models::CloudEvent) -> Result<(), Error<DataSinkError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/DataSink", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DataSinkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Data Ingestion is based on Cloud Events specification for describing event data in a common way. Data Sink accepts data in the form of POST or GET with support for both Header based authorization and as-well via Query string
pub async fn data_sink_get(configuration: &configuration::Configuration, sig: &str, data: &str) -> Result<(), Error<DataSinkGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/DataSink", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("sig", &sig.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("data", &data.to_string())]);
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
        let local_var_entity: Option<DataSinkGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Every event in the Journey has an Identity of the originating individual, different channels can have their own unique Identities, API consumers can tie multiple duplicate Identities together to a unique single individual nondestructively (i.e soft merge) without modifying the Tape. Once aliased, all duplicates are treated as a single Identity for any purpose (Query or View)
pub async fn identities(configuration: &configuration::Configuration, authorization: &str, id: &str) -> Result<String, Error<IdentitiesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Identities", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("id", &id.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IdentitiesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Every event in the Journey has an Identity of the originating individual, different channels can have their own unique Identities, API consumers can tie multiple duplicate Identities together to a unique single individual nondestructively (i.e soft merge) without modifying the Tape. Once aliased, all duplicates are treated as a single Identity for any purpose (Query or View)
pub async fn identities_alias(configuration: &configuration::Configuration, authorization: &str, id: &str, alias: Option<&str>) -> Result<String, Error<IdentitiesAliasError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Identities/alias", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("id", &id.to_string())]);
    if let Some(ref local_var_str) = alias {
        local_var_req_builder = local_var_req_builder.query(&[("alias", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IdentitiesAliasError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove a soft merge
pub async fn identities_delete(configuration: &configuration::Configuration, authorization: &str, id: &str) -> Result<String, Error<IdentitiesDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Identities", configuration.base_path);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("id", &id.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IdentitiesDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Tape is a append-only, immutable data ledger that can be queried to retrieve snapshot of latest events that moment in time or historically to play-back events as they occurred to understand or analyze Journeys using ML/AI models
pub async fn journeys(configuration: &configuration::Configuration, authorization: &str, filter: Option<&str>, top: Option<i32>) -> Result<Vec<crate::models::CloudEvent>, Error<JourneysError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Journey", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder = local_var_req_builder.query(&[("$filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = top {
        local_var_req_builder = local_var_req_builder.query(&[("$top", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<JourneysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Tape is a append-only, immutable data ledger that can be queried to retrieve snapshot of latest events that moment in time or historically to play-back events as they occurred to understand or analyze Journeys using ML/AI models
pub async fn journeys_by_person(configuration: &configuration::Configuration, authorization: &str, person: &str, filter: Option<&str>, top: Option<i32>) -> Result<Vec<crate::models::CloudEvent>, Error<JourneysByPersonError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Journey/{person}", configuration.base_path, person=crate::apis::urlencode(person));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder = local_var_req_builder.query(&[("$filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = top {
        local_var_req_builder = local_var_req_builder.query(&[("$top", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<JourneysByPersonError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// With Shared Access Signatures API consumers get choice of granular control on how to access API(paths), which resources to scope(ex:datasink), with what permissions the request needs(read / write), from which  source (ex: website) and how long(10s to 10 years) among many other parameters.
pub async fn keys(configuration: &configuration::Configuration, authorization: &str, operation: &str, id: &str) -> Result<String, Error<KeysError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Keys/{operation}/{id}", configuration.base_path, operation=crate::apis::urlencode(operation), id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<KeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// With Shared Access Signatures API consumers get choice of granular control on how to access API(paths), which resources to scope(ex:datasink), with what permissions the request needs(read / write), from which  source (ex: website) and how long(10s to 10 years) among many other parameters.
pub async fn keys_delete(configuration: &configuration::Configuration, authorization: &str, id: &str) -> Result<String, Error<KeysDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Keys/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<KeysDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// With Shared Access Signatures API consumers get choice of granular control on how to access API(paths), which resources to scope(ex:datasink), with what permissions the request needs(read / write), from which  source (ex: website) and how long(10s to 10 years) among many other parameters.
pub async fn keys_list(configuration: &configuration::Configuration, authorization: &str) -> Result<String, Error<KeysListError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Keys/list", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<KeysListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Trigger a Online Orchestration such as Webex Walkin or Display Offer Or Chat Bot to modify your Customer's Journey Midway
pub async fn online_orchestration_trigger(configuration: &configuration::Configuration, person: &str, body: crate::models::OnlineOrchestration, sig: Option<&str>, authorization: Option<&str>) -> Result<String, Error<OnlineOrchestrationTriggerError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Orchestration/Trigger/{person}", configuration.base_path, person=crate::apis::urlencode(person));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sig {
        local_var_req_builder = local_var_req_builder.query(&[("sig", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OnlineOrchestrationTriggerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The profile view is created based on a view template that is provided as input.    **Sample request**: *GET /profileview?customer=123XX*    {    &nbsp;&nbsp;&nbsp;&nbsp;\"Name\": \"Test Template 2\",    &nbsp;&nbsp;&nbsp;&nbsp;\"DatapointCount\": 7,    &nbsp;&nbsp;&nbsp;&nbsp;\"Attributes\": [{    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;\"Version\": \"0.1\",    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;\"Event\": \"Add to Cart\",    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;\"Metadata\": \"sku\",    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;\"Limit\": 3,    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;\"DisplayName\": \"Items added\",    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;\"AggregationMode\": \"Value\"    &nbsp;&nbsp;&nbsp;&nbsp;}]    }
pub async fn profile_builder(configuration: &configuration::Configuration, authorization: &str, person_id: &str, body: crate::models::ProfileViewBuilderTemplate) -> Result<crate::models::ProfileViewQueryResponse, Error<ProfileBuilderError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Profileview", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("PersonId", &person_id.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ProfileBuilderError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Optionally accepts $filter to slice/dice further (ex: type eq 'Add to Cart')
pub async fn real_time_sse(configuration: &configuration::Configuration, sig: Option<&str>, authorization: Option<&str>) -> Result<String, Error<RealTimeSseError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Real-time", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sig {
        local_var_req_builder = local_var_req_builder.query(&[("sig", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RealTimeSseError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Optionally accepts $filter to slice/dice further (ex: type eq 'Add to Cart')
pub async fn real_time_sse_person(configuration: &configuration::Configuration, person: &str, sig: Option<&str>, authorization: Option<&str>) -> Result<String, Error<RealTimeSsePersonError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Real-time/{person}", configuration.base_path, person=crate::apis::urlencode(person));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sig {
        local_var_req_builder = local_var_req_builder.query(&[("sig", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RealTimeSsePersonError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update, get data retention, destinations and more
pub async fn settings(configuration: &configuration::Configuration, authorization: &str) -> Result<crate::models::AccountSettings, Error<SettingsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Settings", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update, get data retention, destinations and more
pub async fn update_settings(configuration: &configuration::Configuration, authorization: &str, body: crate::models::AccountSettings) -> Result<crate::models::AccountSettings, Error<UpdateSettingsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Settings", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateSettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Major browsers offer built-in class EventSource, with the EventSource object API consumers can automatically establishes a persistent connection and transparently allow for automatic reconnects with tunable retry timeout & Message ids to resume events.
pub async fn webex_walkin_sse(configuration: &configuration::Configuration, person: &str, sig: Option<&str>, authorization: Option<&str>) -> Result<String, Error<WebexWalkinSseError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/Walkin/{person}", configuration.base_path, person=crate::apis::urlencode(person));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sig {
        local_var_req_builder = local_var_req_builder.query(&[("sig", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WebexWalkinSseError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

