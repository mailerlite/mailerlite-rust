//! Account brand palettes available to the simple editor.
use serde::Deserialize;
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Account, fallback, and preset brand palettes.
pub struct BrandStyles {
    /// `None` when the account has no brand styles.
    pub brand: Option<BrandPalette>,
    /// Fallback palette used when no account palette is configured.
    pub default: BrandPalette,
    #[serde(default)]
    /// Built-in palettes available for selection.
    pub presets: Vec<BrandPalette>,
}

/// A palette of one font and six colors. Match a palette on its `identifier`. The `name` is only for
/// display and can change.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct BrandPalette {
    /// Stable palette key used when selecting a brand style.
    pub identifier: String,
    /// Display name assigned by the API.
    pub name: String,
    /// CSS font-family value used for this style.
    pub font_family: String,
    /// Primary accent color in the palette.
    pub primary_color: String,
    /// Secondary accent color in the palette.
    pub secondary_color: String,
    /// Heading text color in the API color format.
    pub heading_color: String,
    /// Foreground text color in the API color format.
    pub text_color: String,
    /// Border color in the API color format.
    pub border_color: String,
    /// Background color in the API color format.
    pub background_color: String,
}
