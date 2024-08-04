//! Models for the Sendblue API
//!
//! This module provides the data models used by the Sendblue API, including messages, URLs,
//! statuses, and request/response structures for various API endpoints.

pub mod callback_url;
pub mod evaluate_service;
pub mod media_url;
pub mod message;
//pub mod phonenumber;
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
pub use send_style::SendStyle;
pub use status::{ErrorCode, Status};
pub use typing_indicator::{TypingIndicator, TypingIndicatorResponse, TypingIndicatorStatus};
pub use voice_note::VoiceNote;
