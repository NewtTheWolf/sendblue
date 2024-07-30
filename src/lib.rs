// Copyright 2024 NewtTheWolf
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Sendblue API Client
//!
//! This module provides a client for interacting with the Sendblue API, including methods
//! for sending messages, retrieving messages, and evaluating phone numbers.
//!
//! # Overview
//!
//! The Sendblue API allows you to send messages, retrieve message histories, and evaluate
//! phone numbers for their ability to use iMessage. This module encapsulates these functionalities
//! in a user-friendly Rust client.
//!
//! # Features
//!
//! - **Send Messages**: Send single or group messages using the Sendblue API.
//! - **Retrieve Messages**: Fetch message histories with filtering and pagination options.
//! - **Evaluate Phone Numbers**: Check if a phone number can send/receive iMessages.
//! - **Typing Indicators**: Send typing indicators to recipients.
//!
//! # Installation
//!
//! To add `sendblue` to your project, include it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sendblue = "0.1.0"
//! ```
//!
//! Or use the cargo add command:
//!
//! ```sh
//! cargo add sendblue
//! ```
//!
//! # Usage
//!
//! To use the Sendblue API client, create an instance of `SendblueClient` with your API key and secret.
//!
//! ```
//! use sendblue::SendblueClient;
//!
//! let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
//! ```
//!
//! # Examples
//!
//! ## Sending a Message
//!
//! ```
//! use sendblue::SendblueClient;
//! use sendblue::models::MessageBuilder;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
//!
//!     let message = MessageBuilder::new(phonenumber::parse(None, "+10722971673").unwrap())
//!         .content("Hello, world!".into())
//!         .build()
//!         .unwrap();
//!
//!     match client.send(&message).await {
//!         Ok(response) => println!("Message sent: {:?}", response),
//!         Err(e) => eprintln!("Error sending message: {:?}", e),
//!     }
//! }
//! ```
//!
//! ## Retrieving Messages
//!
//! ```
//! use sendblue::SendblueClient;
//! use sendblue::models::GetMessagesParamsBuilder;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
//!
//!     let params = GetMessagesParamsBuilder::new()
//!         .limit(Some(50))
//!         .offset(Some(0))
//!         .number(Some(phonenumber::parse(None, "+10722971673").unwrap()))
//!         .from_date(Some("2023-06-15 12:00:00".into()))
//!         .build();
//!
//!     match client.get_messages(params).await {
//!         Ok(response) => println!("Messages retrieved: {:?}", response.messages),
//!         Err(e) => eprintln!("Error retrieving messages: {:?}", e),
//!     }
//! }
//! ```
//!
//! ## Evaluating a Phone Number
//!
//! ```
//! use sendblue::SendblueClient;
//! use sendblue::models::EvaluateServiceBuilder;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
//!
//!     let evaluate_service = EvaluateServiceBuilder::new()
//!         .number(phonenumber::parse(None, "+10722971673").unwrap())
//!         .build();
//!
//!     match client.evaluate_service(&evaluate_service).await {
//!         Ok(response) => println!("Evaluation result: {:?}", response),
//!         Err(e) => eprintln!("Error evaluating number: {:?}", e),
//!     }
//! }
//! ```
//!
//! ## Sending a Typing Indicator
//!
//! ```
//! use sendblue::SendblueClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
//!
//!     let number = phonenumber::parse(None, "+10722971673").unwrap();
//!
//!     match client.send_typing_indicator(&number).await {
//!         Ok(response) => println!("Typing indicator sent: {:?}", response),
//!         Err(e) => eprintln!("Error sending typing indicator: {:?}", e),
//!     }
//! }
//! ```

use crate::models::{
    EvaluateService, EvaluateServiceResponse, GetMessagesParams, GetMessagesResponse,
    TypingIndicatorResponse,
};
use phonenumber::PhoneNumber;
use reqwest::{header::HeaderMap, Client};

pub mod error;
pub mod models;
pub mod prelude;
pub mod traits;

pub use error::SendblueError;
pub use phonenumber;
use traits::SendableMessage;

static BASE_URL: &str = "https://api.sendblue.co/api";

/// Client for the Sendblue API
///
/// The `SendblueClient` struct provides methods for interacting with the Sendblue API.
///
/// # Examples
///
/// ```
/// use sendblue::SendblueClient;
///
/// let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
/// ```
pub struct SendblueClient {
    pub api_key: String,
    pub api_secret: String,
    pub client: Client,
    base_url: String,
}

impl SendblueClient {
    /// Creates a new Sendblue client with the default reqwest client
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authentication
    /// * `api_secret` - The API secret for authentication
    ///
    /// # Returns
    ///
    /// * `SendblueClient` - A new Sendblue client instance
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::SendblueClient;
    ///
    /// let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
    /// ```
    pub fn new(api_key: String, api_secret: String) -> Self {
        SendblueClient {
            api_key,
            api_secret,
            client: Client::new(),
            base_url: BASE_URL.into(),
        }
    }

    /// Creates a new Sendblue client with a custom base URL
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authentication
    /// * `api_secret` - The API secret for authentication
    /// * `base_url` - The base URL for the API
    ///
    /// # Returns
    ///
    /// * `SendblueClient` - A new Sendblue client instance
    ///
    /// This is a private function and not intended for public use.
    pub fn new_with_url(api_key: String, api_secret: String, base_url: String) -> Self {
        SendblueClient {
            api_key,
            api_secret,
            client: Client::new(),
            base_url,
        }
    }

    /// Sends a message using the Sendblue API
    ///
    /// # Arguments
    ///
    /// * `message` - The message to be sent
    ///
    /// # Returns
    ///
    /// * `MessageResponse` - The response from the Sendblue API
    /// * `SendblueError` - An error that occurred during the request
    ///
    /// # Examples
    ///
    /// Sending a normal message:
    ///
    /// ```
    /// use sendblue::SendblueClient;
    /// use sendblue::models::MessageBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
    ///
    ///     let message = MessageBuilder::new(phonenumber::parse(None, "+10722971673").unwrap())
    ///         .content("Hello, world!".into())
    ///         .build()
    ///         .unwrap();
    ///
    ///     match client.send(&message).await {
    ///         Ok(response) => println!("Message sent: {:?}", response),
    ///         Err(e) => eprintln!("Error sending message: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// Sending a group message:
    ///
    /// ```
    /// use sendblue::SendblueClient;
    /// use sendblue::models::{MessageBuilder, GroupMessage};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
    ///
    ///     let group_message = MessageBuilder::<GroupMessage>::new_group()
    ///         .numbers(vec![phonenumber::parse(None, "+10722971673").unwrap(), phonenumber::parse(None, "+10722971673").unwrap()])
    ///         .content("Hello, group!".into())
    ///         .build()
    ///         .unwrap();
    ///
    ///     match client.send::<>(&group_message).await {
    ///         Ok(response) => println!("Group message sent: {:?}", response),
    ///         Err(e) => eprintln!("Error sending group message: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn send<T: SendableMessage>(
        &self,
        message: &T,
    ) -> Result<T::ResponseType, SendblueError> {
        let url = format!("{}{}", self.base_url, T::endpoint());
        let mut headers = HeaderMap::new();
        headers.insert("sb-api-key-id", self.api_key.parse().unwrap());
        headers.insert("sb-api-secret-key", self.api_secret.parse().unwrap());

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(message)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let message_response = response.json::<T::ResponseType>().await?;
                Ok(message_response)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(SendblueError::BadRequest(response.text().await?))
            }
            _ => Err(SendblueError::Unknown(response.text().await?)),
        }
    }

    /// Retrieves messages using the Sendblue API
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for filtering and paginating messages
    ///
    /// # Returns
    ///
    /// * `GetMessagesResponse` - The response containing the retrieved messages
    /// * `SendblueError` - An error that occurred during the request
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::SendblueClient;
    /// use sendblue::models::{GetMessagesParamsBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
    ///
    ///     let params = GetMessagesParamsBuilder::new()
    ///         .limit(Some(50))
    ///         .offset(Some(0))
    ///         .number(Some(phonenumber::parse(None, "+10722971673").unwrap()))
    ///         .from_date(Some("2023-06-15 12:00:00".into()))
    ///         .cid(None)
    ///         .build();
    ///
    ///     match client.get_messages(params).await {
    ///         Ok(response) => println!("Messages retrieved: {:?}", response.messages),
    ///         Err(e) => eprintln!("Error retrieving messages: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_messages(
        &self,
        params: GetMessagesParams,
    ) -> Result<GetMessagesResponse, SendblueError> {
        let url = format!("{}/accounts/messages", self.base_url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("sb-api-key-id", self.api_key.parse().unwrap());
        headers.insert("sb-api-secret-key", self.api_secret.parse().unwrap());

        let response = self
            .client
            .get(&url)
            .headers(headers)
            .query(&params)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let messages_response = response.json::<GetMessagesResponse>().await?;
                Ok(messages_response)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(SendblueError::BadRequest(response.text().await?))
            }
            _ => Err(SendblueError::Unknown(response.text().await?)),
        }
    }

    /// Evaluates if a number can send/receive iMessages using the Sendblue API
    ///
    /// # Arguments
    ///
    /// * `evaluate_service` - The evaluation request containing the phone number in E.164 format
    ///
    /// # Returns
    ///
    /// * `EvaluateServiceResponse` - The response containing the evaluation result
    /// * `SendblueError` - An error that occurred during the request
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::SendblueClient;
    /// use sendblue::models::{EvaluateServiceBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
    ///
    /// let evaluate_service = EvaluateServiceBuilder::new()
    ///     .number(phonenumber::parse(None, "+10722971673").unwrap())
    ///     .build();
    ///
    ///     match client.evaluate_service(&evaluate_service).await {
    ///         Ok(response) => println!("Evaluation result: {:?}", response),
    ///         Err(e) => eprintln!("Error evaluating number: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn evaluate_service(
        &self,
        evaluate_service: &EvaluateService,
    ) -> Result<EvaluateServiceResponse, SendblueError> {
        let url = format!("{}/evaluate-service", self.base_url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("sb-api-key-id", self.api_key.parse().unwrap());
        headers.insert("sb-api-secret-key", self.api_secret.parse().unwrap());

        let response = self
            .client
            .get(&url)
            .headers(headers)
            .query(&[("number", &evaluate_service.number.to_string())])
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let service_response = response.json::<EvaluateServiceResponse>().await?;
                Ok(service_response)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(SendblueError::BadRequest(response.text().await?))
            }
            _ => Err(SendblueError::Unknown(response.text().await?)),
        }
    }

    /// Sends a typing indicator to a recipient using the Sendblue API
    ///
    /// # Arguments
    ///
    /// * `number` - The recipient's phone number in E.164 format
    ///
    /// # Returns
    ///
    /// * `TypingIndicatorResponse` - The response from the Sendblue API
    /// * `SendblueError` - An error that occurred during the request
    ///
    /// # Examples
    ///
    /// ```
    /// use sendblue::SendblueClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
    ///
    ///     let number = phonenumber::parse(None, "+10722971673").unwrap();
    ///
    ///     match client.send_typing_indicator(&number).await {
    ///         Ok(response) => println!("Typing indicator sent: {:?}", response),
    ///         Err(e) => eprintln!("Error sending typing indicator: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn send_typing_indicator(
        &self,
        number: &PhoneNumber,
    ) -> Result<TypingIndicatorResponse, SendblueError> {
        let url = format!("{}/send-typing-indicator", self.base_url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("sb-api-key-id", self.api_key.parse().unwrap());
        headers.insert("sb-api-secret-key", self.api_secret.parse().unwrap());
        let body = serde_json::json!({ "number": number.to_string() });

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let typing_indicator_response = response.json::<TypingIndicatorResponse>().await?;
                Ok(typing_indicator_response)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(SendblueError::BadRequest(response.text().await?))
            }
            _ => Err(SendblueError::Unknown(response.text().await?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use models::{
        EvaluateServiceBuilder, GetMessagesParamsBuilder, GroupMessage, MessageBuilder, Status,
        TypingIndicatorStatus,
    };
    use phonenumber::parse;
    use serde_json::json;

    fn create_client_with_mock_url(base_url: &str) -> SendblueClient {
        SendblueClient::new_with_url("test_key".into(), "test_secret".into(), base_url.into())
    }

    #[tokio::test]
    async fn test_send_message_success() {
        let mock_server = MockServer::start();
        let mock = mock_server.mock(|when, then| {
            when.method("POST")
                .path("/send-message")
                .header("sb-api-key-id", "test_key")
                .header("sb-api-secret-key", "test_secret");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "accountEmail": "YOUR EMAIL",
                    "content": "Hello world!",
                    "is_outbound": true,
                    "status": "QUEUED",
                    "error_code": null,
                    "error_message": null,
                    "message_handle": "dfd747ba-5600-4a8a-804a-a614a0fbc1c5",
                    "date_sent": "2023-09-27T16:35:32.287Z",
                    "date_updated": "2023-09-27T16:35:32.703Z",
                    "from_number": "+16468528190",
                    "number": "+19998887777",
                    "to_number": "+19998887777",
                    "was_downgraded": null,
                    "plan": "dedicated",
                    "media_url": "https://picsum.photos/200/300.jpg",
                    "message_type": "message",
                    "group_id": "",
                    "participants": [],
                    "send_style": "invisible",
                    "opted_out": false,
                    "error_detail": null
                }));
        });

        let client = create_client_with_mock_url(&mock_server.base_url());
        let phone_number = parse(None, "+10722971673").unwrap();
        let message = MessageBuilder::new(phone_number.clone())
            .content("Test message".into())
            .build()
            .unwrap();

        let result = client.send(&message).await;
        if let Err(e) = &result {
            eprintln!("Error in test_send_message_success: {:?}", e);
        }
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, Status::Queued);
        assert_eq!(
            response.message_handle,
            "dfd747ba-5600-4a8a-804a-a614a0fbc1c5"
        );
        mock.assert_hits(1);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_messages_success() {
        let mock_server = MockServer::start();
        let mock = mock_server.mock(|when, then| {
            when.method("GET")
                .path("/accounts/messages")
                .header("sb-api-key-id", "test_key")
                .header("sb-api-secret-key", "test_secret");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "messages": [
                        {
                            "error_message": null,
                            "date": "2023-09-21T20:22:05.066Z",
                            "to_number": "+10722971673",
                            "date_sent": {
                                "_seconds": 1695327725,
                                "_nanoseconds": 66000000
                            },
                            "date_updated": {
                                "_seconds": 1695327725,
                                "_nanoseconds": 456000000
                            },
                            "error_detail": null,
                            "phoneID": "worker_5s_spacegray_1",
                            "message_type": "message",
                            "uuid": "595578e5-6701-4b89-ac9b-28cbfe99cd",
                            "media_url": "",
                            "content": "test\n - Sent using sendblue.co",
                            "send_style": "",
                            "callback_url": "",
                            "is_outbound": true,
                            "allow_sms": false,
                            "accountEmail": "youremail@gmail.com",
                            "was_downgraded": null,
                            "group_id": "",
                            "from_number": "+88888888888",
                            "error_code": 22,
                            "row_id": "4444",
                            "status": "ERROR"
                        }
                    ]
                }));
        });

        let client = create_client_with_mock_url(&mock_server.base_url());
        let params = GetMessagesParamsBuilder::new().build();

        let result = client.get_messages(params).await;
        if let Err(e) = &result {
            eprintln!("Error in test_get_messages_success: {:?}", e);
        }
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.messages.len(), 2);
        mock.assert_hits(1);
    }

    #[tokio::test]
    async fn test_send_group_message_success() {
        let mock_server = MockServer::start();
        let mock = mock_server.mock(|when, then| {
            when.method("POST")
                .path("/send-group-message")
                .header("sb-api-key-id", "test_key")
                .header("sb-api-secret-key", "test_secret");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                  "accountEmail": "YOUR EMAIL",
                  "content": "Hello world",
                  "is_outbound": true,
                  "status": "QUEUED",
                  "error_code": null,
                  "error_message": null,
                  "message_handle": "073c1408-a6d9-48e2-ae8c-01f06443833",
                  "date_sent": "2021-05-19T23:07:23.371Z",
                  "date_updated": "2021-05-19T23:07:23.371Z",
                  "from_number": "+19998887777",
                  "number": ["+11112223333", "+13332221111"],
                  "to_number": ["+11112223333", "+13332221111"],
                  "was_downgraded": null,
                  "plan": "blue",
                  "media_url": "https://picsum.photos/200/300.jpg",
                  "message_type": "group",
                  "group_id": "66e3b90d-4447-43c6-9439-15a69408ac2"
                }));
        });

        let client = create_client_with_mock_url(&mock_server.base_url());
        let phone_number1 = parse(None, "+10722971673").unwrap();
        let phone_number2 = parse(None, "+1234567891").unwrap();
        let group_message = MessageBuilder::<GroupMessage>::new_group()
            .numbers(vec![phone_number1, phone_number2])
            .content("Test group message".into())
            .build()
            .unwrap();

        let result = client.send(&group_message).await;
        if let Err(e) = &result {
            eprintln!("Error in test_send_group_message_success: {:?}", e);
        }
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, Status::Queued);
        assert_eq!(
            response.message_handle,
            "073c1408-a6d9-48e2-ae8c-01f06443833"
        );
        mock.assert_hits(1);
    }

    #[tokio::test]
    async fn test_evaluate_service_success() {
        let mock_server = MockServer::start();
        let mock = mock_server.mock(|when, then| {
            when.method("GET")
                .path("/evaluate-service")
                .header("sb-api-key-id", "test_key")
                .header("sb-api-secret-key", "test_secret");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "number": "+10722971673",
                    "service": "iMessage"
                }));
        });

        let client = create_client_with_mock_url(&mock_server.base_url());
        let phone_number = parse(None, "+10722971673").unwrap();
        let evaluate_service = EvaluateServiceBuilder::new().number(phone_number).build();

        let result = client.evaluate_service(&evaluate_service).await;
        if let Err(e) = &result {
            eprintln!("Error in test_evaluate_service_success: {:?}", e);
        }
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.number, "+10722971673");
        assert_eq!(response.service, "iMessage");
        mock.assert_hits(1);
    }

    #[tokio::test]
    async fn test_send_typing_indicator_success() {
        let mock_server = MockServer::start();
        let mock = mock_server.mock(|when, then| {
            when.method("POST")
                .path("/send-typing-indicator")
                .header("sb-api-key-id", "test_key")
                .header("sb-api-secret-key", "test_secret");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "number": "+10722971673",
                    "status": "SENT"
                }));
        });

        let client = create_client_with_mock_url(&mock_server.base_url());
        let phone_number = parse(None, "+10722971673").unwrap();

        let result = client.send_typing_indicator(&phone_number).await;
        if let Err(e) = &result {
            eprintln!("Error in test_send_typing_indicator_success: {:?}", e);
        }
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, TypingIndicatorStatus::Sent);
        mock.assert_hits(1);
    }

    #[tokio::test]
    async fn test_send_typing_indicator_failure() {
        let mock_server = MockServer::start();
        let mock = mock_server.mock(|when, then| {
            when.method("POST")
                .path("/send-typing-indicator")
                .header("sb-api-key-id", "test_key")
                .header("sb-api-secret-key", "test_secret");
            then.status(400)
                .header("content-type", "application/json")
                .json_body(json!({
                    "status": "ERROR",
                    "error_message": "Failed to send typing indicator"
                }));
        });

        let client = create_client_with_mock_url(&mock_server.base_url());
        let phone_number = parse(None, "+10722971673").unwrap();

        let result = client.send_typing_indicator(&phone_number).await;
        if let Err(e) = &result {
            eprintln!("Error in test_send_typing_indicator_failure: {:?}", e);
        }
        assert!(result.is_err());
        let response = result.unwrap_err();
        if let SendblueError::BadRequest(body) = response {
            let expected_error = json!({
                "status": "ERROR",
                "error_message": "Failed to send typing indicator"
            });
            let actual_error: serde_json::Value = serde_json::from_str(&body).unwrap();
            assert_eq!(actual_error, expected_error);
        }
        mock.assert_hits(1);
    }

    #[tokio::test]
    async fn test_send_message_failure() {
        let mock_server = MockServer::start();
        let mock = mock_server.mock(|when, then| {
            when.method("POST")
                .path("/send-message")
                .header("sb-api-key-id", "test_key")
                .header("sb-api-secret-key", "test_secret");
            then.status(400)
                .header("content-type", "application/json")
                .json_body(json!({
                    "status": "ERROR",
                    "message": "Bad request"
                }));
        });

        let client = create_client_with_mock_url(&mock_server.base_url());
        let phone_number = parse(None, "+10722971673").unwrap();
        let message = MessageBuilder::new(phone_number)
            .content("Test message".into())
            .build()
            .unwrap();

        let result = client.send(&message).await;
        if let Err(e) = &result {
            eprintln!("Error in test_send_message_failure: {:?}", e);
        }
        assert!(result.is_err());
        mock.assert_hits(1);
    }
}
