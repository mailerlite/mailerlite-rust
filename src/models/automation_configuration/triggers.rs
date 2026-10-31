//! Typed trigger configuration and response details.
use super::{ConfigurationId, ConfigurationValue, ExtensionFields, RelatedObject, TimezoneMode};
use crate::models::Patch;
use serde::{Deserialize, Serialize};

/// Duration unit for ecommerce triggers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DurationUnit {
    /// The `hours` API value.
    #[serde(rename = "hours")]
    Hours,
    /// The `days` API value.
    #[serde(rename = "days")]
    Days,
    /// The `weeks` API value.
    #[serde(rename = "weeks")]
    Weeks,
    /// The `months` API value.
    #[serde(rename = "months")]
    Months,
}

/// Offset unit for date triggers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DateOffsetUnit {
    /// The `day` API value.
    #[serde(rename = "day")]
    Day,
    /// The `month` API value.
    #[serde(rename = "month")]
    Month,
}

/// Group-join trigger settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupTriggerConfiguration {
    /// Included group identifiers.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub group_ids: Patch<Vec<String>>,
    /// Excluded group identifiers.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub exclude_group_ids: Patch<Vec<String>>,
    /// Exit the workflow if membership no longer matches.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub exit_when_no_longer_matches: Patch<bool>,
}
impl GroupTriggerConfiguration {
    /// Set the included group identifiers, preserving empty lists.
    pub fn new(ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            group_ids: ids.into_iter().map(Into::into).collect::<Vec<_>>().into(),
            ..Self::default()
        }
    }
}

/// Segment-join trigger settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentTriggerConfiguration {
    /// Segment identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub segment_id: Patch<ConfigurationId>,
    /// Entry sampling percentage, from 1 through 100.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub entry_percentage: Patch<u8>,
    /// Exit when the subscriber stops matching the segment.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub exit_when_no_longer_matches: Patch<bool>,
}

/// Link-click trigger settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkTriggerConfiguration {
    /// Tracked link URL.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub link: Patch<String>,
}

/// Field-update trigger settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldTriggerConfiguration {
    /// Custom field identifier or the special email field.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub field_id: Patch<ConfigurationId>,
    /// Operator supported for the selected field type.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub field_operator: Patch<String>,
    /// Scalar comparison value; presence operators take no value.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub value: Patch<ConfigurationValue>,
}

/// Anniversary or exact-date settings; before and after offsets are mutually exclusive.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateTriggerConfiguration {
    /// Date field identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub field_id: Patch<ConfigurationId>,
    /// Included groups.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub group_ids: Patch<Vec<String>>,
    /// Excluded groups.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub exclude_group_ids: Patch<Vec<String>>,
    /// Zero-padded send hour.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub send_hour: Patch<String>,
    /// Zero-padded send minute.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub send_minute: Patch<String>,
    /// Offset before the date.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub start_before_time: Patch<u64>,
    /// Before-date offset unit.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub start_before_time_unit: Patch<DateOffsetUnit>,
    /// Offset after the date.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub start_after_time: Patch<u64>,
    /// After-date offset unit.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub start_after_time_unit: Patch<DateOffsetUnit>,
    /// Timezone selection.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub use_timezone: Patch<TimezoneMode>,
    /// Custom timezone identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub timezone_id: Patch<ConfigurationId>,
}

/// Form-completion trigger settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormTriggerConfiguration {
    /// Form type supported by the backend.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub form_type: Patch<String>,
    /// Form identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub form_id: Patch<ConfigurationId>,
}

/// Trigger for any purchase from a shop.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShopTriggerConfiguration {
    /// Ecommerce shop identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub ecommerce_shop_id: Patch<ConfigurationId>,
}

/// Trigger for purchases of a specific product.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductTriggerConfiguration {
    /// Ecommerce shop identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub ecommerce_shop_id: Patch<ConfigurationId>,
    /// Product identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub ecommerce_product_id: Patch<ConfigurationId>,
    /// Selected variants; explicit null matches all variants.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub variation_ids: Patch<Vec<String>>,
}

/// Trigger for purchases from a category.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryTriggerConfiguration {
    /// Ecommerce shop identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub ecommerce_shop_id: Patch<ConfigurationId>,
    /// Category identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub ecommerce_category_id: Patch<ConfigurationId>,
}

/// Trigger for abandoned carts or checkouts.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AbandonedTriggerConfiguration {
    /// Ecommerce shop identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub ecommerce_shop_id: Patch<ConfigurationId>,
    /// Waiting interval unit.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub unit: Patch<DurationUnit>,
    /// Positive waiting interval.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub value: Patch<u64>,
}

/// Trigger for purchase-frequency automation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PurchaseFrequencyConfiguration {
    /// Ecommerce shop identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub ecommerce_shop_id: Patch<ConfigurationId>,
    /// Waiting interval unit.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub unit: Patch<DurationUnit>,
    /// Positive waiting interval.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub value: Patch<u64>,
    /// Purchase interval, at least one.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub interval: Patch<u64>,
    /// Optional cap, at least the purchase interval.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub order_cap: Patch<u64>,
}

/// Typed request data for the supported externally configurable trigger types.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TriggerConfiguration {
    /// Settings for `subscriber_joins_group`.
    Group(GroupTriggerConfiguration),
    /// Settings for `subscriber_joins_segment`.
    Segment(SegmentTriggerConfiguration),
    /// Settings for `clicks_link`.
    ClicksLink(LinkTriggerConfiguration),
    /// Settings for `field_updated`.
    FieldUpdated(FieldTriggerConfiguration),
    /// Settings for `anniversary`.
    Anniversary(DateTriggerConfiguration),
    /// Settings for `exact_date`.
    ExactDate(DateTriggerConfiguration),
    /// Settings for `form_completed`.
    FormCompleted(FormTriggerConfiguration),
    /// Settings for `purchased_any_product`.
    PurchasedAnyProduct(ShopTriggerConfiguration),
    /// Settings for `purchased_specific_product`.
    PurchasedSpecificProduct(ProductTriggerConfiguration),
    /// Settings for `purchased_from_category`.
    PurchasedFromCategory(CategoryTriggerConfiguration),
    /// Settings for `abandoned_cart`.
    AbandonedCart(AbandonedTriggerConfiguration),
    /// Settings for `abandoned_checkout`.
    AbandonedCheckout(AbandonedTriggerConfiguration),
    /// Settings for `purchase_frequency`.
    PurchaseFrequency(PurchaseFrequencyConfiguration),
}
impl TriggerConfiguration {
    /// API trigger type associated with this configuration.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Group(..) => "subscriber_joins_group",
            Self::Segment(..) => "subscriber_joins_segment",
            Self::ClicksLink(..) => "clicks_link",
            Self::FieldUpdated(..) => "field_updated",
            Self::Anniversary(..) => "anniversary",
            Self::ExactDate(..) => "exact_date",
            Self::FormCompleted(..) => "form_completed",
            Self::PurchasedAnyProduct(..) => "purchased_any_product",
            Self::PurchasedSpecificProduct(..) => "purchased_specific_product",
            Self::PurchasedFromCategory(..) => "purchased_from_category",
            Self::AbandonedCart(..) => "abandoned_cart",
            Self::AbandonedCheckout(..) => "abandoned_checkout",
            Self::PurchaseFrequency(..) => "purchase_frequency",
        }
    }
}

/// Known type-specific trigger response settings.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct TriggerProperties {
    /// Resolved segment; an empty array from the backend means no segment.
    #[serde(default, deserialize_with = "empty_segment")]
    pub segment: Option<RelatedObject>,
    /// Backend `complete` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Backend `groups` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<RelatedObject>>,
    /// Backend `excluded_groups` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_groups: Option<Vec<RelatedObject>>,
    /// Backend `exit_when_no_longer_matches` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_when_no_longer_matches: Option<bool>,
    /// Backend `segment_id` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<ConfigurationId>,
    /// Backend `entry_percentage` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_percentage: Option<u8>,
    /// Backend `default_entry_percentage` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_entry_percentage: Option<u8>,
    /// Backend `segment_rules` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_rules: Option<Vec<SegmentDisplayRule>>,
    /// Backend `link` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Backend `field_id` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<ConfigurationId>,
    /// Backend `field` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<RelatedObject>,
    /// Backend `value` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ConfigurationValue>,
    /// Backend `field_operator` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_operator: Option<String>,
    /// Backend `form_id` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_id: Option<ConfigurationId>,
    /// Backend `form_type` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_type: Option<String>,
    /// Backend `form` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<RelatedObject>,
    /// Backend `ecommerce_shop_id` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_shop_id: Option<ConfigurationId>,
    /// Backend `ecommerce_shop` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_shop: Option<RelatedObject>,
    /// Backend `ecommerce_category_id` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_category_id: Option<ConfigurationId>,
    /// Backend `ecommerce_category` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_category: Option<RelatedObject>,
    /// Backend `ecommerce_product_id` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_product_id: Option<ConfigurationId>,
    /// Backend `ecommerce_product` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_product: Option<RelatedObject>,
    /// Backend `variation_ids` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation_ids: Option<Vec<String>>,
    /// Backend `ecommerce_product_variants` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_product_variants: Option<Vec<RelatedObject>>,
    /// Backend `interval` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u64>,
    /// Backend `unit` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Backend `order_cap` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_cap: Option<u64>,
    /// Backend `start_before_time` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_before_time: Option<ConfigurationValue>,
    /// Backend `start_before_time_unit` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_before_time_unit: Option<String>,
    /// Backend `start_after_time` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after_time: Option<ConfigurationValue>,
    /// Backend `start_after_time_unit` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after_time_unit: Option<String>,
    /// Backend `send_hour` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_hour: Option<String>,
    /// Backend `send_minute` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_minute: Option<String>,
    /// Backend `use_timezone` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_timezone: Option<String>,
    /// Backend `timezone_id` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<ConfigurationId>,
    /// Backend `timezone` value, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<crate::models::reference::Timezone>,
    /// Unrecognized future fields; known fields never fall back to this map.
    #[serde(flatten)]
    pub extensions: ExtensionFields,
}
fn empty_segment<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<RelatedObject>, D::Error> {
    let value = Option::<serde_json::Value>::deserialize(d)?;
    match value {
        None => Ok(None),
        Some(serde_json::Value::Array(values)) if values.is_empty() => Ok(None),
        Some(value) => serde_json::from_value(value)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

/// One human-readable rule shown for an automatically generated segment.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct SegmentDisplayRule {
    /// Icon name.
    pub icon: String,
    /// Human-readable rule label.
    pub label: String,
}

impl SegmentTriggerConfiguration {
    /// Select the segment whose members enter the workflow.
    pub fn new(segment: impl Into<ConfigurationId>) -> Self {
        Self {
            segment_id: segment.into().into(),
            ..Self::default()
        }
    }
}
impl LinkTriggerConfiguration {
    /// Select the tracked link.
    pub fn new(link: impl Into<String>) -> Self {
        Self {
            link: link.into().into(),
        }
    }
}
impl FieldTriggerConfiguration {
    /// Select the field and operator. Set value for operators requiring a comparison.
    pub fn new(field: impl Into<ConfigurationId>, operator: impl Into<String>) -> Self {
        Self {
            field_id: field.into().into(),
            field_operator: operator.into().into(),
            ..Self::default()
        }
    }
}
impl DateTriggerConfiguration {
    /// Set the required date field, groups, and zero-padded send time.
    pub fn new(
        field: impl Into<ConfigurationId>,
        groups: Vec<String>,
        hour: impl Into<String>,
        minute: impl Into<String>,
    ) -> Self {
        Self {
            field_id: field.into().into(),
            group_ids: groups.into(),
            send_hour: hour.into().into(),
            send_minute: minute.into().into(),
            ..Self::default()
        }
    }
}
impl FormTriggerConfiguration {
    /// Select a form by type and identifier.
    pub fn new(kind: impl Into<String>, id: impl Into<ConfigurationId>) -> Self {
        Self {
            form_type: kind.into().into(),
            form_id: id.into().into(),
        }
    }
}
impl ShopTriggerConfiguration {
    /// Select the shop.
    pub fn new(id: impl Into<ConfigurationId>) -> Self {
        Self {
            ecommerce_shop_id: id.into().into(),
        }
    }
}
impl ProductTriggerConfiguration {
    /// Select the shop and product. Omitted variants match all variants.
    pub fn new(shop: impl Into<ConfigurationId>, product: impl Into<ConfigurationId>) -> Self {
        Self {
            ecommerce_shop_id: shop.into().into(),
            ecommerce_product_id: product.into().into(),
            ..Self::default()
        }
    }
}
impl CategoryTriggerConfiguration {
    /// Select the shop and category.
    pub fn new(shop: impl Into<ConfigurationId>, category: impl Into<ConfigurationId>) -> Self {
        Self {
            ecommerce_shop_id: shop.into().into(),
            ecommerce_category_id: category.into().into(),
        }
    }
}
impl AbandonedTriggerConfiguration {
    /// Select the shop and waiting interval.
    pub fn new(shop: impl Into<ConfigurationId>, unit: DurationUnit, value: u64) -> Self {
        Self {
            ecommerce_shop_id: shop.into().into(),
            unit: unit.into(),
            value: value.into(),
        }
    }
}
impl PurchaseFrequencyConfiguration {
    /// Select the shop, purchase interval, and waiting period.
    pub fn new(
        shop: impl Into<ConfigurationId>,
        interval: u64,
        unit: DurationUnit,
        value: u64,
    ) -> Self {
        Self {
            ecommerce_shop_id: shop.into().into(),
            interval: interval.into(),
            unit: unit.into(),
            value: value.into(),
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::automation_triggers::Trigger,
        resources::{
            automation_actions::AutomationTriggerDefinition,
            automation_triggers::{CreateAutomationTrigger, UpdateAutomationTrigger},
        },
    };
    use serde_json::json;

    fn assert_trigger(configuration: TriggerConfiguration, kind: &str, data: serde_json::Value) {
        let request = CreateAutomationTrigger::configured(configuration).unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"type":kind,"data":data})
        );
    }

    #[test]
    fn known_trigger_fields_cannot_be_inserted_through_extensions() {
        assert!(
            CreateAutomationTrigger::new("field_updated")
                .extension_setting("field_id", true)
                .is_err()
        );
        assert!(
            UpdateAutomationTrigger::new()
                .extension_setting("value", json!([]))
                .is_err()
        );
        assert!(
            AutomationTriggerDefinition::new("subscriber_joins_group")
                .extension_setting("exit_when_no_longer_matches", "wrong")
                .is_err()
        );
    }

    #[test]
    fn trigger_group_configuration_preserves_null_and_false() {
        let config = GroupTriggerConfiguration {
            exclude_group_ids: Patch::Null,
            exit_when_no_longer_matches: false.into(),
            ..GroupTriggerConfiguration::new(["0001"])
        };
        let request =
            CreateAutomationTrigger::configured(TriggerConfiguration::Group(config)).unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"type":"subscriber_joins_group","data":{"group_ids":["0001"],"exclude_group_ids":null,"exit_when_no_longer_matches":false}})
        );
    }

    #[test]
    fn replacement_trigger_configuration_is_flattened_without_duplicate_group_keys() {
        let request = AutomationTriggerDefinition::new("subscriber_joins_group")
            .group_ids(["old"])
            .configuration(TriggerConfiguration::Group(GroupTriggerConfiguration::new(
                ["new"],
            )))
            .unwrap()
            .group_ids(["final"]);
        assert_eq!(
            serde_json::to_string(&request)
                .unwrap()
                .matches("group_ids")
                .count(),
            1
        );
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"type":"subscriber_joins_group","group_ids":["final"]})
        );
    }

    #[test]
    fn typed_product_trigger_preserves_null_variants() {
        let config = ProductTriggerConfiguration {
            variation_ids: Patch::Null,
            ..ProductTriggerConfiguration::new("001", "002")
        };
        let request = CreateAutomationTrigger::configured(
            TriggerConfiguration::PurchasedSpecificProduct(config),
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"type":"purchased_specific_product","data":{"ecommerce_shop_id":"001","ecommerce_product_id":"002","variation_ids":null}})
        );
    }

    #[test]
    fn trigger_properties_decode_related_shop_and_numeric_interval() {
        let trigger: Trigger = serde_json::from_value(json!({"id":"01","type":"purchase_frequency","ecommerce_shop_id":"02","ecommerce_shop":{"id":"02","name":"Shop","url":"https://example.test","is_internal":false},"interval":3,"value":4,"unit":"days","order_cap":9})).unwrap();
        assert_eq!(trigger.settings.interval, Some(3));
        assert_eq!(
            trigger.settings.ecommerce_shop.unwrap().id,
            ConfigurationId::Text("02".into())
        );
        assert!(trigger.settings.extensions.is_empty());
    }

    #[test]
    fn missing_segment_empty_array_is_decoded_as_absent() {
        let trigger: Trigger = serde_json::from_value(
            json!({"id":"1","type":"subscriber_joins_segment","segment":[]}),
        )
        .unwrap();
        assert!(trigger.settings.segment.is_none());
    }

    #[test]
    fn malformed_known_trigger_field_does_not_fall_back_to_extensions() {
        assert!(
            serde_json::from_value::<Trigger>(
                json!({"id":"1","type":"purchase_frequency","interval":"not an integer"})
            )
            .is_err()
        );
    }

    #[test]
    fn future_response_fields_are_explicit_and_redacted() {
        let trigger: Trigger = serde_json::from_value(
            json!({"id":"1","type":"future_trigger","future_secret":"private-future-value"}),
        )
        .unwrap();
        assert_eq!(
            trigger.settings.extensions.get("future_secret").unwrap(),
            "private-future-value"
        );
        assert!(!format!("{trigger:?}").contains("private-future-value"));
    }

    #[test]
    fn segment_trigger_serializes_sampling_percentage() {
        let config = SegmentTriggerConfiguration {
            entry_percentage: 25.into(),
            ..SegmentTriggerConfiguration::new("01")
        };
        assert_trigger(
            TriggerConfiguration::Segment(config),
            "subscriber_joins_segment",
            json!({"segment_id":"01","entry_percentage":25}),
        );
    }

    #[test]
    fn link_trigger_serializes_tracked_url() {
        assert_trigger(
            TriggerConfiguration::ClicksLink(LinkTriggerConfiguration::new(
                "https://example.test/offer",
            )),
            "clicks_link",
            json!({"link":"https://example.test/offer"}),
        );
    }

    #[test]
    fn field_trigger_serializes_exact_scalar_comparison() {
        let config = FieldTriggerConfiguration {
            value: ConfigurationValue::from("001").into(),
            ..FieldTriggerConfiguration::new("email", "equal")
        };
        assert_trigger(
            TriggerConfiguration::FieldUpdated(config),
            "field_updated",
            json!({"field_id":"email","field_operator":"equal","value":"001"}),
        );
    }

    #[test]
    fn anniversary_trigger_keeps_zero_padded_send_time() {
        let config = DateTriggerConfiguration::new("01", vec!["02".into()], "09", "00");
        assert_trigger(
            TriggerConfiguration::Anniversary(config),
            "anniversary",
            json!({"field_id":"01","group_ids":["02"],"send_hour":"09","send_minute":"00"}),
        );
    }

    #[test]
    fn exact_date_trigger_keeps_typed_offset_unit() {
        let config = DateTriggerConfiguration {
            start_before_time: 2.into(),
            start_before_time_unit: DateOffsetUnit::Day.into(),
            ..DateTriggerConfiguration::new("01", vec![], "10", "30")
        };
        assert_trigger(
            TriggerConfiguration::ExactDate(config),
            "exact_date",
            json!({"field_id":"01","group_ids":[],"send_hour":"10","send_minute":"30","start_before_time":2,"start_before_time_unit":"day"}),
        );
    }

    #[test]
    fn form_trigger_serializes_type_and_identifier() {
        assert_trigger(
            TriggerConfiguration::FormCompleted(FormTriggerConfiguration::new("embedded", "01")),
            "form_completed",
            json!({"form_type":"embedded","form_id":"01"}),
        );
    }

    #[test]
    fn any_product_trigger_serializes_shop() {
        assert_trigger(
            TriggerConfiguration::PurchasedAnyProduct(ShopTriggerConfiguration::new("01")),
            "purchased_any_product",
            json!({"ecommerce_shop_id":"01"}),
        );
    }

    #[test]
    fn category_trigger_serializes_shop_and_category() {
        assert_trigger(
            TriggerConfiguration::PurchasedFromCategory(CategoryTriggerConfiguration::new(
                "01", "02",
            )),
            "purchased_from_category",
            json!({"ecommerce_shop_id":"01","ecommerce_category_id":"02"}),
        );
    }

    #[test]
    fn abandoned_cart_trigger_uses_hours_unit() {
        assert_trigger(
            TriggerConfiguration::AbandonedCart(AbandonedTriggerConfiguration::new(
                "01",
                DurationUnit::Hours,
                2,
            )),
            "abandoned_cart",
            json!({"ecommerce_shop_id":"01","unit":"hours","value":2}),
        );
    }

    #[test]
    fn abandoned_checkout_trigger_uses_days_unit() {
        assert_trigger(
            TriggerConfiguration::AbandonedCheckout(AbandonedTriggerConfiguration::new(
                "01",
                DurationUnit::Days,
                1,
            )),
            "abandoned_checkout",
            json!({"ecommerce_shop_id":"01","unit":"days","value":1}),
        );
    }

    #[test]
    fn purchase_frequency_trigger_serializes_interval_and_cap() {
        let config = PurchaseFrequencyConfiguration {
            order_cap: 10.into(),
            ..PurchaseFrequencyConfiguration::new("01", 3, DurationUnit::Weeks, 2)
        };
        assert_trigger(
            TriggerConfiguration::PurchaseFrequency(config),
            "purchase_frequency",
            json!({"ecommerce_shop_id":"01","interval":3,"unit":"weeks","value":2,"order_cap":10}),
        );
    }

    #[test]
    fn trigger_request_type_mismatch_is_a_local_error() {
        assert!(
            CreateAutomationTrigger::new("field_updated")
                .configuration(TriggerConfiguration::Group(GroupTriggerConfiguration::new(
                    ["1"]
                )))
                .is_err()
        );
        assert!(
            AutomationTriggerDefinition::new("field_updated")
                .configuration(TriggerConfiguration::Group(GroupTriggerConfiguration::new(
                    ["1"]
                )))
                .is_err()
        );
    }
}
