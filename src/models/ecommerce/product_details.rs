//! Internal-shop product fields verified against the backend resources.
use super::wire;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

/// Weekday names mapped to start/end times, or null for unavailable days.
///
/// Keys preserve the server's spelling. To reuse this schedule in a request,
/// parse them as [`crate::resources::ecommerce::params::BookingWeekday`]; its
/// example preserves both time ranges and unavailable-day nulls.
pub type BookingAvailability = BTreeMap<String, Option<[String; 2]>>;

/// Kind of downloadable product asset.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProductAssetType {
    /// An uploaded file.
    File,
    /// A link delivered to the customer.
    Url,
    /// A future server asset kind, retained without losing its spelling.
    Other(String),
}
impl<'de> Deserialize<'de> for ProductAssetType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "file" => Self::File,
            "url" => Self::Url,
            other => Self::Other(other.into()),
        })
    }
}

/// A digital product asset returned by MailerLite.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ProductAsset {
    /// Asset identifier, preserved as text even when encoded as an integer.
    #[serde(deserialize_with = "wire::id")]
    pub id: String,
    /// Display name of the asset.
    pub name: String,
    /// File or URL asset kind.
    #[serde(rename = "type")]
    pub kind: ProductAssetType,
    /// Asset destination; booking products may return an empty URL.
    pub url: Option<String>,
    /// Associated product variant, when present.
    #[serde(default, deserialize_with = "wire::optional_id")]
    pub variant_id: Option<String>,
}

/// Aggregate completed orders for a product in an internal-shop listing.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ProductOrderStats {
    /// Number of completed orders; the backend also encodes this as digit text.
    #[serde(deserialize_with = "wire::unsigned")]
    pub total_orders: u64,
    /// Sum of item prices, decoded without floating-point conversion.
    #[serde(deserialize_with = "wire::decimal")]
    pub total_amount: Decimal,
    /// Currency symbol used by the backend, rather than an ISO currency code.
    pub currency: String,
}

/// Timezone embedded in a booking product's resolved settings.
///
/// The backend embeds its model here, rather than the `/timezones` resource.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ProductTimezone {
    /// Timezone identifier.
    #[serde(deserialize_with = "wire::id")]
    pub id: String,
    /// IANA timezone name.
    pub name: String,
    /// UTC offset supplied by the API.
    pub offset: i32,
    /// Display name of the UTC offset.
    pub offset_name: String,
}

/// Resolved settings of an internal digital or booking product.
///
/// Fields are optional because they depend on product kind and account setup.
/// Unknown object fields are ignored for forward compatibility; malformed known
/// fields fail decoding. Request settings live in
/// [`crate::resources::ecommerce::params::ProductSettings`].
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct ProductSettings {
    /// Action applied to the subscriber after purchase.
    pub action: Option<String>,
    /// Group targeted by a move/remove action.
    pub group_id: Option<String>,
    /// Custom field targeted by an update action.
    pub field_id: Option<String>,
    /// Custom-field operation, such as setting the current date.
    pub action_on_field: Option<String>,
    /// Custom-field value; may be null for a current-date action.
    pub field_value: Option<String>,
    /// Checkout provider identifier.
    pub checkout_type: Option<String>,
    /// Whether checkout requests a tax identifier.
    #[serde(default, deserialize_with = "wire::optional_bool")]
    pub enable_tax_id: Option<bool>,
    /// Whether checkout requires acceptance of terms.
    #[serde(default, deserialize_with = "wire::optional_bool")]
    pub enable_terms: Option<bool>,
    /// Provider price identifier.
    pub price_id: Option<String>,
    /// Email of the user who created the product.
    pub created_by_user_email: Option<String>,
    /// Whether the product has been used.
    #[serde(default, deserialize_with = "wire::optional_bool")]
    pub used: Option<bool>,
    /// Calendar integration identifier, such as google, outlook, or external.
    pub calendar_integration_type: Option<String>,
    /// Booking duration in the supplied duration unit.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub duration: Option<Decimal>,
    /// Duration unit, normally minutes or hours.
    pub duration_unit: Option<String>,
    /// Timezone identifier selected for the booking product.
    #[serde(default, deserialize_with = "wire::optional_id")]
    pub timezone_id: Option<String>,
    /// Resolved timezone, when the configured identifier exists.
    pub timezone: Option<ProductTimezone>,
    /// Number of future days during which a booking may be made.
    #[serde(default, deserialize_with = "wire::optional_unsigned")]
    pub booking_window_days: Option<u64>,
    /// Booking mode, such as one-on-one or group.
    pub booking_type: Option<String>,
    /// Buffer separating meetings.
    #[serde(default, deserialize_with = "wire::optional_unsigned")]
    pub buffer_between_meetings: Option<u64>,
    /// Buffer unit, normally minutes or hours.
    pub buffer_between_meetings_unit: Option<String>,
    /// Minimum notice required before a booking.
    #[serde(default, deserialize_with = "wire::optional_unsigned")]
    pub minimum_notice_period: Option<u64>,
    /// Notice unit, normally minutes, hours, or days.
    pub minimum_notice_period_unit: Option<String>,
    /// Meeting location identifier.
    pub location: Option<String>,
    /// Address for an in-person meeting.
    pub location_address: Option<String>,
    /// Additional meeting instructions.
    pub location_instructions: Option<String>,
    /// Link for a custom online meeting location.
    pub location_link: Option<String>,
    /// Link to an external booking system.
    pub external_link: Option<String>,
    /// Weekday names mapped to a start/end time pair or null for unavailable days.
    #[serde(default, deserialize_with = "optional_availability")]
    pub custom_availability_days: Option<BookingAvailability>,
    /// Whether the product is free.
    #[serde(default, deserialize_with = "wire::optional_bool")]
    pub free: Option<bool>,
    /// Group booking start time in UTC.
    pub booking_group_date_time_utc: Option<String>,
    /// Maximum participants allowed in a group booking.
    #[serde(default, deserialize_with = "wire::optional_unsigned")]
    pub booking_group_max_participants: Option<u64>,
    /// Whether checkout displays the remaining group slots.
    #[serde(default, deserialize_with = "wire::optional_bool")]
    pub booking_group_display_remaining_slots: Option<bool>,
    /// Calendar event identifier created for a group booking.
    pub booking_event_id: Option<String>,
    /// Number of group booking attendees recorded by the backend.
    #[serde(default, deserialize_with = "wire::optional_unsigned")]
    pub booking_attendees_count: Option<u64>,
}

fn optional_availability<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<BookingAvailability>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AvailabilityWire {
        Days(BookingAvailability),
        Empty([(); 0]),
    }
    Ok(
        Option::<AvailabilityWire>::deserialize(deserializer)?.map(|days| match days {
            AvailabilityWire::Days(days) => days,
            AvailabilityWire::Empty(_) => BTreeMap::new(),
        }),
    )
}

// PHP encodes an empty settings array as [], while populated settings are an
// object. Only that empty array is accepted; other malformed shapes stay errors.
pub(super) fn optional_settings<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<ProductSettings>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum SettingsWire {
        Settings(Box<ProductSettings>),
        Empty([(); 0]),
    }
    Ok(
        Option::<SettingsWire>::deserialize(deserializer)?.map(|settings| match settings {
            SettingsWire::Settings(settings) => *settings,
            SettingsWire::Empty(_) => ProductSettings::default(),
        }),
    )
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::ProductAssetType;
    use crate::models::ecommerce::Product;
    use serde_json::json;

    #[test]
    fn settings_accept_php_empty_array_as_present_empty_settings() {
        let product: Product =
            serde_json::from_value(json!({"id":"1","name":"Empty","settings":[]})).unwrap();
        assert!(product.settings.unwrap().action.is_none());
    }

    #[test]
    fn settings_accept_null_as_absent_settings() {
        let product: Product =
            serde_json::from_value(json!({"id":"1","name":"Empty","settings":null})).unwrap();
        assert!(product.settings.is_none());
    }

    #[test]
    fn settings_accept_absent_settings() {
        let product: Product = serde_json::from_value(json!({"id":"1","name":"Empty"})).unwrap();
        assert!(product.settings.is_none());
    }

    #[test]
    fn availability_accepts_php_empty_array() {
        let product: Product = serde_json::from_value(
            json!({"id":"1","name":"Empty","settings":{"custom_availability_days":[]}}),
        )
        .unwrap();
        assert!(
            product
                .settings
                .unwrap()
                .custom_availability_days
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn settings_accept_laravel_integer_boolean_values() {
        let product: Product = serde_json::from_value(
            json!({"id":"1","name":"Stored","settings":{"free":1,"enable_terms":0}}),
        )
        .unwrap();
        let settings = product.settings.unwrap();
        assert_eq!(settings.free, Some(true));
        assert_eq!(settings.enable_terms, Some(false));
    }

    #[test]
    fn settings_accept_laravel_digit_string_boolean_values() {
        let product: Product = serde_json::from_value(
            json!({"id":"1","name":"Stored","settings":{"free":"0","enable_terms":"1"}}),
        )
        .unwrap();
        let settings = product.settings.unwrap();
        assert_eq!(settings.free, Some(false));
        assert_eq!(settings.enable_terms, Some(true));
    }

    #[test]
    fn unknown_asset_kinds_preserve_the_server_value() {
        let product: Product = serde_json::from_value(json!({"id":"1","name":"Future","assets":[{"id":"2","name":"Extra","type":"future-kind","url":null,"variant_id":null}]})).unwrap();
        assert_eq!(
            product.assets.unwrap()[0].kind,
            ProductAssetType::Other("future-kind".into())
        );
    }

    #[test]
    fn settings_ignore_unknown_fields_without_weakening_known_fields() {
        let product: Product = serde_json::from_value(
            json!({"id":"1","name":"Future","settings":{"free":true,"future":{"nested":[1,2]}}}),
        )
        .unwrap();
        assert_eq!(product.settings.unwrap().free, Some(true));
    }

    #[test]
    fn settings_reject_nonempty_array() {
        assert!(
            serde_json::from_value::<Product>(
                json!({"id":"1","name":"Invalid","settings":[{"action":"move_to_group"}]})
            )
            .is_err()
        );
    }

    #[test]
    fn settings_reject_invalid_boolean_instead_of_falling_back_to_json() {
        assert!(
            serde_json::from_value::<Product>(
                json!({"id":"1","name":"Invalid","settings":{"free":"yes"}})
            )
            .is_err()
        );
    }

    #[test]
    fn availability_rejects_more_than_two_times() {
        assert!(serde_json::from_value::<Product>(json!({"id":"1","name":"Invalid","settings":{"custom_availability_days":{"monday":["09:00","12:00","17:00"]}}})).is_err());
    }

    #[test]
    fn order_stats_reject_count_overflow() {
        assert!(serde_json::from_value::<Product>(json!({"id":"1","name":"Invalid","order_stats":{"total_orders":"18446744073709551616","total_amount":"0","currency":"$"}})).is_err());
    }
}
