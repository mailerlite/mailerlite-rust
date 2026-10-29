/// An unknown value goes into `Other`, so a new value of the API does not break the decode.
/// Equality and the hash use the value, so `Other("paragraph")` is equal to `Paragraph`.
macro_rules! open_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $( $(#[$variant_meta:meta])* $variant:ident => $value:literal, )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[serde(from = "String", into = "String")]
        #[non_exhaustive]
        pub enum $name {
            $( $(#[$variant_meta])* $variant, )*
            /// A value that this version of the SDK does not know.
            Other(String),
        }

        impl $name {
            /// Returns the value that the API uses.
            pub fn as_str(&self) -> &str {
                match self {
                    $( $name::$variant => $value, )*
                    $name::Other(value) => value,
                }
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $( $value => $name::$variant, )*
                    _ => $name::Other(value),
                }
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $name::Other(value) => value,
                    known => known.as_str().to_string(),
                }
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl Eq for $name {}

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.as_str().hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

use automation::Automation;
use batch::Batch;
use brand_style::BrandStyle;
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
use email::Email;
use field::Field;
use file::File;
use form::Form;
use group::Group;
use segment::Segment;
use subscriber::Subscriber;
use timezone::Timezone;
use webhook::Webhook;

pub mod api_response;
pub mod automation;
pub mod batch;
pub mod brand_style;
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
pub mod email;
pub mod error;
pub mod field;
pub mod file;
pub mod form;
pub mod group;
mod http;
pub mod pagination;
pub mod parameter;
pub mod response;
pub mod segment;
pub mod subscriber;
pub mod timezone;
mod validate;
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
    ///     MailerLite::new("api_key".to_string()).with_base_url("https://api.example.com/v1");
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

    pub fn email(&self) -> Email {
        Email::new(self.clone())
    }

    pub fn brand_style(&self) -> BrandStyle {
        BrandStyle::new(self.clone())
    }

    pub fn file(&self) -> File {
        File::new(self.clone())
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
            MailerLite::new("api_key".to_string()).with_base_url("https://api.example.com/v1");
        assert_eq!(mailerlite.base_url, "https://api.example.com/v1/");

        let mailerlite: MailerLite =
            MailerLite::new("api_key".to_string()).with_base_url("https://api.example.com/v1/");
        assert_eq!(mailerlite.base_url, "https://api.example.com/v1/");
    }
}
