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

/// struct for passing parameters to the method [`fetch_incoming_phone_number_assigned_add_on_extension`]
#[derive(Clone, Debug)]
pub struct FetchIncomingPhoneNumberAssignedAddOnExtensionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resource to fetch.
    pub account_sid: String,
    /// The SID of the Phone Number to which the Add-on is assigned.
    pub resource_sid: String,
    /// The SID that uniquely identifies the assigned Add-on installation.
    pub assigned_add_on_sid: String,
    /// The Twilio-provided string that uniquely identifies the resource to fetch.
    pub sid: String
}

/// struct for passing parameters to the method [`list_incoming_phone_number_assigned_add_on_extension`]
#[derive(Clone, Debug)]
pub struct ListIncomingPhoneNumberAssignedAddOnExtensionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read.
    pub account_sid: String,
    /// The SID of the Phone Number to which the Add-on is assigned.
    pub resource_sid: String,
    /// The SID that uniquely identifies the assigned Add-on installation.
    pub assigned_add_on_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>
}


/// struct for typed errors of method [`fetch_incoming_phone_number_assigned_add_on_extension`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchIncomingPhoneNumberAssignedAddOnExtensionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_incoming_phone_number_assigned_add_on_extension`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncomingPhoneNumberAssignedAddOnExtensionError {
    UnknownValue(serde_json::Value),
}


/// Fetch an instance of an Extension for the Assigned Add-on.
pub async fn fetch_incoming_phone_number_assigned_add_on_extension(configuration: &configuration::Configuration, params: FetchIncomingPhoneNumberAssignedAddOnExtensionParams) -> Result<models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOnPeriodIncomingPhoneNumberAssignedAddOnExtension, Error<FetchIncomingPhoneNumberAssignedAddOnExtensionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let resource_sid = params.resource_sid;
    let assigned_add_on_sid = params.assigned_add_on_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), ResourceSid=crate::apis::urlencode(resource_sid), AssignedAddOnSid=crate::apis::urlencode(assigned_add_on_sid), Sid=crate::apis::urlencode(sid));
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
        let local_var_entity: Option<FetchIncomingPhoneNumberAssignedAddOnExtensionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of Extensions for the Assigned Add-on.
pub async fn list_incoming_phone_number_assigned_add_on_extension(configuration: &configuration::Configuration, params: ListIncomingPhoneNumberAssignedAddOnExtensionParams) -> Result<models::ListIncomingPhoneNumberAssignedAddOnExtensionResponse, Error<ListIncomingPhoneNumberAssignedAddOnExtensionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let resource_sid = params.resource_sid;
    let assigned_add_on_sid = params.assigned_add_on_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), ResourceSid=crate::apis::urlencode(resource_sid), AssignedAddOnSid=crate::apis::urlencode(assigned_add_on_sid));
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
        let local_var_entity: Option<ListIncomingPhoneNumberAssignedAddOnExtensionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

