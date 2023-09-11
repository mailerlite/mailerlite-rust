<a href="https://www.mailerlite.com"><img src="https://app.mailerlite.com/assets/images/logo-color.png" width="200px"/></a>

MailerLite Rust SDK

[![Run Tests](https://github.com/mailerlite/mailerlite-rust/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/mailerlite/mailerlite-rust/actions/workflows/test.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

# Table of Contents

- [Installation](#installation)
- [Usage](#usage)
    - [Subscribers](#subscribers)
        - [Get a list of subscribers](#get-a-list-of-subscribers)
        - [Get a subscriber](#get-a-subscriber)
        - [Create a subscriber](#create-a-subscriber)
        - [Update a subscriber](#update-a-subscriber)
        - [Delete a subscriber](#delete-a-subscriber)
    - [Campaigns](#campaigns)
        - [Get a list of campaigns](#get-a-list-of-campaigns)
        - [Get a campaign](#get-a-campaign)
        - [Create a campaign](#create-a-campaign)
        - [Update a campaign](#update-a-campaign)
        - [Delete a campaign](#delete-a-campaign)
        - [Schedule a campaign](#schedule-a-campaign)
        - [Cancel a ready campaign](#schedule-a-ready-campaign)

# Installation

Run the following Cargo command in your project directory:

```shell
cargo add mailerlite-rust
```

Or add the following line to your Cargo.toml:

```shell
mailerlite-rust = "0.1.0"
```

# Usage

## Subscribers

### Get a list of subscribers

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example get_subscibers
```
</details>

```rust
use mailerlite_rust::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new()
        .add("filter[status]", "active")
        .add("limit", "10");

    let response: Response = mailerlite.subscriber().get(parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a subscriber

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example find_subsciber
```
</details>

```rust
use mailerlite_rust::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Subscriber ID");

    let response: Response = mailerlite.subscriber().find(id).await;

    println!("{:#?}", response);
}
```

### Create a subscriber

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example create_subsciber
```
</details>

```rust
use mailerlite_rust::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let form: Form = Form::new().add("email", "john@gmail.com");

    let response: Response = mailerlite.subscriber().create(form.clone()).await;

    println!("{:#?}", response);
}
```

### Update a subscriber

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example update_subsciber
```
</details>

```rust
use mailerlite_rust::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Subscriber ID");

    let form: Form = Form::new()
        .add("fields[name]", "John")
        .add("fields[last_name]", "Doe");

    let response: Response = mailerlite.subscriber().update(id, form.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a subscriber

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example delete_subsciber
```
</details>

```rust
use mailerlite_rust::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Subscriber ID");

    let response: Response = mailerlite.subscriber().delete(id).await;

    println!("{:#?}", response);
}
```

## Campaigns

### Get a list of campaigns

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example get_campaigns
```
</details>

```rust
use mailerlite_rust::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("filter[status]", "sent");

    let response: Response = mailerlite.campaign().get(parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example find_campaign
```
</details>

```rust
use mailerlite_rust::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let response: Response = mailerlite.campaign().find(id).await;

    println!("{:#?}", response);
}
```

### Create a campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example create_campaign
```
</details>

```rust
use mailerlite_rust::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let form: Form = Form::new()
        .add("name", "Regular Campaign")
        .add("type", "regular")
        .add("emails[0][subject]", "Test Subject")
        .add("emails[0][from_name]", "John Doe")
        .add("emails[0][from]", "john@gmail.com");

    let response: Response = mailerlite.campaign().create(form.clone()).await;

    println!("{:#?}", response);
}
```


### Update a campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example update_campaign
```
</details>

```rust
use mailerlite_rust::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let form: Form = Form::new()
        .add("name", "Regular Campaign")
        .add("emails[0][subject]", "Test Subject")
        .add("emails[0][from_name]", "John Doe")
        .add("emails[0][from]", "john@gmail.com");

    let response: Response = mailerlite.campaign().update(id, form.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example delete_campaign
```
</details>

```rust
use mailerlite_rust::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let response: Response = mailerlite.campaign().delete(id).await;

    println!("{:#?}", response);
}
```

### Schedule a campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example schedule_campaign
```
</details>

```rust
use mailerlite_rust::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let form: Form = Form::new().add("delivery", "instant");

    let response: Response = mailerlite.campaign().schedule(id, form.clone()).await;

    println!("{:#?}", response);
}
```

### Cancel a ready campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rust --example cancel_campaign
```
</details>

```rust
use mailerlite_rust::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let response: Response = mailerlite.campaign().cancel(id).await;

    println!("{:#?}", response);
}
```
