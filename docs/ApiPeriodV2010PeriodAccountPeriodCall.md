# ApiPeriodV2010PeriodAccountPeriodCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string that we created to identify this Call resource. | [optional]
**date_created** | Option<**String**> | The date and time in UTC that this resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format. | [optional]
**date_updated** | Option<**String**> | The date and time in UTC that this resource was last updated, specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format. | [optional]
**parent_call_sid** | Option<**String**> | The SID that identifies the call that created this leg. | [optional]
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Call resource. | [optional]
**to** | Option<**String**> | The phone number, SIP address, Client identifier or SIM SID that received this call. Phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. Client identifiers are formatted `client:name`. SIM SIDs are formatted as `sim:sid`. | [optional]
**to_formatted** | Option<**String**> | The phone number, SIP address or Client identifier that received this call. Formatted for display. Non-North American phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +442071838750). | [optional]
**from** | Option<**String**> | The phone number, SIP address, Client identifier or SIM SID that made this call. Phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. Client identifiers are formatted `client:name`. SIM SIDs are formatted as `sim:sid`. | [optional]
**from_formatted** | Option<**String**> | The calling phone number, SIP address, or Client identifier formatted for display. Non-North American phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +442071838750). | [optional]
**phone_number_sid** | Option<**String**> | If the call was inbound, this is the SID of the IncomingPhoneNumber resource that received the call. If the call was outbound, it is the SID of the OutgoingCallerId resource from which the call was placed. | [optional]
**status** | Option<[**models::CallEnumStatus**](call_enum_status.md)> |  | [optional]
**start_time** | Option<**String**> | The start time of the call, given as UTC in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format. Empty if the call has not yet been dialed. | [optional]
**end_time** | Option<**String**> | The time the call ended, given as UTC in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format. Empty if the call did not complete successfully. | [optional]
**duration** | Option<**String**> | The length of the call in seconds. This value is empty for busy, failed, unanswered, or ongoing calls. | [optional]
**price** | Option<**String**> | The charge for this call, in the currency associated with the account. Populated after the call is completed. May not be immediately available. The price associated with a call only reflects the charge for connectivity.  Charges for other call-related features such as Answering Machine Detection, Text-To-Speech, and SIP REFER are not included in this value. | [optional]
**price_unit** | Option<**String**> | The currency in which `Price` is measured, in [ISO 4127](https://www.iso.org/iso/home/standards/currency_codes.htm) format (e.g., `USD`, `EUR`, `JPY`). Always capitalized for calls. | [optional]
**direction** | Option<**String**> | A string describing the direction of the call. Can be: `inbound` for inbound calls, `outbound-api` for calls initiated via the REST API or `outbound-dial` for calls initiated by a `<Dial>` verb. Using [Elastic SIP Trunking](https://www.twilio.com/docs/sip-trunking), the values can be [`trunking-terminating`](https://www.twilio.com/docs/sip-trunking#termination) for outgoing calls from your communications infrastructure to the PSTN or [`trunking-originating`](https://www.twilio.com/docs/sip-trunking#origination) for incoming calls to your communications infrastructure from the PSTN. | [optional]
**answered_by** | Option<**String**> | Either `human` or `machine` if this call was initiated with answering machine detection. Empty otherwise. | [optional]
**api_version** | Option<**String**> | The API version used to create the call. | [optional]
**forwarded_from** | Option<**String**> | The forwarding phone number if this call was an incoming call forwarded from another number (depends on carrier supporting forwarding). Otherwise, empty. | [optional]
**group_sid** | Option<**String**> | The Group SID associated with this call. If no Group is associated with the call, the field is empty. | [optional]
**caller_name** | Option<**String**> | The caller's name if this call was an incoming call to a phone number with caller ID Lookup enabled. Otherwise, empty. | [optional]
**queue_time** | Option<**String**> | The wait time in milliseconds before the call is placed. | [optional]
**trunk_sid** | Option<**String**> | The unique identifier of the trunk resource that was used for this call. The field is empty if the call was not made using a SIP trunk or if the call is not terminated. | [optional]
**uri** | Option<**String**> | The URI of this resource, relative to `https://api.twilio.com`. | [optional]
**subresource_uris** | Option<[**serde_json::Value**](.md)> | A list of subresources available to this call, identified by their URIs relative to `https://api.twilio.com`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


