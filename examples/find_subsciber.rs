use mailerlite_rust::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiN2JjODJjY2Q3MjExMjUyZTlhMjQwYTY0NjIwNWFlODhiM2U0ZWY2MjZmOTk2ZGJiOTI2YjNkZDI0MjBkYTU0Y2U0MTYzMDVjNzMyZjE2MjMiLCJpYXQiOjE2ODAzNjcwMzUuOTQ5NTM3LCJuYmYiOjE2ODAzNjcwMzUuOTQ5NTM5LCJleHAiOjQ4MzYwNDA2MzUuOTM4MjQyLCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.j46H135IkTnI28k5bGEXOezliftNmCjFbZh3DqcCbkL7hN8dxbF57ct-rmql8cXhuvegKZdZzjnW68ghVzo45pGFQ5ngawUaQDXn_RfRCsLiFmYJ7QXDKN7NDLsdZgVfmLN_U9d2fysDmcNTj65ohHLu9V9ACmv2uu8DuotjBwsSq-Bqyvf_Dpj65towRI1ZW_eVwanjjHYXakpgjl2ZvirhnNGtEH9-Tady8G37eSKzqq93jp09-OW54tJdyBbSg6yu8pJgWAGpyFsI_Gaa0hgT8mmTMPtqSLzD3_j6Osit1UuRE1bQdWyM77TYeayS28coyNW3xZUaJ3iKQw6LAIJFtn50UdmlZr13RduVeazABoxuhadgCsxr_c2O8nj7H-VaZCbyBUCReAABMNx1exz2nZO_ZZ6r96wSwfWUHHiXDlGTZUE7I-mKLaE9BVWSY-xYBP0YY1HAmr0-dUNEabRITAJQOHhDqQ2hsD0pIfxa9OjA8pJX8F3gSKmjGsXc5-3ohApL8WveHGSW7AVhYnzY5t8445DhdeOpId8rjXBmfcK7e9wlocW8NpdyC2xnbexpwD1Vh02zV0ryOpBzsZ9MOTD-wrw3MLbnlJ-eSchMyTxPd8InMvsDwd5TN3MS8pxRnCO5Sq3y73hsWK1dqfcP_SYPlsbe8ixu7KezMtY");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new().add("id", "82814769200890897");

    let response: Response = mailerlite.subscriber().find(parameter).await;

    println!("{:#?}", response);
}
