//! The brand styles of the account: the palettes that an email can use.

use crate::{
    api_response::{ApiResponse, DataOnly},
    error::Error,
    http::{self, Method, NO_BODY},
    MailerLite,
};
use serde::Deserialize;

const END_POINT: &str = "brand-styles";

/// All palettes that the account can use.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct BrandStyles {
    /// The brand styles of the account. `None` when the account has none.
    pub brand: Option<BrandPalette>,
    pub default: BrandPalette,
    #[serde(default)]
    pub presets: Vec<BrandPalette>,
}

/// A palette of one font and six colors. Match a palette on its `identifier`. The `name` is only for
/// display and can change.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct BrandPalette {
    pub identifier: String,
    pub name: String,
    pub font_family: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub heading_color: String,
    pub text_color: String,
    pub border_color: String,
    pub background_color: String,
}

/// The brand style methods. Get it with `MailerLite::brand_style`.
#[derive(Debug)]
pub struct BrandStyle {
    mailerlite: MailerLite,
}

impl BrandStyle {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    /// Gets the brand styles of the account, the default palette and the preset palettes.
    pub async fn get(&self) -> Result<ApiResponse<DataOnly<BrandStyles>>, Error> {
        http::send(&self.mailerlite, Method::GET, END_POINT, &[], NO_BODY, None).await
    }
}
