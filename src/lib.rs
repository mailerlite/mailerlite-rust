use automation::Automation;
use batch::Batch;
use campaign::Campaign;
use client::Client;
use ecommerce_cart::EcommerceCart;
use ecommerce_cart_item::EcommerceCartItem;
use ecommerce_category::EcommerceCategory;
use ecommerce_customer::EcommerceCustomer;
use ecommerce_import::EcommerceImport;
use ecommerce_order::EcommerceOrder;
use ecommerce_product::EcommerceProduct;
use ecommerce_product_variant::EcommerceProductVariant;
use ecommerce_setting::EcommerceSetting;
use ecommerce_shop::EcommerceShop;
use field::Field;
use form::Form;
use group::Group;
use segment::Segment;
use subscriber::Subscriber;
use timezone::Timezone;
use webhook::Webhook;

pub mod api_response;
pub mod automation;
pub mod batch;
pub mod campaign;
pub mod client;
pub mod data;
pub mod ecommerce_cart;
pub mod ecommerce_cart_item;
pub mod ecommerce_category;
pub mod ecommerce_customer;
pub mod ecommerce_import;
pub mod ecommerce_order;
pub mod ecommerce_product;
pub mod ecommerce_product_variant;
pub mod ecommerce_setting;
pub mod ecommerce_shop;
pub mod error;
pub mod field;
pub mod form;
pub mod group;
pub mod parameter;
pub mod response;
pub mod segment;
pub mod subscriber;
pub mod timezone;
pub mod webhook;

const BASE_PATH: &str = "https://connect.mailerlite.com/api/";

#[derive(Debug, Clone)]
pub struct MailerLite {
    client: Client,
    base_url: String,
}

impl MailerLite {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(api_key),
            base_url: BASE_PATH.to_string(),
        }
    }

    /// Sends all requests to a different base URL, for example a mock server or a development instance of
    /// the API. The SDK adds a "/" at the end if the URL has none.
    ///
    /// ```
    /// use mailerlite_rs::MailerLite;
    ///
    /// let mailerlite: MailerLite =
    ///     MailerLite::new("api_key".to_string()).with_base_url("http://localhost:8080/api");
    /// ```
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        let mut base_url: String = base_url.into();
        if !base_url.ends_with('/') {
            base_url.push('/');
        }
        self.base_url = base_url;

        self
    }

    pub fn subscriber(&self) -> Subscriber {
        Subscriber::new(self.clone())
    }

    pub fn campaign(&self) -> Campaign {
        Campaign::new(self.clone())
    }

    pub fn group(&self) -> Group {
        Group::new(self.clone())
    }

    pub fn segment(&self) -> Segment {
        Segment::new(self.clone())
    }

    pub fn field(&self) -> Field {
        Field::new(self.clone())
    }

    pub fn form(&self) -> Form {
        Form::new(self.clone())
    }

    pub fn automation(&self) -> Automation {
        Automation::new(self.clone())
    }

    pub fn webhook(&self) -> Webhook {
        Webhook::new(self.clone())
    }

    pub fn batch(&self) -> Batch {
        Batch::new(self.clone())
    }

    pub fn timezone(&self) -> Timezone {
        Timezone::new(self.clone())
    }

    pub fn ecommerce_shop(&self) -> EcommerceShop {
        EcommerceShop::new(self.clone())
    }

    pub fn ecommerce_product(&self) -> EcommerceProduct {
        EcommerceProduct::new(self.clone())
    }

    pub fn ecommerce_product_variant(&self) -> EcommerceProductVariant {
        EcommerceProductVariant::new(self.clone())
    }

    pub fn ecommerce_category(&self) -> EcommerceCategory {
        EcommerceCategory::new(self.clone())
    }

    pub fn ecommerce_customer(&self) -> EcommerceCustomer {
        EcommerceCustomer::new(self.clone())
    }

    pub fn ecommerce_order(&self) -> EcommerceOrder {
        EcommerceOrder::new(self.clone())
    }

    pub fn ecommerce_cart(&self) -> EcommerceCart {
        EcommerceCart::new(self.clone())
    }

    pub fn ecommerce_cart_item(&self) -> EcommerceCartItem {
        EcommerceCartItem::new(self.clone())
    }

    pub fn ecommerce_import(&self) -> EcommerceImport {
        EcommerceImport::new(self.clone())
    }

    pub fn ecommerce_setting(&self) -> EcommerceSetting {
        EcommerceSetting::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_output_hides_api_key() {
        let mailerlite: MailerLite = MailerLite::new("secret_api_key".to_string());

        assert!(!format!("{mailerlite:?}").contains("secret_api_key"));
    }

    #[test]
    fn base_url_defaults_to_the_api() {
        let mailerlite: MailerLite = MailerLite::new("api_key".to_string());

        assert_eq!(mailerlite.base_url, "https://connect.mailerlite.com/api/");
    }

    #[test]
    fn with_base_url_adds_one_trailing_slash() {
        let mailerlite: MailerLite =
            MailerLite::new("api_key".to_string()).with_base_url("http://localhost:8080/api");
        assert_eq!(mailerlite.base_url, "http://localhost:8080/api/");

        let mailerlite: MailerLite =
            MailerLite::new("api_key".to_string()).with_base_url("http://localhost:8080/api/");
        assert_eq!(mailerlite.base_url, "http://localhost:8080/api/");
    }
}
