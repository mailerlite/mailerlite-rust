use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let automation_id: String = String::from("Your Automation ID");

    let step_id: String = String::from("Your Step ID");

    let parameter: Parameter = Parameter::new().add("branch_to_delete", "all");

    let response: Response = mailerlite
        .automation_step()
        .delete(automation_id, step_id, parameter.clone())
        .await;

    println!("{:#?}", response);
}
