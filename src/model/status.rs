//! Status Model
//!
//! This module provides the data model for message statuses used in the Sendblue API.

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Status of the message in the Sendblue API
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Queued,
    Failed,
    Sent,
    Delivered,
    Read,
    Received,
}

/// Error codes returned by the Sendblue API
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum ErrorCode {
    /// Validation Error: see error_message field (Code: 4000)
    #[serde(rename = "4000")]
    ValidationError,

    /// Rate Limit Exceeded (Code: 4001)
    #[serde(rename = "4001")]
    RateLimitExceeded,

    /// Blacklisted Number (e.g., emergency number like 911) (Code: 4002)
    #[serde(rename = "4002")]
    BlacklistedNumber,

    /// Internal Error (Code: 5000)
    #[serde(rename = "5000")]
    InternalError,

    /// Server Rate Exceeded (Code: 5003)
    #[serde(rename = "5003")]
    ServerRateExceeded,

    /// Message failed to send (Code: 10001)
    #[serde(rename = "10001")]
    MessageFailedToSend,

    /// Failed to resolve message status (Code: 10002)
    #[serde(rename = "10002")]
    FailedToResolveMessageStatus,

    /// Unknown error code
    #[serde(other)]
    Unknown,
}
