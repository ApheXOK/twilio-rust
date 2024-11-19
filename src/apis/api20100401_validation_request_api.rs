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

/// struct for passing parameters to the method [`create_validation_request`]
#[derive(Clone, Debug)]
pub struct CreateValidationRequestParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for the new caller ID resource.
    pub account_sid: String,
    /// The phone number to verify in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, which consists of a + followed by the country code and subscriber number.
    pub phone_number: String,
    /// A descriptive string that you create to describe the new caller ID resource. It can be up to 64 characters long. The default value is a formatted version of the phone number.
    pub friendly_name: Option<String>,
    /// The number of seconds to delay before initiating the verification call. Can be an integer between `0` and `60`, inclusive. The default is `0`.
    pub call_delay: Option<i32>,
    /// The digits to dial after connecting the verification call.
    pub extension: Option<String>,
    /// The URL we should call using the `status_callback_method` to send status information about the verification process to your application.
    pub status_callback: Option<String>,
    /// The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST`, and the default is `POST`.
    pub status_callback_method: Option<String>
}


/// struct for typed errors of method [`create_validation_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateValidationRequestError {
    UnknownValue(serde_json::Value),
}


/// 
pub async fn create_validation_request(configuration: &configuration::Configuration, params: CreateValidationRequestParams) -> Result<models::ApiPeriodV2010PeriodAccountPeriodValidationRequest, Error<CreateValidationRequestError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let phone_number = params.phone_number;
    let friendly_name = params.friendly_name;
    let call_delay = params.call_delay;
    let extension = params.extension;
    let status_callback = params.status_callback;
    let status_callback_method = params.status_callback_method;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("PhoneNumber", phone_number.to_string());
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = call_delay {
        local_var_form_params.insert("CallDelay", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = extension {
        local_var_form_params.insert("Extension", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback {
        local_var_form_params.insert("StatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback_method {
        local_var_form_params.insert("StatusCallbackMethod", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateValidationRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

