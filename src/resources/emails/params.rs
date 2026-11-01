//! Typed simple-editor write parameters and extension-key validation.
use super::validation as validate;
use crate::{
    Error,
    models::emails::{
        BlockType, BrandMode, EditorText, FontWeight, LayoutAlignment, LetterSpacing, LineHeight,
        Position, WidthMode,
    },
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

impl NewBlock {
    /// Create a block with its required type and editor document.
    ///
    /// Content remains an open JSON object so editor-specific camelCase keys and
    /// future document nodes are preserved. The API validates the document.
    pub fn new(block_type: BlockType, editor_content: Map<String, Value>) -> Self {
        Self {
            block_type,
            editor_content,
            placement: None,
        }
    }

    /// Replace the placement; omission appends the block to the email.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = Some(placement);
        self
    }
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

/// Changes to an email's design settings, for [`super::Emails::update_settings`]
/// and [`super::Emails::validate_settings`].
///
/// `None` omits a field; `Some(value)` sends it. Typed choices preserve their wire
/// strings, and scalar sizes remain strings such as `"16"`. The API validates
/// values and combinations, including conflicts with an inherited brand mode.
///
/// Empty strings in `brand.font_family`, `brand.background_color`,
/// `brand.text_color`, and `brand.border_color` restore account inheritance.
/// Other colors, sizes, and `brand.style` reject empty strings with HTTP 422.
/// Empty footer link labels also restore inherited labels. To clear a footer
/// editor document, use [`EditorText::default`] (which sends
/// `{"editor_content": {}}`) or an editor document with `"content": []`.
///
/// Send only intended changes. Explicit overrides can stop a value following
/// the account, while brand mode changes select an inherited or custom palette.
/// There is deliberately no automatic conversion from response settings to a
/// request. Social links replace the complete list when supplied.
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
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to brand; omitted fields are not sent.
pub struct BrandUpdate {
    /// One of the `BrandMode` values, for example `Some(BrandMode::Custom)`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<BrandMode>,
    /// Palette identifier from
    /// [`crate::resources::brand_styles::BrandStylesResource::get`].
    /// The API applies the palette but does not return this write-only value.
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
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Optional changes to layout; omitted fields are not sent.
pub struct LayoutUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Width preset understood by the editor.
    pub width_mode: Option<WidthMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Horizontal alignment understood by the editor.
    pub alignment: Option<LayoutAlignment>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
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
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
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
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
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
    pub font_weight: Option<FontWeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Line height as a CSS string.
    pub line_height: Option<LineHeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<LetterSpacing>,
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
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
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
    pub font_weight: Option<FontWeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Line height as a CSS string.
    pub line_height: Option<LineHeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<LetterSpacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Styles for the default paragraph level.
    pub p1: Option<TextStyleUpdate>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are. A key must not be
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
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
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
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
    pub font_weight: Option<FontWeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<LetterSpacing>,
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
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
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
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
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
    /// the JSON name of a typed field of this struct ([`Error::Config`]).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl SocialLinkUpdate {
    /// Create a replacement social link with its required URL, type, and label.
    ///
    /// Link types remain strings because the editor schema does not enumerate
    /// them. The API sanitizes the URL and validates the supplied values.
    pub fn new(
        url: impl Into<String>,
        link_type: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            url: Some(url.into()),
            link_type: Some(link_type.into()),
            label: Some(label.into()),
            ..Self::default()
        }
    }

    /// Retain an existing link's identifier and click history.
    pub fn link_id(mut self, link_id: u64) -> Self {
        self.link_id = Some(link_id);
        self
    }
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
    use serde_json::json;

    #[test]
    fn typed_brand_modes_preserve_wire_values_and_omission() {
        for mode in [
            BrandMode::Account,
            BrandMode::Custom,
            BrandMode::Other("future".into()),
        ] {
            let expected = mode.as_str().to_owned();
            let update = BrandUpdate {
                mode: Some(mode),
                ..Default::default()
            };
            assert_eq!(
                serde_json::to_value(update).unwrap(),
                json!({"mode": expected})
            );
        }
        assert_eq!(
            serde_json::to_value(BrandUpdate::default()).unwrap(),
            json!({})
        );
    }

    #[test]
    fn new_block_preserves_content_and_replaces_placement() {
        let content = Map::from_iter([("textAlign".into(), json!("left"))]);
        let block = NewBlock::new(BlockType::Paragraph, content)
            .placement(Placement::before("00123"))
            .placement(Placement::after("00456"));
        assert_eq!(
            serde_json::to_value(block).unwrap(),
            json!({
                "type": "paragraph", "editor_content": {"textAlign": "left"},
                "placement": {"position": "after", "block_id": "00456"}
            })
        );
        let block = NewBlock::new(BlockType::Divider, Map::new());
        assert_eq!(
            serde_json::to_value(block).unwrap(),
            json!({"type": "divider", "editor_content": {}})
        );
    }

    #[test]
    fn social_link_constructor_supplies_required_fields_and_preserves_id() {
        let link = SocialLinkUpdate::new("https://example.test", "future", "Example")
            .link_id(1)
            .link_id(u64::MAX);
        assert_eq!(
            serde_json::to_value(link).unwrap(),
            json!({
                "url": "https://example.test", "type": "future", "label": "Example",
                "link_id": u64::MAX
            })
        );
    }

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
            mode: Some(BrandMode::Custom),
            style: set(),
            font_family: set(),
            background_color: set(),
            text_color: set(),
            border_color: set(),
            extra: extra(),
        });
        keys(LayoutUpdate {
            width_mode: Some(WidthMode::Default),
            alignment: Some(LayoutAlignment::Left),
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
            font_weight: Some(FontWeight::Normal),
            line_height: Some(LineHeight::Percent150),
            letter_spacing: Some(LetterSpacing::Normal),
            h1: set(),
            h2: set(),
            h3: set(),
            extra: extra(),
        });
        keys(ParagraphStyleUpdate {
            font_family: set(),
            font_weight: Some(FontWeight::Normal),
            line_height: Some(LineHeight::Percent150),
            letter_spacing: Some(LetterSpacing::Normal),
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
            font_weight: Some(FontWeight::Normal),
            letter_spacing: Some(LetterSpacing::Normal),
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
}
