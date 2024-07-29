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
//!     let message = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap())
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
//!         .number(Some(phonenumber::parse(None, "+12345678912").unwrap()))
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
//!         .number(phonenumber::parse(None, "+19999999999").unwrap())
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
//!     let number = phonenumber::parse(None, "+1234567890").unwrap();
//!
//!     match client.send_typing_indicator(&number).await {
//!         Ok(response) => println!("Typing indicator sent: {:?}", response),
//!         Err(e) => eprintln!("Error sending typing indicator: {:?}", e),
//!     }
//! }
//! ```

use phonenumber::PhoneNumber;
use reqwest::{header::HeaderMap, Client};

pub mod error;
pub mod models;
pub mod prelude;
pub mod traits;

pub use error::SendblueError;
pub use models::*;
pub use traits::*;

const BASE_URL: &str = "https://api.sendblue.co/api";

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
    fn new_with_url(api_key: String, api_secret: String, base_url: String) -> Self {
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
    ///     let message = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap())
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
    ///         .numbers(vec![phonenumber::parse(None, "+1234567890").unwrap(), phonenumber::parse(None, "+1234567890").unwrap()])
    ///         .content("Hello, group!".into())
    ///         .build()
    ///         .unwrap();
    ///
    ///     match client.send(&group_message).await {
    ///         Ok(response) => println!("Group message sent: {:?}", response),
    ///         Err(e) => eprintln!("Error sending group message: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn send<T: SendableMessage>(
        &self,
        message: &T,
    ) -> Result<MessageResponse, SendblueError> {
        let url = format!("{}{}", BASE_URL, T::endpoint());
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
                let message_response = response.json::<MessageResponse>().await?;
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
    ///         .number(Some(phonenumber::parse(None, "+12345678912").unwrap()))
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
        let url = format!("{}/accounts/messages", BASE_URL);
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
    ///     .number(phonenumber::parse(None, "+19999999999").unwrap())
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
        let url = format!("{}/evaluate-service", BASE_URL);
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
    ///     let number = phonenumber::parse(None, "+1234567890").unwrap();
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
    use mockito::{Matcher, Server};
    use phonenumber::PhoneNumber;
    use test_generator::test_resources;

    async fn setup_client() -> SendblueClient {
        let server = Server::new();
        let base_url = server.url();
        let api_key = "your_api_key";
        let api_secret = "your_api_secret";

        SendblueClient::new_with_url(api_key.into(), api_secret.into(), base_url)
    }

    #[test_resources("tests/message_case.json")]
    async fn test_send_message(case: &str) {
        let client = setup_client().await;
        let mut server = Server::new();

        let _m = server.mock("POST", "/send-message")
            .with_status(200)
            .with_body(r#"{"account_email": "test@example.com", "content": "Hello, world!", "is_outbound": true, "status": "QUEUED", "message_handle": "handle123", "date_sent": "2023-07-26T12:00:00Z", "date_updated": "2023-07-26T12:00:00Z", "from_number": "+1234567890", "number": "+1234567890"}"#)
            .create();

        let message = MessageBuilder::new(phonenumber::parse(None, case).unwrap())
            .content("Hello, world!".into())
            .build()
            .unwrap();

        let result = tokio_test::block_on(client.send(&message));

        assert!(result.is_ok(), "Failed to send message: {:?}", result.err());
    }

    #[test_resources("tests/group_message_case.json")]
    async fn test_send_group_message(case: &str) {
        let client = setup_client().await;
        let mut server = Server::new();

        let _m = server.mock("POST", "/send-group-message")
            .with_status(200)
            .with_body(r#"{"account_email": "test@example.com", "content": "Hello, group!", "is_outbound": true, "status": "QUEUED", "message_handle": "handle123", "date_sent": "2023-07-26T12:00:00Z", "date_updated": "2023-07-26T12:00:00Z", "from_number": "+1234567890", "numbers": ["+1234567890", "+0987654321"]}"#)
            .create();

        let group_message = MessageBuilder::<GroupMessage>::new_group()
            .numbers(vec![
                phonenumber::parse(None, case).unwrap(),
                phonenumber::parse(None, "+0987654321").unwrap(),
            ])
            .content("Hello, group!".into())
            .build()
            .unwrap();

        let result = tokio_test::block_on(client.send(&group_message));

        assert!(
            result.is_ok(),
            "Failed to send group message: {:?}",
            result.err()
        );
    }

    #[test_resources("tests/get_messages_case.json")]
    async fn test_get_messages(case: &str) {
        let client = setup_client().await;
        let mut server = Server::new();

        let _m = server.mock("GET", "/accounts/messages")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_body(r#"{"messages": [{"date": "2023-08-15T16:04:38.866Z", "content": "Hey", "number": "+12345678912", "status": "QUEUED"}]}"#)
            .create();

        let params = GetMessagesParamsBuilder::new()
            .limit(Some(50))
            .offset(Some(0))
            .number(Some(phonenumber::parse(None, case).unwrap()))
            .from_date(Some("2023-06-15 12:00:00".into()))
            .cid(None)
            .build();

        let result = tokio_test::block_on(client.get_messages(params));

        assert!(result.is_ok(), "Failed to get messages: {:?}", result.err());
    }

    #[test_resources("tests/typing_indicator_case.json")]
    async fn test_send_typing_indicator(case: &str) {
        let client = setup_client().await;
        let mut server = Server::new();

        let _m = server
            .mock("POST", "/send-typing-indicator")
            .with_status(200)
            .with_body(r#"{"number": "+1234567890", "status": "SENT", "error_message": null}"#)
            .create();

        let number = phonenumber::parse(None, case).unwrap();
        let result = tokio_test::block_on(client.send_typing_indicator(&number));

        assert!(
            result.is_ok(),
            "Failed to send typing indicator: {:?}",
            result.err()
        );
    }

    #[test_resources("tests/evaluate_service_case.json")]
    async fn test_evaluate_service(case: &str) {
        let client = setup_client().await;
        let mut server = Server::new();

        let _m = server
            .mock("GET", "/evaluate-service")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_body(r#"{"number": "+19999999999", "service": "iMessage"}"#)
            .create();

        let evaluate_service = EvaluateServiceBuilder::new()
            .number(phonenumber::parse(None, case).unwrap())
            .build();

        let result = tokio_test::block_on(client.evaluate_service(&evaluate_service));

        assert!(
            result.is_ok(),
            "Failed to evaluate service: {:?}",
            result.err()
        );
    }
}
