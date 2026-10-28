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
    - [E-commerce shops](#e-commerce-shops)
        - [Get a list of shops](#get-a-list-of-shops)
        - [Get a shop](#get-a-shop)
        - [Create a shop](#create-a-shop)
        - [Update a shop](#update-a-shop)
        - [Delete a shop](#delete-a-shop)
        - [Get shop stats](#get-shop-stats)
    - [E-commerce products](#e-commerce-products)
        - [Get a list of products](#get-a-list-of-products)
        - [Get a product](#get-a-product)
        - [Create a product](#create-a-product)
        - [Update a product](#update-a-product)
        - [Delete a product](#delete-a-product)
        - [Enable or disable a product](#enable-or-disable-a-product)
        - [Assign categories to a product](#assign-categories-to-a-product)
    - [E-commerce product variants](#e-commerce-product-variants)
        - [Get a list of product variants](#get-a-list-of-product-variants)
        - [Sync product variants](#sync-product-variants)
        - [Update a product variant](#update-a-product-variant)
        - [Delete a product variant](#delete-a-product-variant)
    - [E-commerce categories](#e-commerce-categories)
        - [Get a list of categories](#get-a-list-of-categories)
        - [Get a category](#get-a-category)
        - [Create a category](#create-a-category)
        - [Update a category](#update-a-category)
        - [Delete a category](#delete-a-category)
        - [Get a list of category products](#get-a-list-of-category-products)
        - [Assign a product to a category](#assign-a-product-to-a-category)
        - [Unassign a product from a category](#unassign-a-product-from-a-category)
    - [E-commerce customers](#e-commerce-customers)
        - [Get a list of customers](#get-a-list-of-customers)
        - [Get a customer](#get-a-customer)
        - [Create a customer](#create-a-customer)
        - [Update a customer](#update-a-customer)
        - [Delete a customer](#delete-a-customer)
    - [E-commerce orders](#e-commerce-orders)
        - [Get a list of orders](#get-a-list-of-orders)
        - [Get an order](#get-an-order)
        - [Create an order](#create-an-order)
        - [Update an order](#update-an-order)
        - [Delete an order](#delete-an-order)
    - [E-commerce carts](#e-commerce-carts)
        - [Get a list of carts](#get-a-list-of-carts)
        - [Get a cart](#get-a-cart)
        - [Update a cart](#update-a-cart)
    - [E-commerce cart items](#e-commerce-cart-items)
        - [Get a list of cart items](#get-a-list-of-cart-items)
        - [Get a cart item](#get-a-cart-item)
        - [Create a cart item](#create-a-cart-item)
        - [Update a cart item](#update-a-cart-item)
        - [Delete a cart item](#delete-a-cart-item)
    - [E-commerce imports](#e-commerce-imports)
        - [Import products](#import-products)
        - [Import categories](#import-categories)
        - [Import customers](#import-customers)
        - [Import orders](#import-orders)
    - [E-commerce settings](#e-commerce-settings)
        - [Get e-commerce settings](#get-e-commerce-settings)
    - [E-commerce reports](#e-commerce-reports)
        - [Get campaign e-commerce stats](#get-campaign-e-commerce-stats)
        - [Get campaign e-commerce orders](#get-campaign-e-commerce-orders)
        - [Get subscriber e-commerce stats](#get-subscriber-e-commerce-stats)
        - [Get subscriber e-commerce orders](#get-subscriber-e-commerce-orders)
        - [Get automation e-commerce stats](#get-automation-e-commerce-stats)
        - [Get automation email e-commerce stats](#get-automation-email-e-commerce-stats)
        - [Get automation e-commerce orders](#get-automation-e-commerce-orders)

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

## E-commerce shops

### Get a list of shops

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_shops
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("limit", "25");

    let response: Response = mailerlite.ecommerce_shop().get(parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a shop

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_ecommerce_shop
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let response: Response = mailerlite.ecommerce_shop().find(shop_id).await;

    println!("{:#?}", response);
}
```

### Create a shop

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_ecommerce_shop
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new()
        .add("name", "Dummy Shop")
        .add("url", "https://shop.example.com")
        .add("currency", "EUR");

    let response: Response = mailerlite.ecommerce_shop().create(data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a shop

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_ecommerce_shop
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("name", "Dummy Shop");

    let response: Response = mailerlite.ecommerce_shop().update(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a shop

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_ecommerce_shop
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let response: Response = mailerlite.ecommerce_shop().delete(shop_id).await;

    println!("{:#?}", response);
}
```

### Get shop stats

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_shop_stats
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let start_date: String = String::from("2026-10-01");

    let end_date: String = String::from("2026-10-31");

    let response: Response = mailerlite.ecommerce_shop().stats(shop_id, start_date, end_date).await;

    println!("{:#?}", response);
}
```

## E-commerce products

### Get a list of products

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_products
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let parameter: Parameter = Parameter::new().add("filter[enabled]", "true");

    let response: Response = mailerlite.ecommerce_product().get(shop_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a product

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_ecommerce_product
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let response: Response = mailerlite.ecommerce_product().find(shop_id, product_id).await;

    println!("{:#?}", response);
}
```

### Create a product

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_ecommerce_product
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("name", "Dummy Product")
        .add("price", "19.99")
        .add("url", "https://shop.example.com/dummy-product");

    let response: Response = mailerlite.ecommerce_product().create(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a product

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_ecommerce_product
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let data: Data = Data::new()
        .add("price", "24.99");

    let response: Response = mailerlite.ecommerce_product().update(shop_id, product_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a product

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_ecommerce_product
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let response: Response = mailerlite.ecommerce_product().delete(shop_id, product_id).await;

    println!("{:#?}", response);
}
```

### Enable or disable a product

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example change_ecommerce_product_status
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let data: Data = Data::new()
        .add("status", "0");

    let response: Response = mailerlite.ecommerce_product().change_status(shop_id, product_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Assign categories to a product

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example assign_categories_to_ecommerce_product
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let data: Data = Data::new()
        .add("categories[]", "Your Category ID")
        .add("replace", "0");

    let response: Response = mailerlite.ecommerce_product().assign_categories(shop_id, product_id, data.clone()).await;

    println!("{:#?}", response);
}
```

## E-commerce product variants

### Get a list of product variants

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_product_variants
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let response: Response = mailerlite.ecommerce_product_variant().get(shop_id, product_id).await;

    println!("{:#?}", response);
}
```

### Sync product variants

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example sync_ecommerce_product_variants
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let data: Data = Data::new()
        .add("0[name]", "Large")
        .add("0[attributes][size]", "L")
        .add("0[resource_id]", "variant-l");

    let response: Response = mailerlite.ecommerce_product_variant().sync(shop_id, product_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a product variant

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_ecommerce_product_variant
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let variant_id: String = String::from("Your Variant ID");

    let data: Data = Data::new()
        .add("name", "Extra large")
        .add("attributes[size]", "XL");

    let response: Response = mailerlite.ecommerce_product_variant().update(shop_id, product_id, variant_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a product variant

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_ecommerce_product_variant
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let variant_id: String = String::from("Your Variant ID");

    let response: Response = mailerlite.ecommerce_product_variant().delete(shop_id, product_id, variant_id).await;

    println!("{:#?}", response);
}
```

## E-commerce categories

### Get a list of categories

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_categories
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let parameter: Parameter = Parameter::new().add("limit", "25");

    let response: Response = mailerlite.ecommerce_category().get(shop_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a category

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_ecommerce_category
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let category_id: String = String::from("Your Category ID");

    let response: Response = mailerlite.ecommerce_category().find(shop_id, category_id).await;

    println!("{:#?}", response);
}
```

### Create a category

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_ecommerce_category
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("name", "Dummy Category");

    let response: Response = mailerlite.ecommerce_category().create(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a category

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_ecommerce_category
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let category_id: String = String::from("Your Category ID");

    let data: Data = Data::new()
        .add("name", "Dummy Category");

    let response: Response = mailerlite.ecommerce_category().update(shop_id, category_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a category

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_ecommerce_category
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let category_id: String = String::from("Your Category ID");

    let response: Response = mailerlite.ecommerce_category().delete(shop_id, category_id).await;

    println!("{:#?}", response);
}
```

### Get a list of category products

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_category_products
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let category_id: String = String::from("Your Category ID");

    let parameter: Parameter = Parameter::new().add("limit", "25");

    let response: Response = mailerlite.ecommerce_category().products(shop_id, category_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Assign a product to a category

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example assign_product_to_ecommerce_category
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let category_id: String = String::from("Your Category ID");

    let product_id: String = String::from("Your Product ID");

    let response: Response = mailerlite.ecommerce_category().assign_product(shop_id, category_id, product_id).await;

    println!("{:#?}", response);
}
```

### Unassign a product from a category

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example unassign_product_from_ecommerce_category
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let category_id: String = String::from("Your Category ID");

    let product_id: String = String::from("Your Product ID");

    let response: Response = mailerlite.ecommerce_category().unassign_product(shop_id, category_id, product_id).await;

    println!("{:#?}", response);
}
```

## E-commerce customers

### Get a list of customers

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_customers
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let parameter: Parameter = Parameter::new().add("filter[email]", "dummy@example.com");

    let response: Response = mailerlite.ecommerce_customer().get(shop_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a customer

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_ecommerce_customer
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let customer_id: String = String::from("Your Customer ID");

    let response: Response = mailerlite.ecommerce_customer().find(shop_id, customer_id).await;

    println!("{:#?}", response);
}
```

### Create a customer

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_ecommerce_customer
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("email", "dummy@example.com")
        .add("create_subscriber", "1");

    let response: Response = mailerlite.ecommerce_customer().create(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a customer

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_ecommerce_customer
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let customer_id: String = String::from("Your Customer ID");

    let data: Data = Data::new()
        .add("accepts_marketing", "1");

    let response: Response = mailerlite.ecommerce_customer().update(shop_id, customer_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a customer

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_ecommerce_customer
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let customer_id: String = String::from("Your Customer ID");

    let response: Response = mailerlite.ecommerce_customer().delete(shop_id, customer_id).await;

    println!("{:#?}", response);
}
```

## E-commerce orders

### Get a list of orders

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_orders
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let parameter: Parameter = Parameter::new().add("filter[attributed]", "true");

    let response: Response = mailerlite.ecommerce_order().get(shop_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get an order

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_ecommerce_order
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let order_id: String = String::from("Your Order ID");

    let response: Response = mailerlite.ecommerce_order().find(shop_id, order_id).await;

    println!("{:#?}", response);
}
```

### Create an order

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_ecommerce_order
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("customer[email]", "dummy@example.com")
        .add("customer[create_subscriber]", "1")
        .add("cart[items][0][ecommerce_product_id]", "Your Product ID")
        .add("cart[items][0][quantity]", "1")
        .add("cart[items][0][price]", "19.99")
        .add("status", "complete");

    let response: Response = mailerlite.ecommerce_order().create(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Update an order

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_ecommerce_order
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let order_id: String = String::from("Your Order ID");

    let data: Data = Data::new()
        .add("status", "complete");

    let response: Response = mailerlite.ecommerce_order().update(shop_id, order_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete an order

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_ecommerce_order
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let order_id: String = String::from("Your Order ID");

    let response: Response = mailerlite.ecommerce_order().delete(shop_id, order_id).await;

    println!("{:#?}", response);
}
```

## E-commerce carts

### Get a list of carts

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_carts
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let parameter: Parameter = Parameter::new().add("limit", "25");

    let response: Response = mailerlite.ecommerce_cart().get(shop_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a cart

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_ecommerce_cart
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let response: Response = mailerlite.ecommerce_cart().find(shop_id, cart_id).await;

    println!("{:#?}", response);
}
```

### Update a cart

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_ecommerce_cart
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let data: Data = Data::new()
        .add("checkout_url", "https://shop.example.com/cart");

    let response: Response = mailerlite.ecommerce_cart().update(shop_id, cart_id, data.clone()).await;

    println!("{:#?}", response);
}
```

## E-commerce cart items

### Get a list of cart items

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_ecommerce_cart_items
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let parameter: Parameter = Parameter::new().add("limit", "25");

    let response: Response = mailerlite.ecommerce_cart_item().get(shop_id, cart_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get a cart item

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_ecommerce_cart_item
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let item_id: String = String::from("Your Item ID");

    let response: Response = mailerlite.ecommerce_cart_item().find(shop_id, cart_id, item_id).await;

    println!("{:#?}", response);
}
```

### Create a cart item

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example create_ecommerce_cart_item
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let data: Data = Data::new()
        .add("ecommerce_product_id", "Your Product ID")
        .add("quantity", "1")
        .add("price", "19.99");

    let response: Response = mailerlite.ecommerce_cart_item().create(shop_id, cart_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Update a cart item

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example update_ecommerce_cart_item
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let item_id: String = String::from("Your Item ID");

    let data: Data = Data::new()
        .add("quantity", "2");

    let response: Response = mailerlite.ecommerce_cart_item().update(shop_id, cart_id, item_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Delete a cart item

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example delete_ecommerce_cart_item
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let item_id: String = String::from("Your Item ID");

    let response: Response = mailerlite.ecommerce_cart_item().delete(shop_id, cart_id, item_id).await;

    println!("{:#?}", response);
}
```

## E-commerce imports

### Import products

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example import_ecommerce_products
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("0[name]", "Dummy Product")
        .add("0[price]", "19.99")
        .add("0[url]", "https://shop.example.com/dummy-product");

    let response: Response = mailerlite.ecommerce_import().products(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Import categories

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example import_ecommerce_categories
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("0[name]", "Dummy Category")
        .add("0[exclude_from_automations]", "0");

    let response: Response = mailerlite.ecommerce_import().categories(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Import customers

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example import_ecommerce_customers
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("0[email]", "dummy@example.com")
        .add("0[accepts_marketing]", "1");

    let response: Response = mailerlite.ecommerce_import().customers(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
```

### Import orders

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example import_ecommerce_orders
```
</details>

```rust
use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("orders[0][customer][email]", "dummy@example.com")
        .add("orders[0][cart][items][0][ecommerce_product_id]", "Your Product ID")
        .add("orders[0][cart][items][0][quantity]", "1")
        .add("orders[0][cart][items][0][price]", "19.99")
        .add("orders[0][status]", "complete");

    let response: Response = mailerlite.ecommerce_import().orders(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
```

## E-commerce settings

### Get e-commerce settings

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example find_ecommerce_settings
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let response: Response = mailerlite.ecommerce_setting().find().await;

    println!("{:#?}", response);
}
```

## E-commerce reports

### Get campaign e-commerce stats

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_campaign_ecommerce_stats
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let shop_id: String = String::from("Your Shop ID");

    let response: Response = mailerlite.campaign().ecommerce_stats(id, shop_id).await;

    println!("{:#?}", response);
}
```

### Get campaign e-commerce orders

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_campaign_ecommerce_orders
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let shop_id: String = String::from("Your Shop ID");

    let parameter: Parameter = Parameter::new().add("page", "1");

    let response: Response = mailerlite.campaign().ecommerce_orders(id, shop_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get subscriber e-commerce stats

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_subscriber_ecommerce_stats
```
</details>

```rust
use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Subscriber ID");

    let shop_id: String = String::from("Your Shop ID");

    let response: Response = mailerlite.subscriber().ecommerce_stats(id, shop_id).await;

    println!("{:#?}", response);
}
```

### Get subscriber e-commerce orders

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_subscriber_ecommerce_orders
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Subscriber ID");

    let shop_id: String = String::from("Your Shop ID");

    let parameter: Parameter = Parameter::new().add("page", "1");

    let response: Response = mailerlite.subscriber().ecommerce_orders(id, shop_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get automation e-commerce stats

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_automation_ecommerce_stats
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let parameter: Parameter = Parameter::new();

    let response: Response = mailerlite.automation().ecommerce_stats(id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get automation email e-commerce stats

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_automation_email_ecommerce_stats
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let email_id: String = String::from("Your Email ID");

    let parameter: Parameter = Parameter::new().add("shop_id", "Your Shop ID");

    let response: Response = mailerlite.automation().email_ecommerce_stats(id, email_id, parameter.clone()).await;

    println!("{:#?}", response);
}
```

### Get automation e-commerce orders

<details>
<summary>
You can test out the example by running it with the command provided.
</summary>

```bash
cargo run --package mailerlite-rs --example get_automation_ecommerce_orders
```
</details>

```rust
use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let parameter: Parameter = Parameter::new().add("time_frame", "all_time");

    let response: Response = mailerlite.automation().ecommerce_orders(id, parameter.clone()).await;

    println!("{:#?}", response);
}
```
