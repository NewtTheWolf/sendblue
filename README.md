[![crates.io](https://img.shields.io/crates/v/sendblue.svg)](https://crates.io/crates/sendblue)
[![docs | sendblue](https://img.shields.io/badge/docs-sendblue-blue)](https://docs.rs/sendblue)
![Build Status](https://github.com/NewtTheWolf/sendblue-rs/actions/workflows/release.yml/badge.svg)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/sendblue.svg)](https://crates.io/crates/sendblue)

# sendblue

Sendblue API Client

This module provides a client for interacting with the Sendblue API, including methods for sending messages, retrieving messages, and evaluating phone numbers.

## Overview

The Sendblue API allows you to send messages, retrieve message histories, and evaluate phone numbers for their ability to use iMessage. This module encapsulates these functionalities in a user-friendly Rust client.

## Features

- **Send Messages**: Send single or group messages using the Sendblue API.
- **Retrieve Messages**: Fetch message histories with filtering and pagination options.
- **Evaluate Phone Numbers**: Check if a phone number can send/receive iMessages.
- **Typing Indicators**: Send typing indicators to recipients.

## Installation

Use the cargo add command:

```sh
cargo add sendblue
```

## Usage

To use the Sendblue API client, create an instance of `SendblueClient` with your API key and secret.

```rust
use sendblue::SendblueClient;

let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());
```

## Examples

### Sending a Message

```rust
use sendblue::SendblueClient;
use sendblue::models::MessageBuilder;

#[tokio::main]
async fn main() {
    let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());

    let message = MessageBuilder::new(phonenumber::parse(None, "+1234567890").unwrap())
        .content("Hello, world!".into())
        .build()
        .unwrap();

    match client.send(&message).await {
        Ok(response) => println!("Message sent: {:?}", response),
        Err(e) => eprintln!("Error sending message: {:?}", e),
    }
}
```

### Retrieving Messages

```rust
use sendblue::SendblueClient;
use sendblue::models::GetMessagesParamsBuilder;

#[tokio::main]
async fn main() {
    let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());

    let params = GetMessagesParamsBuilder::new()
        .limit(Some(50))
        .offset(Some(0))
        .number(Some(phonenumber::parse(None, "+12345678912").unwrap()))
        .from_date(Some("2023-06-15 12:00:00".into()))
        .build();

    match client.get_messages(params).await {
        Ok(response) => println!("Messages retrieved: {:?}", response.messages),
        Err(e) => eprintln!("Error retrieving messages: {:?}", e),
    }
}
```

### Evaluating a Phone Number

```rust
use sendblue::SendblueClient;
use sendblue::models::EvaluateServiceBuilder;

#[tokio::main]
async fn main() {
    let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());

    let evaluate_service = EvaluateServiceBuilder::new()
        .number(phonenumber::parse(None, "+19999999999").unwrap())
        .build();

    match client.evaluate_service(&evaluate_service).await {
        Ok(response) => println!("Evaluation result: {:?}", response),
        Err(e) => eprintln!("Error evaluating number: {:?}", e),
    }
}
```

### Sending a Typing Indicator

```rust
use sendblue::SendblueClient;

#[tokio::main]
async fn main() {
    let client = SendblueClient::new("your_api_key".into(), "your_api_secret".into());

    let number = phonenumber::parse(None, "+1234567890").unwrap();

    match client.send_typing_indicator(&number).await {
        Ok(response) => println!("Typing indicator sent: {:?}", response),
        Err(e) => eprintln!("Error sending typing indicator: {:?}", e),
    }
}
```

## Contribution

We welcome contributions! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for more details on how to contribute to this project.

## License

This project is dual-licensed under either of the following licenses, at your option:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

## Contact

For any questions or issues, please contact: dominik@spitzli.dev
