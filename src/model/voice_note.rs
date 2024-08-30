//! Voice Note Model
//!
//! This module provides the data model for voice notes used in the Sendblue API.

use crate::r#trait::Url;
use serde::{Deserialize, Serialize};
use url::Url as RawUrl;
use validator::ValidationError;

/// A URL specifically for audio messages, must end with `.caf`
///
/// # Examples
///
/// ```
/// use sendblue::models::VoiceNote;
/// use sendblue::r#trait::Url;
///
/// let voice_note = VoiceNote::new("https://example.com/audio.caf").unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceNote(RawUrl);

impl Url for VoiceNote {
    fn new(url: &str) -> Result<Self, ValidationError> {
        let url = RawUrl::parse(url).map_err(|_| ValidationError::new("invalid url format"))?;
        if url.path().ends_with(".caf") {
            Ok(Self(url))
        } else {
            Err(ValidationError::new(
                "invalid voice note url format, must end with .caf",
            ))
        }
    }

    fn from_raw_url(raw_url: RawUrl) -> Self {
        Self(raw_url)
    }

    fn raw_url(&self) -> &RawUrl {
        &self.0
    }
}

impl Serialize for VoiceNote {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for VoiceNote {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        VoiceNote::new(s).map_err(serde::de::Error::custom)
    }
}
