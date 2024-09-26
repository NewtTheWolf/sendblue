//! Typing Indicator Model
//!
//! This module provides the request and response models for typing indicators used in the Sendblue API.

use serde::{Deserialize, Serialize};
use crate::model::phonenumber::deserialize_phone_number;
use super::PhoneNumber;

/// Status of the typing indicator in the Sendblue API
///
/// # Variants
///
/// * `Sent` - The typing indicator was sent successfully
/// * `Error` - An error occurred while sending the typing indicator
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "UPPERCASE")]
pub enum TypingIndicatorStatus {
    Sent,
    Error,
}

/// Response from the Sendblue API after sending a typing indicator
///
/// The response comes back as JSON with the following fields:
/// - `number`: The number you evaluated in E.164 format
/// - `status`: The status of the typing indicator you tried to send (this will either be SENT or ERROR)
/// - `error_message`: The error message if the status is ERROR
#[derive(Serialize, Deserialize, Debug)]
pub struct TypingIndicatorResponse {
    /// The number you evaluated in E.164 format
    #[serde(deserialize_with = "deserialize_phone_number")]
    pub number: PhoneNumber,
    /// The status of the typing indicator you tried to send (this will either be SENT or ERROR)
    pub status: TypingIndicatorStatus,
    /// The error message if the status is ERROR (optional)
    pub error_message: Option<String>,
}

/// Typing Indicator Request
///
/// This struct represents a request to send a typing indicator in the Sendblue API.
///
/// # Examples
///
/// ```
/// use sendblue::models::TypingIndicator;
/// use phonenumber::parse;
///
/// let phone_number = parse(None, "+1234567890").unwrap();
/// let request = TypingIndicator {
///     number: phone_number,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct TypingIndicator {
    /// The phone number to send the typing indicator to
    #[serde(deserialize_with = "deserialize_phone_number")]
    pub number: PhoneNumber,
}
