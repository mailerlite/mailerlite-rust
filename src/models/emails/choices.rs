//! Named editor choices; unknown wire values are retained.
use crate::models::open_enum::open_enum;
open_enum! {
    /// Editor or message type reported by an email document or campaign email.
    pub enum EmailType {
        SimpleEditor => "builder_inline_email",
        DragAndDrop => "builder",
        RichText => "rich_text",
        IosRichText => "ios_rich_text",
        BuilderRichText => "builder_rich_text",
        CustomHtml => "builder_html",
        AiCustomHtml => "builder_nebula_ai_custom_html",
        Confirmation => "confirmation_email",
        StripeProduct => "stripe_product",
        StripeSubscriptionCreate => "stripe_subscription_create",
        StripeSubscriptionCancel => "stripe_subscription_cancel",
        PreferenceCenter => "preference_center_email",
        WheelPrize => "wheel_prize_email",
        DigitalProduct => "digital_product",
        BookingConfirmation => "booking_confirmation",
    }
}

open_enum! {
    /// Resource that owns an email document.
    pub enum EmailOwnerType {
        Campaign => "campaigns",
        Automation => "automations",
    }
}

open_enum! {
    /// Category of a saved email template.
    pub enum EmailTemplateType {
        Email => "email",
    }
}

open_enum! {
    /// Alignment of inline text within its block.
    pub enum TextAlignment {
        Left => "left",
        Center => "center",
        Right => "right",
        Justify => "justify",
    }
}

open_enum! {
    /// Horizontal placement of a block within the email.
    pub enum BlockAlignment {
        Left => "left",
        Center => "center",
        Right => "right",
    }
}

open_enum! {
    /// Devices on which the editor renders a block.
    pub enum BlockVisibility {
        All => "all",
        Desktop => "desktop",
        Mobile => "mobile",
    }
}

open_enum! {
    /// Editor button size preset.
    pub enum ButtonSize {
        Small => "small",
        Medium => "medium",
        Large => "large",
    }
}

open_enum! {
    /// Style slot used by a button.
    pub enum ButtonStyle {
        Primary => "primary",
        Secondary => "secondary",
        Tertiary => "tertiary",
    }
}

open_enum! {
    /// Whether a button uses its natural width or fills the row.
    pub enum ButtonWidth {
        Auto => "auto",
        Full => "full",
    }
}

open_enum! {
    /// Line pattern of a divider.
    pub enum DividerStyle {
        Solid => "solid",
        Dashed => "dashed",
        Dotted => "dotted",
    }
}

open_enum! {
    /// Semantic role of an editor block within the footer.
    pub enum FooterRole {
        Logo => "logo",
        CompanyInfo => "companyInfo",
        Social => "social",
    }
}

open_enum! {
    /// Color treatment of social icons.
    pub enum SocialIconColor {
        Black => "black",
        Color => "color",
        Grey => "grey",
        White => "white",
    }
}

open_enum! {
    /// Shape preset for social icons.
    pub enum SocialIconShape {
        Default => "default",
        Round => "round",
        RoundStroke => "round_stroke",
        Square => "square",
        RoundedCorners => "rounded_corners",
        RoundedStroke => "rounded_stroke",
    }
}

open_enum! {
    /// Thumbnail mode used for a video block.
    pub enum VideoPreview {
        Static => "static",
        Animated => "animated",
    }
}

open_enum! {
    /// Social network or destination category of a footer link or social icon.
    pub enum SocialLinkType {
        Facebook => "facebook",
        X => "x",
        LinkedIn => "linkedin",
        YouTube => "youtube",
        Vimeo => "vimeo",
        Pinterest => "pinterest",
        Tumblr => "tumblr",
        WordPress => "wordpress",
        Instagram => "instagram",
        Snapchat => "snapchat",
        Imdb => "imdb",
        SoundCloud => "soundcloud",
        Vine => "vine",
        Slack => "slack",
        Spotify => "spotify",
        Periscope => "periscope",
        Email => "email",
        Website => "website",
        Location => "location",
        Rss => "rss",
        Behance => "behance",
        DeviantArt => "deviantart",
        Dribbble => "dribbble",
        Foursquare => "foursquare",
        Odnoklassniki => "odnoklassniki",
        Telegram => "telegram",
        BookBub => "bookbub",
        Amazon => "amazon",
        Goodreads => "goodreads",
        GitHub => "github",
        Medium => "medium",
        Xing => "xing",
        Twitch => "twitch",
        Discord => "discord",
        TikTok => "tiktok",
        WhatsApp => "whatsapp",
        Flickr => "flickr",
        Patreon => "patreon",
        Riot => "riot",
        GooglePodcasts => "google_podcasts",
        ApplePodcasts => "apple_podcasts",
        Audible => "audible",
        MeWe => "mewe",
        Overcast => "overcast",
        Reddit => "reddit",
        Steam => "steam",
        Strava => "strava",
        Viber => "viber",
        Clubhouse => "clubhouse",
        Gettr => "gettr",
        OpenSea => "open_sea",
        Mastodon => "mastodon",
        Bandcamp => "bandcamp",
        Quora => "quora",
        Threads => "threads",
        Bluesky => "bluesky",
    }
}
