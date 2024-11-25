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

/// struct for passing parameters to the method [`fetch_available_phone_number_country`]
#[derive(Clone, Debug)]
pub struct FetchAvailablePhoneNumberCountryParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the available phone number Country resource.
    pub account_sid: String,
    /// The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country to fetch available phone number information about.
    pub country_code: String
}

/// struct for passing parameters to the method [`list_available_phone_number_country`]
#[derive(Clone, Debug)]
pub struct ListAvailablePhoneNumberCountryParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the available phone number Country resources.
    pub account_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>
}


/// struct for typed errors of method [`fetch_available_phone_number_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchAvailablePhoneNumberCountryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_available_phone_number_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAvailablePhoneNumberCountryError {
    UnknownValue(serde_json::Value),
}


/// 
pub async fn fetch_available_phone_number_country(configuration: &configuration::Configuration, params: FetchAvailablePhoneNumberCountryParams) -> Result<models::ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry, Error<FetchAvailablePhoneNumberCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let country_code = params.country_code;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CountryCode=crate::apis::urlencode(country_code));
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
        let local_var_entity: Option<FetchAvailablePhoneNumberCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 
pub async fn list_available_phone_number_country(configuration: &configuration::Configuration, params: ListAvailablePhoneNumberCountryParams) -> Result<models::ListAvailablePhoneNumberCountryResponse, Error<ListAvailablePhoneNumberCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
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
        let local_var_entity: Option<ListAvailablePhoneNumberCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

