//! Message Model
//!
//! This module provides the data models for messages used in the Sendblue API, including
//! individual and group messages, their builders, and response structures.

use crate::{
    models::{CallbackUrl, MediaUrl, SendStyle},
    SendableMessage, SendblueError,
};
use phonenumber::PhoneNumber;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::Status;

/// Message to be sent using the Sendblue API
///
/// # Examples
///
/// ```
/// use sendblue::models::{Message, MessageBuilder};
///
/// let message = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap())
///     .content("Hello, world!".into())
///     .build()
///     .unwrap();
/// ```
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Message {
    /// The recipient's phone number in E.164 format
    pub number: PhoneNumber,
    /// The content of the message (optional)
    #[validate(length(min = 1))]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    /// The URL of the media to be sent (optional)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub media_url: Option<MediaUrl>,
    /// The callback URL for the message status (optional)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status_callback: Option<CallbackUrl>,
    /// The style of the message delivery (optional)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub send_style: Option<SendStyle>,
}

impl SendableMessage for Message {
    fn endpoint() -> &'static str {
        "/send-message"
    }
}

/// Response from the Sendblue API after sending a message
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    /// The email of the account
    pub account_email: String,
    /// The content of the message
    pub content: String,
    /// Whether the message is outbound
    pub is_outbound: bool,
    /// The status of the message
    pub status: Status,
    /// The error code if any (optional)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub error_code: Option<i32>,
    /// The error message if any (optional)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub error_message: Option<String>,
    /// The handle of the message
    pub message_handle: String,
    /// The date the message was sent
    pub date_sent: String,
    /// The date the message was updated
    pub date_updated: String,
    /// The sender's phone number
    pub from_number: PhoneNumber,
    /// The recipient's phone number
    pub number: PhoneNumber,
    /// Whether the recipient opted out
    pub opted_out: bool,
}

/// Request parameters for getting messages
///
/// # Examples
///
/// ```
/// use sendblue::models::GetMessagesParams;
///
/// let params = GetMessagesParams {
///     cid: Some("contact_id".into()),
///     number: Some(phonenumber::parse(None, "+1234567890").unwrap()),
///     limit: Some(50),
///     offset: Some(0),
///     from_date: Some("2023-06-15 12:00:00".into()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetMessagesParams {
    pub cid: Option<String>,
    pub number: Option<PhoneNumber>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub from_date: Option<String>, // or use a more specific date type
}

/// Message retrieved from the Sendblue API
///
/// # Examples
///
/// ```
/// use sendblue::models::RetrievedMessage;
/// use sendblue::Status;
/// use sendblue::DateTime;
///
/// let message = RetrievedMessage {
///     date: "2023-08-15T16:04:38.866Z".into(),
///     allow_sms: None,
///     send_style: Some("".into()),
///     message_type: "message".into(),
///     uuid: "e8942f7a-c1d2-49e1-b35f-68958754635d".into(),
///     media_url: Some("".into()),
///     content: Some("Hey".into()),
///     number: Some(phonenumber::parse(None, "+1234567890").unwrap()),
///     is_outbound: true,
///     account_email: "youremail@gmail.com".into(),
///     was_downgraded: None,
///     callback_url: Some("".into()),
///     row_id: None,
///     status: Status::Queued,
///     error_message: None,
///     to_number: Some(phonenumber::parse(None, "+1234567890").unwrap()),
///     date_sent: Some(DateTime { seconds: 1695327725, nanoseconds: 66000000 }),
///     date_updated: Some(DateTime { seconds: 1695327725, nanoseconds: 456000000 }),
///     error_detail: None,
///     phone_id: Some("worker_5s_spacegray_1".into()),
///     group_id: Some("".into()),
///     from_number: Some(phonenumber::parse(None, "+1234567890").unwrap()),
///     error_code: Some(22),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct RetrievedMessage {
    pub date: String,
    pub allow_sms: Option<bool>,
    pub send_style: Option<String>,
    pub message_type: String,
    pub uuid: String,
    pub media_url: Option<String>,
    pub content: Option<String>,
    pub number: Option<PhoneNumber>,
    pub is_outbound: bool,
    pub account_email: String,
    pub was_downgraded: Option<bool>,
    pub callback_url: Option<String>,
    pub row_id: Option<String>,
    pub status: Status,
    pub error_message: Option<String>,
    pub to_number: Option<PhoneNumber>,
    pub date_sent: Option<DateTime>,
    pub date_updated: Option<DateTime>,
    pub error_detail: Option<String>,
    pub phone_id: Option<String>,
    pub group_id: Option<String>,
    pub from_number: Option<PhoneNumber>,
    pub error_code: Option<i32>,
}

/// DateTime structure for timestamp fields
///
/// # Examples
///
/// ```
/// use sendblue::models::DateTime;
///
/// let datetime = DateTime {
///     seconds: 1695327725,
///     nanoseconds: 66000000,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct DateTime {
    #[serde(rename = "_seconds")]
    pub seconds: i64,
    #[serde(rename = "_nanoseconds")]
    pub nanoseconds: i32,
}

/// Response from the Sendblue API for getting messages
///
/// # Examples
///
/// ```
/// use sendblue::models::GetMessagesResponse;
/// use sendblue::message::RetrievedMessage;
/// use sendblue::Status;
/// use sendblue::DateTime;
///
/// let response = GetMessagesResponse {
///     messages: vec![
///         RetrievedMessage {
///             date: "2023-08-15T16:04:38.866Z".into(),
///             allow_sms: None,
///             send_style: Some("".into()),
///             message_type: "message".into(),
///             uuid: "e8942f7a-c1d2-49e1-b35f-68958754635d".into(),
///             media_url: Some("".into()),
///             content: Some("Hey".into()),
///             number: Some(phonenumber::parse(None, "+1234567890").unwrap()),
///             is_outbound: true,
///             account_email: "youremail@gmail.com".into(),
///             was_downgraded: None,
///             callback_url: Some("".into()),
///             row_id: None,
///             status: Status::Queued,
///             error_message: None,
///             to_number: Some(phonenumber::parse(None, "+1234567890").unwrap()),
///             date_sent: Some(DateTime { seconds: 1695327725, nanoseconds: 66000000 }),
///             date_updated: Some(DateTime { seconds: 1695327725, nanoseconds: 456000000 }),
///             error_detail: None,
///             phone_id: Some("worker_5s_spacegray_1".into()),
///             group_id: Some("".into()),
///             from_number: Some(phonenumber::parse(None, "+1234567890").unwrap()),
///             error_code: Some(22),
///         }
///     ],
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct GetMessagesResponse {
    pub messages: Vec<RetrievedMessage>,
}

/// Group message request payload
///
/// # Examples
///
/// ```
/// use sendblue::models::GroupMessage;
/// use sendblue::MediaUrl;
/// use sendblue::CallbackUrl;
/// use sendblue::Url;
///
/// let request = GroupMessage {
///     numbers: Some(vec![phonenumber::parse(None, "+19998887777").unwrap(), phonenumber::parse(None, "+17778889999").unwrap()]),
///     group_id: None,
///     content: Some("Hello group!".into()),
///     media_url: Some(MediaUrl::new("https://picsum.photos/200/300.jpg").unwrap()),
///     send_style: None,
///     status_callback: Some(CallbackUrl::new("https://example.com/message-status/1234abcd").unwrap()),
/// };
/// ```
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct GroupMessage {
    /// An array of E.164-formatted phone numbers of the desired recipients in a group chat.
    pub numbers: Option<Vec<PhoneNumber>>,
    /// The group ID to message an existing group.
    pub group_id: Option<String>,
    /// The content of the message.
    #[validate(length(min = 1))]
    pub content: Option<String>,
    /// A URL to a media file to send to the group.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub media_url: Option<MediaUrl>,
    /// The style of delivery of the message.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub send_style: Option<SendStyle>,
    /// An endpoint to notify your app of status updates for this message.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status_callback: Option<CallbackUrl>,
}

impl SendableMessage for GroupMessage {
    fn endpoint() -> &'static str {
        "/send-group-message"
    }
}

/// Group message response payload
#[derive(Serialize, Deserialize, Debug)]
pub struct GroupMessageResponse {
    /// The email of the account
    pub account_email: String,
    /// The content of the message
    pub content: String,
    /// Whether the message is outbound
    pub is_outbound: bool,
    /// The status of the message
    pub status: String,
    /// The error code if any (optional)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub error_code: Option<i32>,
    /// The error message if any (optional)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub error_message: Option<String>,
    /// The handle of the message
    pub message_handle: String,
    /// The date the message was sent
    pub date_sent: String,
    /// The date the message was updated
    pub date_updated: String,
    /// The sender's phone number
    pub from_number: PhoneNumber,
    /// The recipient's phone numbers
    pub numbers: Vec<PhoneNumber>,
    /// Whether the message was downgraded (optional)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub was_downgraded: Option<bool>,
    /// The plan of the message
    pub plan: String,
    /// The URL of the media (optional)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub media_url: Option<MediaUrl>,
    /// The type of the message
    pub message_type: String,
    /// The group ID
    pub group_id: String,
}

/// Generic builder for creating a `Message` or `GroupMessage`
pub struct MessageBuilder<T> {
    message: Option<Message>,
    group_message: Option<GroupMessage>,
    _marker: std::marker::PhantomData<T>,
}

impl MessageBuilder<Message> {
    /// Creates a new `MessageBuilder` for an individual message
    ///
    /// # Arguments
    ///
    /// * `number` - The recipient's phone number in E.164 format
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::MessageBuilder;
    ///
    /// let builder = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap());
    /// ```
    pub fn new(number: PhoneNumber) -> Self {
        Self {
            message: Some(Message {
                number,
                content: None,
                media_url: None,
                status_callback: None,
                send_style: None,
            }),
            group_message: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Sets the content of the message
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the message
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::MessageBuilder;
    ///
    /// let builder = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap())
    ///     .content("Hello, world!".into());
    /// ```
    pub fn content(mut self, content: String) -> Self {
        if let Some(ref mut msg) = self.message {
            msg.content = Some(content);
        }
        self
    }

    /// Sets the media URL of the message
    ///
    /// # Arguments
    ///
    /// * `media_url` - The URL of the media to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::{MessageBuilder, MediaUrl};
    /// use sendblue::Url;
    ///
    /// let builder = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap())
    ///     .media_url(MediaUrl::new("https://example.com/media.jpg").unwrap());
    /// ```
    pub fn media_url(mut self, media_url: MediaUrl) -> Self {
        if let Some(ref mut msg) = self.message {
            msg.media_url = Some(media_url);
        }
        self
    }

    /// Sets the status callback URL of the message
    ///
    /// # Arguments
    ///
    /// * `status_callback` - The callback URL for the message status
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::{MessageBuilder, CallbackUrl};
    /// use sendblue::Url;
    ///
    /// let builder = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap())
    ///     .status_callback(CallbackUrl::new("https://example.com/message-status/1234abcd").unwrap());
    /// ```
    pub fn status_callback(mut self, status_callback: CallbackUrl) -> Self {
        if let Some(ref mut msg) = self.message {
            msg.status_callback = Some(status_callback);
        }
        self
    }

    /// Sets the style of delivery of the message
    ///
    /// # Arguments
    ///
    /// * `send_style` - The style of the message delivery
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::{MessageBuilder, SendStyle};
    ///
    /// let builder = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap())
    ///     .send_style(SendStyle::Invisible);
    /// ```
    pub fn send_style(mut self, send_style: SendStyle) -> Self {
        if let Some(ref mut msg) = self.message {
            msg.send_style = Some(send_style);
        }
        self
    }

    /// Builds the `Message`
    ///
    /// # Returns
    ///
    /// * `Result<Message, ValidationError>` - The constructed `Message` object or a `ValidationError`
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::MessageBuilder;
    ///
    /// let message = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap())
    ///     .content("Hello, world!".into())
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Message, SendblueError> {
        if let Some(msg) = self.message {
            msg.validate()
                .map_err(|e| SendblueError::ValidationError(e.to_string()))?;
            Ok(msg)
        } else {
            Err(SendblueError::ValidationError(
                "Message not initialized".into(),
            ))
        }
    }
}

impl MessageBuilder<GroupMessage> {
    /// Creates a new `MessageBuilder` for a group message
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::MessageBuilder;
    ///
    /// let builder = MessageBuilder::new_group();
    /// ```
    pub fn new_group() -> Self {
        Self {
            message: None,
            group_message: Some(GroupMessage {
                numbers: None,
                group_id: None,
                content: None,
                media_url: None,
                send_style: None,
                status_callback: None,
            }),
            _marker: std::marker::PhantomData,
        }
    }

    /// Sets the list of phone numbers for the group message
    ///
    /// # Arguments
    ///
    /// * `numbers` - An array of E.164-formatted phone numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::MessageBuilder;
    ///
    /// let builder = MessageBuilder::new_group()
    ///     .numbers(vec![phonenumber::parse(None, "+19998887777").unwrap(), phonenumber::parse(None, "+17778889999").unwrap()]);
    /// ```
    pub fn numbers(mut self, numbers: Vec<PhoneNumber>) -> Self {
        if let Some(ref mut grp_msg) = self.group_message {
            grp_msg.numbers = Some(numbers);
        }
        self
    }

    /// Sets the group ID for the group message
    ///
    /// # Arguments
    ///
    /// * `group_id` - The group ID
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::MessageBuilder;
    ///
    /// let builder = MessageBuilder::new_group()
    ///     .group_id("group_id".into());
    /// ```
    pub fn group_id(mut self, group_id: String) -> Self {
        if let Some(ref mut grp_msg) = self.group_message {
            grp_msg.group_id = Some(group_id);
        }
        self
    }

    /// Sets the content of the group message
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the group message
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::MessageBuilder;
    ///
    /// let builder = MessageBuilder::new_group()
    ///     .content("Hello group!".into());
    /// ```
    pub fn content(mut self, content: String) -> Self {
        if let Some(ref mut grp_msg) = self.group_message {
            grp_msg.content = Some(content);
        }
        self
    }

    /// Sets the media URL for the group message
    ///
    /// # Arguments
    ///
    /// * `media_url` - The URL of the media to be sent to the group
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::{MessageBuilder, MediaUrl};
    /// use sendblue::Url;
    ///
    /// let builder = MessageBuilder::new_group()
    ///     .media_url(MediaUrl::new("https://example.com/media.jpg").unwrap());
    /// ```
    pub fn media_url(mut self, media_url: MediaUrl) -> Self {
        if let Some(ref mut grp_msg) = self.group_message {
            grp_msg.media_url = Some(media_url);
        }
        self
    }

    /// Sets the status callback URL for the group message
    ///
    /// # Arguments
    ///
    /// * `status_callback` - The callback URL for the message status
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::{MessageBuilder, CallbackUrl};
    /// use sendblue::Url;
    ///
    /// let builder = MessageBuilder::new_group()
    ///     .status_callback(CallbackUrl::new("https://example.com/message-status/1234abcd").unwrap());
    /// ```
    pub fn status_callback(mut self, status_callback: CallbackUrl) -> Self {
        if let Some(ref mut grp_msg) = self.group_message {
            grp_msg.status_callback = Some(status_callback);
        }
        self
    }

    /// Sets the style of delivery of the group message
    ///
    /// # Arguments
    ///
    /// * `send_style` - The style of the message delivery
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::{MessageBuilder, SendStyle};
    ///
    /// let builder = MessageBuilder::new_group()
    ///     .send_style(SendStyle::Invisible);
    /// ```
    pub fn send_style(mut self, send_style: SendStyle) -> Self {
        if let Some(ref mut grp_msg) = self.group_message {
            grp_msg.send_style = Some(send_style);
        }
        self
    }

    /// Builds the `GroupMessage`
    ///
    /// # Returns
    ///
    /// * `Result<GroupMessage, ValidationError>` - The constructed `GroupMessage` object or a `ValidationError`
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::models::MessageBuilder;
    ///
    /// let group_message = MessageBuilder::new_group()
    ///     .numbers(vec![phonenumber::parse(None, "+19998887777").unwrap(), phonenumber::parse(None, "+17778889999").unwrap()])
    ///     .content("Hello group!".into())
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<GroupMessage, SendblueError> {
        if let Some(grp_msg) = self.group_message {
            if grp_msg.numbers.as_ref().map_or(true, |ns| ns.is_empty())
                && grp_msg.group_id.is_none()
            {
                return Err(SendblueError::ValidationError(
                    "Either numbers or group_id must be provided".into(),
                ));
            }
            if grp_msg.content.is_none() && grp_msg.media_url.is_none() {
                return Err(SendblueError::ValidationError(
                    "Either content or media_url must be provided".into(),
                ));
            }
            grp_msg
                .validate()
                .map_err(|e| SendblueError::ValidationError(e.to_string()))?;
            Ok(grp_msg)
        } else {
            Err(SendblueError::ValidationError(
                "GroupMessage not initialized".into(),
            ))
        }
    }
}

/// Builder for creating a `GetMessagesParams`
///
/// # Examples
///
/// ```
/// use sendblue::models::GetMessagesParamsBuilder;
///
/// let params = GetMessagesParamsBuilder::new()
///     .cid(Some("contact_id".into()))
///     .number(Some(phonenumber::parse(None, "+1234567890").unwrap()))
///     .limit(Some(50))
///     .offset(Some(0))
///     .from_date(Some("2023-06-15 12:00:00".into()))
///     .build();
/// ```
pub struct GetMessagesParamsBuilder {
    cid: Option<String>,
    number: Option<PhoneNumber>,
    limit: Option<u32>,
    offset: Option<u32>,
    from_date: Option<String>,
}

impl GetMessagesParamsBuilder {
    pub fn new() -> Self {
        Self {
            cid: None,
            number: None,
            limit: None,
            offset: None,
            from_date: None,
        }
    }

    pub fn cid(mut self, cid: Option<String>) -> Self {
        self.cid = cid;
        self
    }

    pub fn number(mut self, number: Option<PhoneNumber>) -> Self {
        self.number = number;
        self
    }

    pub fn limit(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }

    pub fn offset(mut self, offset: Option<u32>) -> Self {
        self.offset = offset;
        self
    }

    pub fn from_date(mut self, from_date: Option<String>) -> Self {
        self.from_date = from_date;
        self
    }

    pub fn build(self) -> GetMessagesParams {
        GetMessagesParams {
            cid: self.cid,
            number: self.number,
            limit: self.limit,
            offset: self.offset,
            from_date: self.from_date,
        }
    }
}
