//! Voice Note Model
//!
//! This module provides the data model for voice notes used in the Sendblue API.

use std::process::Stdio;

use crate::{r#trait::Url, SendblueError};
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use url::Url as RawUrl;

#[cfg(feature = "convert")]
use bytes::Bytes;

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
    fn new(url: &str) -> Result<Self, SendblueError> {
        let url = RawUrl::parse(url)
            .map_err(|_| SendblueError::ValidationError("invalid url format".to_owned()))?;
        if url.path().ends_with(".caf") {
            Ok(Self(url))
        } else {
            Err(SendblueError::ValidationError(
                "invalid voice note url format, must end with .caf".to_owned(),
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

#[cfg(feature = "convert")]
/// Asynchronously converts an audio byte stream into a `.caf` format using `ffmpeg`.
///
/// # Arguments
///
/// * `audio` - A byte stream of the audio file.
/// * `format` - The format of the input audio file (e.g., "mp3", "wav").
///
/// # Returns
///
/// * `Ok(RawUrl)` - A URL containing the converted `.caf` audio as a base64-encoded data URL.
/// * `Err(SendblueError)` - An error occurred during the conversion process.
///
/// # Errors
///
/// This function will return an error if:
/// - `ffmpeg` is not installed or not found in the system's PATH.
/// - The `ffmpeg` process fails to start or complete.
/// - There is an error reading from or writing to the `ffmpeg` process.
///
/// # Examples
///
/// ```rust
/// use bytes::Bytes;
/// use url::Url;
/// use sendblue::models::convert;
/// use sendblue::error::SendblueError;
///
/// #[tokio::main]
/// async fn main() -> Result<(), SendblueError> {
///     let audio_data = Bytes::from_static(b"audio byte stream here");
///     let format = "mp3";
///
///     let url = convert(audio_data, format).await?;
///
///     println!("Converted audio URL: {}", url);
///     Ok(())
/// }
/// ```
pub async fn convert(audio: Bytes, format: &str) -> Result<RawUrl, SendblueError> {
    // Überprüfe, ob ffmpeg installiert ist

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let status = Command::new("ffmpeg")
        .arg("-version")
        .status()
        .await
        .map_err(|_| {
            SendblueError::ValidationError("ffmpeg not installed or not found in PATH".into())
        })?;

    if !status.success() {
        return Err(SendblueError::ValidationError(
            "ffmpeg command failed".into(),
        ));
    }

    // Starte den ffmpeg-Prozess
    let mut ffmpeg = Command::new("ffmpeg")
        .args(&[
            "-i", "pipe:0", // Input from stdin
            "-acodec", "opus", // Output codec
            "-b:a", "24k", // Bitrate
            "-f", "caf",    // Output format
            "pipe:1", // Output to stdout
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| SendblueError::Unknown("failed to start ffmpeg process".into()))?;

    // Schreibe die Audiodaten in den stdin von ffmpeg
    if let Some(mut stdin) = ffmpeg.stdin.take() {
        stdin
            .write_all(&audio)
            .await
            .map_err(|_| SendblueError::Unknown("failed to write audio data to ffmpeg".into()))?;
    } else {
        return Err(SendblueError::Unknown(
            "failed to open stdin for ffmpeg".into(),
        ));
    }

    // Lese die Ausgabedaten aus dem stdout von ffmpeg
    let mut output = Vec::new();
    if let Some(mut stdout) = ffmpeg.stdout.take() {
        stdout
            .read_to_end(&mut output)
            .await
            .map_err(|_| SendblueError::Unknown("failed to read ffmpeg output".into()))?;
    } else {
        return Err(SendblueError::Unknown(
            "failed to open stdout for ffmpeg".into(),
        ));
    }

    // Kodieren der Ausgabedaten als Data URL
    let base64_audio = base64::encode(output);
    let data_url = format!("data:audio/x-caf;base64,{}", base64_audio);
    let url = RawUrl::parse(&data_url)
        .map_err(|_| SendblueError::ValidationError("failed to parse data URL".into()))?;

    Ok(url)
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
