//! Status Model
//!
//! This module provides the data model for message statuses used in the Sendblue API.

use serde::{Deserialize, Serialize};

/// Status of the message in the Sendblue API
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Queued,
    Failed,
    Sent,
    Delivered,
    Read,
}

/// Error codes returned by the Sendblue API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ErrorCode {
    #[serde(rename = "4000")]
    ValidationError,
    #[serde(rename = "4001")]
    RateLimitExceeded,
    #[serde(rename = "4002")]
    BlacklistedNumber,
    #[serde(rename = "5000")]
    InternalError,
    #[serde(rename = "5003")]
    ServerRateExceeded,
    #[serde(rename = "10001")]
    MessageFailedToSend,
    #[serde(rename = "10002")]
    FailedToResolveMessageStatus,
    #[serde(other)]
    Unknown,
}
