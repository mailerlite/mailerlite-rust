<a href="https://www.mailerlite.com"><img src="https://app.mailerlite.com/assets/images/logo-color.png" width="200px"/></a>

MailerLite Rust SDK

![Build](https://img.shields.io/github/actions/workflow/status/mailerlite/mailerlite-rust/main.yaml)
![Downloads](https://img.shields.io/crates/d/mailerlite-rs?label=downloads)
![Crates.io](https://img.shields.io/crates/v/mailerlite-rs)
![License](https://img.shields.io/crates/l/mailerlite-rs)

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
        - [Cancel a ready campaign](#cancel-a-ready-campaign)
        - [Get subscribers activity of sent campaign](#get-subscribers-activity-of-sent-campaign)
    - [Groups](#groups)
        - [Get a list of groups](#get-a-list-of-groups)
        - [Create a group](#create-a-group)
        - [Update a group](#update-a-group)
        - [Delete a group](#delete-a-group)
        - [Get subscribers from a group](#get-subscribers-from-a-group)
        - [Assign subscriber to a group](#assign-subscriber-to-a-group)
        - [Unassign subscriber from a group](#unassign-subscriber-from-a-group)
    - [Segments](#segments)
        - [Get a list of segments](#get-a-list-of-segments)
        - [Update a segment](#update-a-segment)
        - [Delete a segment](#delete-a-segment)
        - [Get subscribers from a segment](#get-subscribers-from-a-segment)
    - [Fields](#fields)
        - [Get a list of fields](#get-a-list-of-fields)
        - [Create a field](#create-a-field)
        - [Update a field](#update-a-field)
        - [Delete a field](#delete-a-field)
    - [Forms](#forms)
        - [Get a list of forms](#get-a-list-of-forms)
        - [Get a form](#get-a-form)
        - [Update a form](#update-a-form)
        - [Delete a form](#delete-a-form)
        - [Get subscribers who signed up to a specific form](#get-subscribers-who-signed-up-to-a-specific-form)
    - [Automations](#automations)
        - [Get a list of automations](#get-a-list-of-automations)
        - [Get a automation](#get-a-automation)
        - [Get the subscribers activity for an automation](#get-the-subscribers-activity-for-an-automation)
    - [Webhooks](#webhooks)
        - [Get a list of webhooks](#get-a-list-of-webhooks)
        - [Get a webhook](#get-a-webhook)
        - [Create a webhook](#create-a-webhook)
        - [Update a webhook](#update-a-webhook)
        - [Delete a webhook](#delete-a-webhook)
    - [Batch](#batch)
        - [Create a batch](#create-a-batch)
    - [Timezones](#timezones)
        - [Get a list of timezones](#get-a-list-of-timezones)
    - [Campaign languages](#campaign-languages)
        - [Get a list of campaign languages](#get-a-list-of-campaign-languages)

# Installation

Run the following Cargo command in your project directory:

```shell
cargo add mailerlite-rs
```

Or add the following line to your Cargo.toml:

```shell
mailerlite-rs = "1.0.0"
```

# Usage

## Subscribers

### Get a list of subscribers

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_subscibers
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

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
cargo run --package mailerlite-rs --example find_subsciber
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

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
cargo run --package mailerlite-rs --example create_subsciber
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new().add("email", "john@gmail.com");

    let response: Response = mailerlite.subscriber().create(data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a subscriber

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_subsciber
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Subscriber ID");

    let data: Data = Data::new()
        .add("fields[name]", "John")
        .add("fields[last_name]", "Doe");

    let response: Response = mailerlite.subscriber().update(id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a subscriber

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_subsciber
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

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
cargo run --package mailerlite-rs --example get_campaigns
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

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
cargo run --package mailerlite-rs --example find_campaign
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

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
cargo run --package mailerlite-rs --example create_campaign
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new()
        .add("name", "Regular Campaign")
        .add("type", "regular")
        .add("emails[0][subject]", "Test Subject")
        .add("emails[0][from_name]", "John Doe")
        .add("emails[0][from]", "john@gmail.com");

    let response: Response = mailerlite.campaign().create(data.clone()).await;

    println!("{:#?}", response);
}
```


### Update a campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_campaign
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let data: Data = Data::new()
        .add("name", "Regular Campaign")
        .add("emails[0][subject]", "Test Subject")
        .add("emails[0][from_name]", "John Doe")
        .add("emails[0][from]", "john@gmail.com");

    let response: Response = mailerlite.campaign().update(id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_campaign
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

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
cargo run --package mailerlite-rs --example schedule_campaign
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let data: Data = Data::new().add("delivery", "instant");

    let response: Response = mailerlite.campaign().schedule(id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Cancel a ready campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example cancel_campaign
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let response: Response = mailerlite.campaign().cancel(id).await;

    println!("{:#?}", response);
}
```

### Get subscribers activity of sent campaign

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_subscribers_activity_for_campaign
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let parameter: Parameter = Parameter::new().add("filter[type]", "opened");

    let response: Response = mailerlite.campaign().subscribers_activity(id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

## Groups

### Get a list of groups

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_groups
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("limit", "10");

    let response: Response = mailerlite.group().get(parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Create a group

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_group
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new().add("name", "Dummy Group");

    let response: Response = mailerlite.group().create(data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a group

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_group
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Group ID");

    let data: Data = Data::new().add("name", "Dummy Group");

    let response: Response = mailerlite.group().update(id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a group

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_group
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Group ID");

    let response: Response = mailerlite.group().delete(id).await;

    println!("{:#?}", response);
}
```

### Get subscribers from a group

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_group_subscribers
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Group ID");

    let parameter: Parameter = Parameter::new().add("filter[status]", "unsubscribed");

    let response: Response = mailerlite.group().get_subscribers(id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Assign subscriber to a group

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example assign_subscriber_to_group
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let group_id: String = String::from("Your Group ID");

    let subscriber_id: String = String::from("Your Subscriber ID");

    let response: Response = mailerlite.group().assign_subscriber(group_id, subscriber_id).await;

    println!("{:#?}", response);
}
```

### Unassign subscriber from a group

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example unassign_subscriber_from_group
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let group_id: String = String::from("Your Group ID");

    let subscriber_id: String = String::from("Your Subscriber ID");

    let response: Response = mailerlite.group().unassign_subscriber(group_id, subscriber_id).await;

    println!("{:#?}", response);
}
```

## Segments

### Get a list of segments

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_segments
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("page", "1");

    let response: Response = mailerlite.segment().get(parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Update a segment

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_segment
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Segment ID");

    let data: Data = Data::new().add("name", "Dummy Segment");

    let response: Response = mailerlite.segment().update(id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a segment

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_segment
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Segment ID");

    let response: Response = mailerlite.segment().delete(id).await;

    println!("{:#?}", response);
}
```

### Get subscribers from a segment

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_segment_subscribers
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Segment ID");

    let parameter: Parameter = Parameter::new().add("filter[status]", "unsubscribed");

    let response: Response = mailerlite.segment().get_subscribers(id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

## Fields

### Get a list of fields

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_fields
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("filter[type]", "number");

    let response: Response = mailerlite.field().get(parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Create a field

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_field
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new().add("name", "Dummy Field").add("type", "text");

    let response: Response = mailerlite.field().create(data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a field

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_field
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Field ID");

    let data: Data = Data::new().add("name", "Dummy Field");

    let response: Response = mailerlite.field().update(id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a field

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_field
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Field ID");

    let response: Response = mailerlite.field().delete(id).await;

    println!("{:#?}", response);
}
```

## Forms

### Get a list of forms

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_forms
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let form_type: String = String::from("Your Form Type");

    let parameter: Parameter = Parameter::new().add("sort", "created_at");

    let response: Response = mailerlite.form().get(form_type, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a form

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_form
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Form ID");

    let response: Response = mailerlite.form().find(id).await;

    println!("{:#?}", response);
}
```

### Update a form

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_form
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Group ID");

    let data: Data = Data::new().add("name", "Dummy Group");

    let response: Response = mailerlite.group().update(id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a form

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_form
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Form ID");

    let response: Response = mailerlite.form().delete(id).await;

    println!("{:#?}", response);
}
```

### Get subscribers who signed up to a specific form

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_form_subscribers
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Form ID");

    let parameter: Parameter = Parameter::new().add("filter[status]", "unsubscribed");

    let response: Response = mailerlite.form().get_subscribers(id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

## Automations

### Get a list of automations

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_automations
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("filter[enabled]", "true");

    let response: Response = mailerlite.automation().get(parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a automation

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_automation
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let response: Response = mailerlite.automation().find(id).await;

    println!("{:#?}", response);
}
```

### Get the subscribers activity for an automation

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_subscribers_activity_for_automation
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let parameter: Parameter = Parameter::new().add("filter[status]", "completed");

    let response: Response = mailerlite.automation().subscribers_activity(id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

## Webhooks

### Get a list of webhooks

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_webhooks
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new();

    let response: Response = mailerlite.webhook().get(parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a webhook

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_webhook
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Webhook ID");

    let response: Response = mailerlite.webhook().find(id).await;

    println!("{:#?}", response);
}
```

### Create a webhook

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_webhook
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new()
        .add("name", "Dummy Webhook")
        .add("events[]", "subscriber.created")
        .add("url", "https://www.cartwright.info/eligendi-soluta-corporis-in-quod-ullam");

    let response: Response = mailerlite.webhook().create(data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a webhook

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_webhook
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Webhook ID");

    let data: Data = Data::new().add("name", "Dummy Webhook");

    let response: Response = mailerlite.webhook().update(id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a webhook

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_webhook
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Webhook ID");

    let response: Response = mailerlite.webhook().delete(id).await;

    println!("{:#?}", response);
}
```

## Batch

### Create a batch

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_batch
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new()
        .add("requests[0][method]", "POST")
        .add("requests[0][path]", "api/subscribers")
        .add("requests[0][body][email]", "dummy@example.com");

    let response: Response = mailerlite.batch().create(data.clone()).await;

    println!("{:#?}", response);
}
```

## Timezones

### Get a list of timezones

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_timezones
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let response: Response = mailerlite.timezone().get().await;

    println!("{:#?}", response);
}
```

## Campaign languages

### Get a list of campaign languages

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_campaign_languages
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let response: Response = mailerlite.campaign().languages().await;

    println!("{:#?}", response);
}
```
