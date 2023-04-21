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
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

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
use mailerlite_rust::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("id", "82814769200890897");

    let response: Response = mailerlite.subscriber().find(parameter).await;

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
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

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
use mailerlite_rust::{form::Form, parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("id", "84294993589568727");

    let form: Form = Form::new()
        .add("fields[name]", "John")
        .add("fields[last_name]", "Doe");

    let response: Response = mailerlite
        .subscriber()
        .update(parameter.clone(), form.clone())
        .await;

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
use mailerlite_rust::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("id", "84294993589568727");

    let response: Response = mailerlite.subscriber().delete(parameter).await;

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
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("status", "sent");

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
use mailerlite_rust::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("id", "84526712216356349");

    let response: Response = mailerlite.campaign().find(parameter).await;

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
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let form: Form = Form::new()
        .add("name", "Regular Campaign")
        .add("type", "regular")
        .add("emails[0][subject]", "Test Subject")
        .add("emails[0][from_name]", "John Doe")
        .add("emails[0][from]", "dummy@mailerlite.dev");

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
use mailerlite_rust::{form::Form, parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("id", "84294993589568727");

    let form: Form = Form::new()
        .add("name", "Regular Campaign")
        .add("emails[0][subject]", "Test Subject")
        .add("emails[0][from_name]", "John Doe")
        .add("emails[0][from]", "john@gmail.com");

    let response: Response = mailerlite
        .campaign()
        .update(parameter.clone(), form.clone())
        .await;

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
use mailerlite_rust::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("id", "84294993589568727");

    let response: Response = mailerlite.campaign().delete(parameter).await;

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
use mailerlite_rust::{form::Form, parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("id", "82814769200890897");

    let form: Form = Form::new().add("delivery", "instant");

    let response: Response = mailerlite
        .campaign()
        .schedule(parameter.clone(), form.clone())
        .await;

    println!("{:#?}", response);
}
```
