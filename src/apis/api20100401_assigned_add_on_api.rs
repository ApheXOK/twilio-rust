/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_incoming_phone_number_assigned_add_on`]
#[derive(Clone, Debug)]
pub struct CreateIncomingPhoneNumberAssignedAddOnParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource.
    pub account_sid: String,
    /// The SID of the Phone Number to assign the Add-on.
    pub resource_sid: String,
    /// The SID that identifies the Add-on installation.
    pub installed_add_on_sid: String
}

/// struct for passing parameters to the method [`delete_incoming_phone_number_assigned_add_on`]
#[derive(Clone, Debug)]
pub struct DeleteIncomingPhoneNumberAssignedAddOnParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to delete.
    pub account_sid: String,
    /// The SID of the Phone Number to which the Add-on is assigned.
    pub resource_sid: String,
    /// The Twilio-provided string that uniquely identifies the resource to delete.
    pub sid: String
}

/// struct for passing parameters to the method [`fetch_incoming_phone_number_assigned_add_on`]
#[derive(Clone, Debug)]
pub struct FetchIncomingPhoneNumberAssignedAddOnParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resource to fetch.
    pub account_sid: String,
    /// The SID of the Phone Number to which the Add-on is assigned.
    pub resource_sid: String,
    /// The Twilio-provided string that uniquely identifies the resource to fetch.
    pub sid: String
}

/// struct for passing parameters to the method [`list_incoming_phone_number_assigned_add_on`]
#[derive(Clone, Debug)]
pub struct ListIncomingPhoneNumberAssignedAddOnParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read.
    pub account_sid: String,
    /// The SID of the Phone Number to which the Add-on is assigned.
    pub resource_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>
}


/// struct for typed errors of method [`create_incoming_phone_number_assigned_add_on`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncomingPhoneNumberAssignedAddOnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_incoming_phone_number_assigned_add_on`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncomingPhoneNumberAssignedAddOnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_incoming_phone_number_assigned_add_on`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchIncomingPhoneNumberAssignedAddOnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_incoming_phone_number_assigned_add_on`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncomingPhoneNumberAssignedAddOnError {
    UnknownValue(serde_json::Value),
}


/// Assign an Add-on installation to the Number specified.
pub async fn create_incoming_phone_number_assigned_add_on(configuration: &configuration::Configuration, params: CreateIncomingPhoneNumberAssignedAddOnParams) -> Result<models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn, Error<CreateIncomingPhoneNumberAssignedAddOnError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let resource_sid = params.resource_sid;
    let installed_add_on_sid = params.installed_add_on_sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), ResourceSid=crate::apis::urlencode(resource_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("InstalledAddOnSid", installed_add_on_sid.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateIncomingPhoneNumberAssignedAddOnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove the assignment of an Add-on installation from the Number specified.
pub async fn delete_incoming_phone_number_assigned_add_on(configuration: &configuration::Configuration, params: DeleteIncomingPhoneNumberAssignedAddOnParams) -> Result<(), Error<DeleteIncomingPhoneNumberAssignedAddOnError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let resource_sid = params.resource_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), ResourceSid=crate::apis::urlencode(resource_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteIncomingPhoneNumberAssignedAddOnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch an instance of an Add-on installation currently assigned to this Number.
pub async fn fetch_incoming_phone_number_assigned_add_on(configuration: &configuration::Configuration, params: FetchIncomingPhoneNumberAssignedAddOnParams) -> Result<models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn, Error<FetchIncomingPhoneNumberAssignedAddOnError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let resource_sid = params.resource_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), ResourceSid=crate::apis::urlencode(resource_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchIncomingPhoneNumberAssignedAddOnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of Add-on installations currently assigned to this Number.
pub async fn list_incoming_phone_number_assigned_add_on(configuration: &configuration::Configuration, params: ListIncomingPhoneNumberAssignedAddOnParams) -> Result<models::ListIncomingPhoneNumberAssignedAddOnResponse, Error<ListIncomingPhoneNumberAssignedAddOnError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let resource_sid = params.resource_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), ResourceSid=crate::apis::urlencode(resource_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("Page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder.query(&[("PageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListIncomingPhoneNumberAssignedAddOnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

