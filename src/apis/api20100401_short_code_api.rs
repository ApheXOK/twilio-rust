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

/// struct for passing parameters to the method [`fetch_short_code`]
#[derive(Clone, Debug)]
pub struct FetchShortCodeParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource(s) to fetch.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the ShortCode resource to fetch
    pub sid: String
}

/// struct for passing parameters to the method [`list_short_code`]
#[derive(Clone, Debug)]
pub struct ListShortCodeParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource(s) to read.
    pub account_sid: String,
    /// The string that identifies the ShortCode resources to read.
    pub friendly_name: Option<String>,
    /// Only show the ShortCode resources that match this pattern. You can specify partial numbers and use '*' as a wildcard for any digit.
    pub short_code: Option<String>,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>
}

/// struct for passing parameters to the method [`update_short_code`]
#[derive(Clone, Debug)]
pub struct UpdateShortCodeParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource(s) to update.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the ShortCode resource to update
    pub sid: String,
    /// A descriptive string that you created to describe this resource. It can be up to 64 characters long. By default, the `FriendlyName` is the short code.
    pub friendly_name: Option<String>,
    /// The API version to use to start a new TwiML session. Can be: `2010-04-01` or `2008-08-01`.
    pub api_version: Option<String>,
    /// The URL we should call when receiving an incoming SMS message to this short code.
    pub sms_url: Option<String>,
    /// The HTTP method we should use when calling the `sms_url`. Can be: `GET` or `POST`.
    pub sms_method: Option<String>,
    /// The URL that we should call if an error occurs while retrieving or executing the TwiML from `sms_url`.
    pub sms_fallback_url: Option<String>,
    /// The HTTP method that we should use to call the `sms_fallback_url`. Can be: `GET` or `POST`.
    pub sms_fallback_method: Option<String>
}


/// struct for typed errors of method [`fetch_short_code`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchShortCodeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_short_code`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListShortCodeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_short_code`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateShortCodeError {
    UnknownValue(serde_json::Value),
}


/// Fetch an instance of a short code
pub async fn fetch_short_code(configuration: &configuration::Configuration, params: FetchShortCodeParams) -> Result<models::ApiPeriodV2010PeriodAccountPeriodShortCode, Error<FetchShortCodeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
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
        let local_var_entity: Option<FetchShortCodeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of short-codes belonging to the account used to make the request
pub async fn list_short_code(configuration: &configuration::Configuration, params: ListShortCodeParams) -> Result<models::ListShortCodeResponse, Error<ListShortCodeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let friendly_name = params.friendly_name;
    let short_code = params.short_code;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = friendly_name {
        local_var_req_builder = local_var_req_builder.query(&[("FriendlyName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = short_code {
        local_var_req_builder = local_var_req_builder.query(&[("ShortCode", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<ListShortCodeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a short code with the following parameters
pub async fn update_short_code(configuration: &configuration::Configuration, params: UpdateShortCodeParams) -> Result<models::ApiPeriodV2010PeriodAccountPeriodShortCode, Error<UpdateShortCodeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;
    let friendly_name = params.friendly_name;
    let api_version = params.api_version;
    let sms_url = params.sms_url;
    let sms_method = params.sms_method;
    let sms_fallback_url = params.sms_fallback_url;
    let sms_fallback_method = params.sms_fallback_method;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = api_version {
        local_var_form_params.insert("ApiVersion", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_url {
        local_var_form_params.insert("SmsUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_method {
        local_var_form_params.insert("SmsMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_url {
        local_var_form_params.insert("SmsFallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_method {
        local_var_form_params.insert("SmsFallbackMethod", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateShortCodeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

