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

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};

/// struct for passing parameters to the method [`create_realtime_transcription`]
#[derive(Clone, Debug)]
pub struct CreateRealtimeTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Transcription resource.
    pub account_sid: String,
    /// The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Transcription resource is associated with.
    pub call_sid: String,
    /// The user-specified name of this Transcription, if one was given when the Transcription was created. This may be used to stop the Transcription.
    pub name: Option<String>,
    pub track: Option<String>,
    /// Absolute URL of the status callback.
    pub status_callback_url: Option<String>,
    /// The http method for the status_callback (one of GET, POST).
    pub status_callback_method: Option<String>,
    /// Friendly name given to the Inbound Track
    pub inbound_track_label: Option<String>,
    /// Friendly name given to the Outbound Track
    pub outbound_track_label: Option<String>,
    /// Indicates if partial results are going to be sent to the customer
    pub partial_results: Option<bool>,
    /// Language code used by the transcription engine, specified in [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) format
    pub language_code: Option<String>,
    /// Definition of the transcription engine to be used, among those supported by Twilio
    pub transcription_engine: Option<String>,
    /// indicates if the server will attempt to filter out profanities, replacing all but the initial character in each filtered word with asterisks
    pub profanity_filter: Option<bool>,
    /// Recognition dtos used by the transcription engine, among those supported by the provider
    pub speech_model: Option<String>,
    /// A Phrase contains words and phrase \\\"hints\\\" so that the speech recognition engine is more likely to recognize them.
    pub hints: Option<String>,
    /// The provider will add punctuation to recognition result
    pub enable_automatic_punctuation: Option<bool>,
}

/// struct for passing parameters to the method [`delete_recording_transcription`]
#[derive(Clone, Debug)]
pub struct DeleteRecordingTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete.
    pub account_sid: String,
    /// The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to delete.
    pub recording_sid: String,
    /// The Twilio-provided string that uniquely identifies the Transcription resource to delete.
    pub sid: String,
}

/// struct for passing parameters to the method [`delete_transcription`]
#[derive(Clone, Debug)]
pub struct DeleteTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Transcription resource to delete.
    pub sid: String,
}

/// struct for passing parameters to the method [`fetch_recording_transcription`]
#[derive(Clone, Debug)]
pub struct FetchRecordingTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch.
    pub account_sid: String,
    /// The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to fetch.
    pub recording_sid: String,
    /// The Twilio-provided string that uniquely identifies the Transcription resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`fetch_transcription`]
#[derive(Clone, Debug)]
pub struct FetchTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Transcription resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`list_recording_transcription`]
#[derive(Clone, Debug)]
pub struct ListRecordingTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read.
    pub account_sid: String,
    /// The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcriptions to read.
    pub recording_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for passing parameters to the method [`list_transcription`]
#[derive(Clone, Debug)]
pub struct ListTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read.
    pub account_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for passing parameters to the method [`update_realtime_transcription`]
#[derive(Clone, Debug)]
pub struct UpdateRealtimeTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Transcription resource.
    pub account_sid: String,
    /// The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Transcription resource is associated with.
    pub call_sid: String,
    /// The SID of the Transcription resource, or the `name` used when creating the resource
    pub sid: String,
    pub status: String,
}

/// struct for typed errors of method [`create_realtime_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRealtimeTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_recording_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRecordingTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_recording_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchRecordingTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_recording_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRecordingTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_realtime_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRealtimeTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// Create a Transcription
pub async fn create_realtime_transcription(
    configuration: &configuration::Configuration,
    params: CreateRealtimeTranscriptionParams,
) -> Result<
    models::ApiPeriodV2010PeriodAccountPeriodCallPeriodRealtimeTranscription,
    Error<CreateRealtimeTranscriptionError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let call_sid = params.call_sid;
    let name = params.name;
    let track = params.track;
    let status_callback_url = params.status_callback_url;
    let status_callback_method = params.status_callback_method;
    let inbound_track_label = params.inbound_track_label;
    let outbound_track_label = params.outbound_track_label;
    let partial_results = params.partial_results;
    let language_code = params.language_code;
    let transcription_engine = params.transcription_engine;
    let profanity_filter = params.profanity_filter;
    let speech_model = params.speech_model;
    let hints = params.hints;
    let enable_automatic_punctuation = params.enable_automatic_punctuation;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Transcriptions.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CallSid=crate::apis::urlencode(call_sid));
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("Name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = track {
        local_var_form_params
            .insert("Track", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback_url {
        local_var_form_params
            .insert("StatusCallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback_method {
        local_var_form_params
            .insert("StatusCallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = inbound_track_label {
        local_var_form_params
            .insert("InboundTrackLabel", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = outbound_track_label {
        local_var_form_params
            .insert("OutboundTrackLabel", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = partial_results {
        local_var_form_params
            .insert("PartialResults", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = language_code {
        local_var_form_params
            .insert("LanguageCode", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = transcription_engine {
        local_var_form_params
            .insert("TranscriptionEngine", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = profanity_filter {
        local_var_form_params
            .insert("ProfanityFilter", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = speech_model {
        local_var_form_params
            .insert("SpeechModel", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = hints {
        local_var_form_params
            .insert("Hints", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = enable_automatic_punctuation {
        local_var_form_params.insert(
            "EnableAutomaticPunctuation",
            local_var_param_value.to_string(),
        );
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateRealtimeTranscriptionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn delete_recording_transcription(
    configuration: &configuration::Configuration,
    params: DeleteRecordingTranscriptionParams,
) -> Result<(), Error<DeleteRecordingTranscriptionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let recording_sid = params.recording_sid;
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), RecordingSid=crate::apis::urlencode(recording_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteRecordingTranscriptionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a transcription from the account used to make the request
pub async fn delete_transcription(
    configuration: &configuration::Configuration,
    params: DeleteTranscriptionParams,
) -> Result<(), Error<DeleteTranscriptionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid),
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteTranscriptionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn fetch_recording_transcription(configuration: &configuration::Configuration, params: FetchRecordingTranscriptionParams) -> Result<models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription, Error<FetchRecordingTranscriptionError>>{
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let recording_sid = params.recording_sid;
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), RecordingSid=crate::apis::urlencode(recording_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchRecordingTranscriptionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch an instance of a Transcription
pub async fn fetch_transcription(
    configuration: &configuration::Configuration,
    params: FetchTranscriptionParams,
) -> Result<
    models::ApiPeriodV2010PeriodAccountPeriodTranscription,
    Error<FetchTranscriptionError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid),
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchTranscriptionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn list_recording_transcription(
    configuration: &configuration::Configuration,
    params: ListRecordingTranscriptionParams,
) -> Result<
    models::ListRecordingTranscriptionResponse,
    Error<ListRecordingTranscriptionError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let recording_sid = params.recording_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), RecordingSid=crate::apis::urlencode(recording_sid));
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder
            .query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder
            .query(&[("Page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder
            .query(&[("PageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListRecordingTranscriptionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of transcriptions belonging to the account used to make the request
pub async fn list_transcription(
    configuration: &configuration::Configuration,
    params: ListTranscriptionParams,
) -> Result<models::ListTranscriptionResponse, Error<ListTranscriptionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Transcriptions.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid)
    );
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder
            .query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder
            .query(&[("Page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder
            .query(&[("PageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListTranscriptionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Stop a Transcription using either the SID of the Transcription resource or the `name` used when creating the resource
pub async fn update_realtime_transcription(
    configuration: &configuration::Configuration,
    params: UpdateRealtimeTranscriptionParams,
) -> Result<
    models::ApiPeriodV2010PeriodAccountPeriodCallPeriodRealtimeTranscription,
    Error<UpdateRealtimeTranscriptionError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let call_sid = params.call_sid;
    let sid = params.sid;
    let status = params.status;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Transcriptions/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CallSid=crate::apis::urlencode(call_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("Status", status.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateRealtimeTranscriptionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
