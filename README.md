# SendBlue

SendBlue is a Rust library that provides an API client for interacting with the SendBlue REST API, enabling businesses to integrate iMessage and SMS services into their applications.

## Table of Contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Requirements](#requirements)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Description

SendBlue offers a robust API client to connect and manage SendBlue's iMessage and SMS messaging services from within Rust applications. It supports sending messages, handling callbacks, automating workflows, and various other messaging-related functionalities.

## Installation

To use SendBlue in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
sendblue = "1.0.0"
```

Then, run `cargo build` to download and compile the library.

## Usage

Import the necessary modules in your Rust code:

```rust
extern crate sendblue;
use sendblue::prelude::*;
```

### Sending a Message

Here's an example of how to send a message using SendBlue:

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

## Requirements

- Rust 1.79 or later

## Contributing

We welcome contributions! Please read our [contributing guidelines](CONTRIBUTING) for more details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

If you have any questions or feedback, feel free to contact me at [dominik@spitzli.dev].
