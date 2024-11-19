# \Api20100401TranscriptionApi

All URIs are relative to *https://api.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_realtime_transcription**](Api20100401TranscriptionApi.md#create_realtime_transcription) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Transcriptions.json | 
[**delete_recording_transcription**](Api20100401TranscriptionApi.md#delete_recording_transcription) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json | 
[**delete_transcription**](Api20100401TranscriptionApi.md#delete_transcription) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json | 
[**fetch_recording_transcription**](Api20100401TranscriptionApi.md#fetch_recording_transcription) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json | 
[**fetch_transcription**](Api20100401TranscriptionApi.md#fetch_transcription) | **GET** /2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json | 
[**list_recording_transcription**](Api20100401TranscriptionApi.md#list_recording_transcription) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions.json | 
[**list_transcription**](Api20100401TranscriptionApi.md#list_transcription) | **GET** /2010-04-01/Accounts/{AccountSid}/Transcriptions.json | 
[**update_realtime_transcription**](Api20100401TranscriptionApi.md#update_realtime_transcription) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Transcriptions/{Sid}.json | 



## create_realtime_transcription

> models::ApiPeriodV2010PeriodAccountPeriodCallPeriodRealtimeTranscription create_realtime_transcription(account_sid, call_sid, name, track, status_callback_url, status_callback_method, inbound_track_label, outbound_track_label, partial_results, language_code, transcription_engine, profanity_filter, speech_model, hints, enable_automatic_punctuation)


Create a Transcription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Transcription resource. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Transcription resource is associated with. | [required] |
**name** | Option<**String**> | The user-specified name of this Transcription, if one was given when the Transcription was created. This may be used to stop the Transcription. |  |
**track** | Option<**models::RealtimeTranscriptionEnumTrack**> |  |  |
**status_callback_url** | Option<**String**> | Absolute URL of the status callback. |  |
**status_callback_method** | Option<**String**> | The http method for the status_callback (one of GET, POST). |  |
**inbound_track_label** | Option<**String**> | Friendly name given to the Inbound Track |  |
**outbound_track_label** | Option<**String**> | Friendly name given to the Outbound Track |  |
**partial_results** | Option<**bool**> | Indicates if partial results are going to be sent to the customer |  |
**language_code** | Option<**String**> | Language code used by the transcription engine, specified in [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) format |  |
**transcription_engine** | Option<**String**> | Definition of the transcription engine to be used, among those supported by Twilio |  |
**profanity_filter** | Option<**bool**> | indicates if the server will attempt to filter out profanities, replacing all but the initial character in each filtered word with asterisks |  |
**speech_model** | Option<**String**> | Recognition model used by the transcription engine, among those supported by the provider |  |
**hints** | Option<**String**> | A Phrase contains words and phrase \\\"hints\\\" so that the speech recognition engine is more likely to recognize them. |  |
**enable_automatic_punctuation** | Option<**bool**> | The provider will add punctuation to recognition result |  |

### Return type

[**models::ApiPeriodV2010PeriodAccountPeriodCallPeriodRealtimeTranscription**](api.v2010.account.call.realtime_transcription.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_transcription

> delete_recording_transcription(account_sid, recording_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete. | [required] |
**recording_sid** | **String** | The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Transcription resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transcription

> delete_transcription(account_sid, sid)


Delete a transcription from the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Transcription resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_recording_transcription

> models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription fetch_recording_transcription(account_sid, recording_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch. | [required] |
**recording_sid** | **String** | The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Transcription resource to fetch. | [required] |

### Return type

[**models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription**](api.v2010.account.recording.recording_transcription.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_transcription

> models::ApiPeriodV2010PeriodAccountPeriodTranscription fetch_transcription(account_sid, sid)


Fetch an instance of a Transcription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Transcription resource to fetch. | [required] |

### Return type

[**models::ApiPeriodV2010PeriodAccountPeriodTranscription**](api.v2010.account.transcription.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recording_transcription

> models::ListRecordingTranscriptionResponse list_recording_transcription(account_sid, recording_sid, page_size, page, page_token)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read. | [required] |
**recording_sid** | **String** | The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcriptions to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**i32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |

### Return type

[**models::ListRecordingTranscriptionResponse**](ListRecordingTranscriptionResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transcription

> models::ListTranscriptionResponse list_transcription(account_sid, page_size, page, page_token)


Retrieve a list of transcriptions belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**i32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |

### Return type

[**models::ListTranscriptionResponse**](ListTranscriptionResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_realtime_transcription

> models::ApiPeriodV2010PeriodAccountPeriodCallPeriodRealtimeTranscription update_realtime_transcription(account_sid, call_sid, sid, status)


Stop a Transcription using either the SID of the Transcription resource or the `name` used when creating the resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Transcription resource. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Transcription resource is associated with. | [required] |
**sid** | **String** | The SID of the Transcription resource, or the `name` used when creating the resource | [required] |
**status** | **models::RealtimeTranscriptionEnumUpdateStatus** |  | [required] |

### Return type

[**models::ApiPeriodV2010PeriodAccountPeriodCallPeriodRealtimeTranscription**](api.v2010.account.call.realtime_transcription.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

