/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncomingPhoneNumberEnumEmergencyAddressStatus {
    #[serde(rename = "registered")]
    Registered,
    #[serde(rename = "unregistered")]
    Unregistered,
    #[serde(rename = "pending-registration")]
    PendingRegistration,
    #[serde(rename = "registration-failure")]
    RegistrationFailure,
    #[serde(rename = "pending-unregistration")]
    PendingUnregistration,
    #[serde(rename = "unregistration-failure")]
    UnregistrationFailure,

}

impl std::fmt::Display for IncomingPhoneNumberEnumEmergencyAddressStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Registered => write!(f, "registered"),
            Self::Unregistered => write!(f, "unregistered"),
            Self::PendingRegistration => write!(f, "pending-registration"),
            Self::RegistrationFailure => write!(f, "registration-failure"),
            Self::PendingUnregistration => write!(f, "pending-unregistration"),
            Self::UnregistrationFailure => write!(f, "unregistration-failure"),
        }
    }
}

impl Default for IncomingPhoneNumberEnumEmergencyAddressStatus {
    fn default() -> IncomingPhoneNumberEnumEmergencyAddressStatus {
        Self::Registered
    }
}

