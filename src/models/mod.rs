//! Models for the Sendblue API
//!
//! This module provides the data models used by the Sendblue API, including messages, URLs,
//! statuses, and request/response structures for various API endpoints.

pub mod callback_url;
pub mod evaluate_service;
pub mod media_url;
pub mod message;
pub mod send_style;
pub mod status;
pub mod typing_indicator;
pub mod voice_note;

pub use callback_url::CallbackUrl;
pub use evaluate_service::{EvaluateService, EvaluateServiceBuilder, EvaluateServiceResponse};
pub use media_url::MediaUrl;
pub use message::{
    GetMessagesParams, GetMessagesParamsBuilder, GetMessagesResponse, GroupMessage,
    GroupMessageResponse, Message, MessageBuilder, MessageResponse, MessageStatusCallback,
    RetrievedMessage,
};
pub use phonenumber::PhoneNumber;
use phonenumber::{parse, Mode};
pub use send_style::SendStyle;
use serde::{Deserialize, Deserializer, Serializer};
pub use status::{ErrorCode, Status};
pub use typing_indicator::{TypingIndicator, TypingIndicatorResponse, TypingIndicatorStatus};
pub use voice_note::VoiceNote;

pub fn deserialize_phone_number<'de, D>(deserializer: D) -> Result<PhoneNumber, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    parse(None, s).map_err(serde::de::Error::custom)
}

pub fn deserialize_option_phone_number<'de, D>(
    deserializer: D,
) -> Result<Option<PhoneNumber>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<&str> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => parse(None, s).map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

pub fn deserialize_vec_phone_number<'de, D>(deserializer: D) -> Result<Vec<PhoneNumber>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<&str> = Deserialize::deserialize(deserializer)?;
    vec.into_iter()
        .map(|s| parse(None, s).map_err(serde::de::Error::custom))
        .collect()
}

pub fn deserialize_option_vec_phone_number<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<PhoneNumber>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<Vec<&str>> = Option::deserialize(deserializer)?;
    match opt {
        Some(vec) => {
            let mut phone_numbers = Vec::new();
            for s in vec {
                match parse(None, s) {
                    Ok(phone_number) => phone_numbers.push(phone_number),
                    Err(err) => return Err(serde::de::Error::custom(err)),
                }
            }
            Ok(Some(phone_numbers))
        }
        None => Ok(None),
    }
}

pub fn serialize_phone_number<S>(number: &PhoneNumber, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let e164 = number.format().mode(Mode::E164).to_string();
    serializer.serialize_str(&e164)
}
