//! Typed simple-editor write parameters and extension-key validation.
use super::validation as validate;
use crate::{
    Error,
    models::emails::{BlockType, EditorText, Position},
};
use serde::Serialize;
use serde_json::{Map, Value};
/// A new block for `Emails::create_block` and `Emails::validate_block`.
#[derive(Debug, Clone, Serialize)]
pub struct NewBlock {
    #[serde(rename = "type")]
    /// Editor block discriminator.
    pub block_type: BlockType,
    /// The editor document of the block. Its keys are camelCase. Use an empty map for a block with no
    /// content, for example a divider.
    pub editor_content: Map<String, Value>,
    /// Where to put the block. Without a placement, the API puts the block at the end of the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
}

/// Where to put a block. The positions `Before` and `After` need the `block_id` of a block at the same
/// level. The SDK sends the placement as you give it.
#[derive(Debug, Clone, Serialize)]
pub struct Placement {
    /// Relative position at the current nesting level.
    pub position: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Adjacent block identifier for before or after placement.
    pub block_id: Option<String>,
}

impl Placement {
    /// Place the block first at its current nesting level.
    pub fn start() -> Self {
        Self {
            position: Position::Start,
            block_id: None,
        }
    }

    /// Place the block last at its current nesting level.
    pub fn end() -> Self {
        Self {
            position: Position::End,
            block_id: None,
        }
    }

    /// Place the block immediately before the supplied block ID.
    pub fn before(block_id: impl Into<String>) -> Self {
        Self {
            position: Position::Before,
            block_id: Some(block_id.into()),
        }
    }

    /// Place the block immediately after the supplied block ID.
    pub fn after(block_id: impl Into<String>) -> Self {
        Self {
            position: Position::After,
            block_id: Some(block_id.into()),
        }
    }
}

#[derive(Serialize)]
pub(super) struct EtagBody<'a, T: Serialize> {
    #[serde(flatten)]
    pub(super) body: &'a T,
    pub(super) etag: &'a str,
}

#[derive(Serialize)]
pub(super) struct EditorContentBody<'a> {
    pub(super) editor_content: &'a Map<String, Value>,
    pub(super) etag: &'a str,
}

#[derive(Serialize)]
pub(super) struct MoveBody<'a> {
    pub(super) placement: &'a Placement,
    pub(super) etag: &'a str,
}

/// A change to the design settings of an email, for `Emails::update_settings` and `Emails::validate_settings`.
///
/// Each value has three states:
/// - `None`: the SDK does not send the key, and the value does not change;
/// - `Some(value)`: the API stores the value;
/// - `Some(String::new())`: an empty value. Only the brand colors accept it: `brand.background_color`,
///   `brand.text_color` and `brand.border_color`. For them, the API removes the value, and the email uses
///   the color of the account again. Other colors, the sizes and `brand.style` do not accept an empty value.
///   For them, the API gives a `422` error.
///
/// To remove a footer text, send an empty editor document: `EditorText::default()` (`{}`) or
/// `{"content": []}`. Then the email uses the text of the account again.
///
/// Set only the values to change. The API stores each value that you send, also a value that is the same as
/// the value of the account. Then the email no longer follows the account for that value. For this reason,
/// the SDK has no conversion from `EmailSettings`. Send all style values as strings, for example `"16"`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EmailSettingsUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Brand palette and typography overrides.
    pub brand: Option<BrandUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Email width and alignment settings.
    pub layout: Option<LayoutUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Link, heading, and paragraph styles.
    pub typography: Option<TypographyUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Default button styles for this email.
    pub button: Option<ButtonUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Footer content and display settings.
    pub footer: Option<FooterUpdate>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to brand; omitted fields are not sent.
pub struct BrandUpdate {
    /// One of the `BrandMode` values, for example `Some(BrandMode::Custom.into())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// The `identifier` of a palette from `BrandStyle::get`. The API applies the palette. The API does not
    /// return this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CSS font-family value used for this style.
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Background color in the API color format.
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Foreground text color in the API color format.
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Border color in the API color format.
    pub border_color: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to layout; omitted fields are not sent.
pub struct LayoutUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Width preset understood by the editor.
    pub width_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Horizontal alignment understood by the editor.
    pub alignment: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to typography; omitted fields are not sent.
pub struct TypographyUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Styles for hyperlinks.
    pub link: Option<LinkStyleUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Styles shared by heading levels.
    pub heading: Option<HeadingStyleUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Styles shared by paragraphs.
    pub paragraph: Option<ParagraphStyleUpdate>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to link style; omitted fields are not sent.
pub struct LinkStyleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text color in the API color format.
    pub color: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to heading style; omitted fields are not sent.
pub struct HeadingStyleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CSS font-family value used for this style.
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Font weight as a CSS string.
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Line height as a CSS string.
    pub line_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Styles for first-level headings.
    pub h1: Option<TextStyleUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Styles for second-level headings.
    pub h2: Option<TextStyleUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Styles for third-level headings.
    pub h3: Option<TextStyleUpdate>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to paragraph style; omitted fields are not sent.
pub struct ParagraphStyleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CSS font-family value used for this style.
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Font weight as a CSS string.
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Line height as a CSS string.
    pub line_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Styles for the default paragraph level.
    pub p1: Option<TextStyleUpdate>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to text style; omitted fields are not sent.
pub struct TextStyleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text color in the API color format.
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Font size as a CSS string.
    pub size: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to button; omitted fields are not sent.
pub struct ButtonUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CSS font-family value used for this style.
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Font weight as a CSS string.
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Background color in the API color format.
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Foreground text color in the API color format.
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Corner radius as a CSS string.
    pub border_radius: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to footer; omitted fields are not sent.
pub struct FooterUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the footer displays preference-management links.
    pub show_preferences: Option<bool>,
    /// An empty editor document (`{}` or `{"content": []}`) removes the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<EditorText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Postal address represented as editor content.
    pub address: Option<EditorText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional footer information represented as editor content.
    pub info_text: Option<EditorText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Label for the unsubscribe link.
    pub unsubscribe_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Label for the preferences link.
    pub preferences_text: Option<String>,
    /// The new list replaces the whole list. Keep the `link_id` of each link to keep its click history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_links: Option<Vec<SocialLinkUpdate>>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to social link; omitted fields are not sent.
pub struct SocialLinkUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URL returned by or submitted to the API.
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    /// Social-link type understood by the editor.
    pub link_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Display label for the link.
    pub label: Option<String>,
    /// The ID of an existing link. Send it to keep the click history of the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<u64>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct (`Error::DuplicateSettingsKey`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A unit test compares each `FIELDS` list with the serialized fields.
pub(super) trait SettingsKeys {
    const FIELDS: &'static [&'static str];

    fn extra(&self) -> &Map<String, Value>;

    fn check_children(&self, _path: &str) -> Result<(), Error> {
        Ok(())
    }

    fn check_keys(&self, path: &str) -> Result<(), Error> {
        validate::no_field_keys(self.extra(), Self::FIELDS, path)?;
        self.check_children(path)
    }
}

fn check_child<T: SettingsKeys>(child: &Option<T>, path: &str, key: &str) -> Result<(), Error> {
    match child {
        Some(child) => child.check_keys(&validate::settings_path(path, key)),
        None => Ok(()),
    }
}

macro_rules! settings_keys {
    ($name:ty, [$($field:literal),* $(,)?]) => {
        settings_keys!($name, [$($field),*], |_value, _path| Ok(()));
    };
    ($name:ty, [$($field:literal),* $(,)?], |$value:ident, $path:ident| $children:expr) => {
        impl SettingsKeys for $name {
            const FIELDS: &'static [&'static str] = &[$($field),*];

            fn extra(&self) -> &Map<String, Value> {
                &self.extra
            }

            fn check_children(&self, path: &str) -> Result<(), Error> {
                let ($value, $path): (&Self, &str) = (self, path);
                $children
            }
        }
    };
}

settings_keys!(
    EmailSettingsUpdate,
    ["brand", "layout", "typography", "button", "footer"],
    |value, path| {
        check_child(&value.brand, path, "brand")?;
        check_child(&value.layout, path, "layout")?;
        check_child(&value.typography, path, "typography")?;
        check_child(&value.button, path, "button")?;
        check_child(&value.footer, path, "footer")
    }
);
settings_keys!(
    BrandUpdate,
    [
        "mode",
        "style",
        "font_family",
        "background_color",
        "text_color",
        "border_color"
    ]
);
settings_keys!(LayoutUpdate, ["width_mode", "alignment"]);
settings_keys!(
    TypographyUpdate,
    ["link", "heading", "paragraph"],
    |value, path| {
        check_child(&value.link, path, "link")?;
        check_child(&value.heading, path, "heading")?;
        check_child(&value.paragraph, path, "paragraph")
    }
);
settings_keys!(LinkStyleUpdate, ["color"]);
settings_keys!(
    HeadingStyleUpdate,
    [
        "font_family",
        "font_weight",
        "line_height",
        "letter_spacing",
        "h1",
        "h2",
        "h3"
    ],
    |value, path| {
        check_child(&value.h1, path, "h1")?;
        check_child(&value.h2, path, "h2")?;
        check_child(&value.h3, path, "h3")
    }
);
settings_keys!(
    ParagraphStyleUpdate,
    [
        "font_family",
        "font_weight",
        "line_height",
        "letter_spacing",
        "p1"
    ],
    |value, path| check_child(&value.p1, path, "p1")
);
settings_keys!(TextStyleUpdate, ["color", "size"]);
settings_keys!(
    ButtonUpdate,
    [
        "font_family",
        "font_weight",
        "letter_spacing",
        "background_color",
        "text_color",
        "border_radius"
    ]
);
settings_keys!(
    FooterUpdate,
    [
        "show_preferences",
        "company_name",
        "address",
        "info_text",
        "unsubscribe_text",
        "preferences_text",
        "social_links",
    ],
    |value, path| {
        for (index, link) in value.social_links.iter().flatten().enumerate() {
            link.check_keys(&validate::settings_path(
                path,
                &format!("social_links[{index}]"),
            ))?;
        }
        Ok(())
    }
);
settings_keys!(SocialLinkUpdate, ["url", "type", "label", "link_id"]);

impl EmailSettingsUpdate {
    /// Only `update_settings` adds an etag, so `validate_settings` does not check for an `etag` key.
    pub(super) fn check(&self) -> Result<(), Error> {
        validate::no_etag_key(&self.extra)?;
        self.check_keys("")
    }
}

#[derive(Serialize)]
pub(super) struct NameBody<'a> {
    pub(super) name: &'a str,
}

/// The page options of `Emails::versions`. The SDK does not send `None` values.
#[derive(Debug, Clone, Default)]
pub struct ListVersionsOptions {
    /// One-based page number; omission uses the server default.
    pub page: Option<u32>,
    /// Requested page size; omission uses the server default.
    pub limit: Option<u32>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]
mod tests {
    use super::*;
    use crate::models::emails::BrandMode;
    use serde_json::json;

    fn set<T: Default>() -> Option<T> {
        Some(T::default())
    }

    fn keys<T: Serialize + SettingsKeys>(value: T) {
        let json: Value = serde_json::to_value(&value).unwrap();
        let mut serialized: Vec<&str> = json
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        let mut fields: Vec<&str> = T::FIELDS.to_vec();
        serialized.sort();
        fields.sort();
        assert_eq!(serialized, fields);
    }

    #[test]
    fn settings_field_lists_match_the_serialized_fields() {
        // The struct literals have no `..Default::default()`, so a new field breaks this test until its
        // JSON name is in the list.
        let extra = Map::new;
        keys(EmailSettingsUpdate {
            brand: set(),
            layout: set(),
            typography: set(),
            button: set(),
            footer: set(),
            extra: extra(),
        });
        keys(BrandUpdate {
            mode: set(),
            style: set(),
            font_family: set(),
            background_color: set(),
            text_color: set(),
            border_color: set(),
            extra: extra(),
        });
        keys(LayoutUpdate {
            width_mode: set(),
            alignment: set(),
            extra: extra(),
        });
        keys(TypographyUpdate {
            link: set(),
            heading: set(),
            paragraph: set(),
            extra: extra(),
        });
        keys(LinkStyleUpdate {
            color: set(),
            extra: extra(),
        });
        keys(HeadingStyleUpdate {
            font_family: set(),
            font_weight: set(),
            line_height: set(),
            letter_spacing: set(),
            h1: set(),
            h2: set(),
            h3: set(),
            extra: extra(),
        });
        keys(ParagraphStyleUpdate {
            font_family: set(),
            font_weight: set(),
            line_height: set(),
            letter_spacing: set(),
            p1: set(),
            extra: extra(),
        });
        keys(TextStyleUpdate {
            color: set(),
            size: set(),
            extra: extra(),
        });
        keys(ButtonUpdate {
            font_family: set(),
            font_weight: set(),
            letter_spacing: set(),
            background_color: set(),
            text_color: set(),
            border_radius: set(),
            extra: extra(),
        });
        keys(FooterUpdate {
            show_preferences: set(),
            company_name: set(),
            address: set(),
            info_text: set(),
            unsubscribe_text: set(),
            preferences_text: set(),
            social_links: set(),
            extra: extra(),
        });
        keys(SocialLinkUpdate {
            url: set(),
            link_type: set(),
            label: set(),
            link_id: set(),
            extra: extra(),
        });
    }

    #[test]
    fn open_enums_keep_unknown_values() {
        assert_eq!(
            BlockType::from("paragraph".to_string()),
            BlockType::Paragraph
        );
        assert_eq!(
            BlockType::from("countdown".to_string()),
            BlockType::Other("countdown".to_string())
        );
        assert_eq!(String::from(BlockType::BulletedList), "bulleted_list");
        assert_eq!(String::from(BlockType::Other("x".to_string())), "x");
        assert_eq!(
            serde_json::to_value(Position::After).unwrap(),
            json!("after")
        );
        assert_eq!(
            serde_json::from_value::<BrandMode>(json!("custom")).unwrap(),
            BrandMode::Custom
        );
        assert_eq!(BrandMode::Account.to_string(), "account");
    }
}
