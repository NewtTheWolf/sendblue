//! Prelude for the Sendblue API
//!
//! This module re-exports commonly used items for convenience, including the Sendblue client,
//! error types, and various models such as messages and URLs.

pub use crate::errors::SendblueError;
pub use crate::models::{
    CallbackUrl, EvaluateService, EvaluateServiceBuilder, EvaluateServiceResponse, MediaUrl,
    Message, MessageBuilder, MessageResponse, SendStyle, Status, TypingIndicatorResponse,
    VoiceNote,
};
pub use crate::traits::Url;
