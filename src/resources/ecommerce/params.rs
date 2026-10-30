//! Typed e-commerce request parameters.
use crate::models::ecommerce::wire;
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::BTreeMap;
/// Documented OrderStatus values.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum OrderStatus {
    /// Pending.
    #[serde(rename = "pending")]
    Pending,
    /// Complete.
    #[serde(rename = "complete")]
    Complete,
}
impl OrderStatus {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Complete => "complete",
        }
    }
}
/// Documented ProductType values.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ProductType {
    /// OneTime.
    #[serde(rename = "one-time")]
    OneTime,
    /// Subscription.
    #[serde(rename = "subscription")]
    Subscription,
    /// Booking.
    #[serde(rename = "booking")]
    Booking,
}
impl ProductType {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::OneTime => "one-time",
            Self::Subscription => "subscription",
            Self::Booking => "booking",
        }
    }
}
/// Documented PriceType values.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum PriceType {
    /// Fixed.
    #[serde(rename = "fixed")]
    Fixed,
    /// Increment.
    #[serde(rename = "increment")]
    Increment,
}
/// Documented ProductAction values.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ProductAction {
    /// MoveToGroup.
    #[serde(rename = "move_to_group")]
    MoveToGroup,
    /// RemoveFromGroup.
    #[serde(rename = "remove_from_group")]
    RemoveFromGroup,
    /// UpdateCustomField.
    #[serde(rename = "update_custom_field")]
    UpdateCustomField,
}
/// Parameters for SaveShop; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize)]
pub struct SaveShop {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_access_data"
    )]
    access_data: Option<crate::models::SecretString>,
    name: String,
    url: String,
    currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_popups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}
impl SaveShop {
    /// Set platform-specific access data, redacted from debug output.
    pub fn access_data(mut self, value: impl Into<String>) -> Self {
        self.access_data = Some(crate::models::SecretString::new(value));
        self
    }

    /// Supply the required request fields.
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        currency: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            currency: currency.into(),
            access_data: None,
            platform: None,
            group_id: None,
            enable_popups: None,
            enabled: None,
        }
    }
    /// Set platform.
    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }
    /// Set group id.
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }
    /// Set enable popups.
    pub fn enable_popups(mut self, value: bool) -> Self {
        self.enable_popups = Some(value);
        self
    }
    /// Set enabled.
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
}
/// Parameters for SaveCategory; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize)]
pub struct SaveCategory {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_from_automations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
}
impl SaveCategory {
    /// Supply the required request fields.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            exclude_from_automations: None,
            resource_id: None,
        }
    }
    /// Set exclude from automations.
    pub fn exclude_from_automations(mut self, value: bool) -> Self {
        self.exclude_from_automations = Some(value);
        self
    }
    /// Set resource id.
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }
}
/// Parameters for SaveProduct; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize)]
pub struct SaveProduct {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_from_automations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<ProductType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<ProductSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<SaveDiscount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<Vec<SaveProductAsset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variants: Option<Vec<SaveVariant>>,
}
impl SaveProduct {
    /// Supply fields required when creating or updating a product in the internal MailerLite shop.
    pub fn mailerlite(
        name: impl Into<String>,
        price: Decimal,
        currency: impl Into<String>,
        settings: ProductSettings,
    ) -> Self {
        Self::new(name)
            .price(price)
            .currency(currency)
            .settings(settings)
    }

    /// Supply the required request fields.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            price: None,
            exclude_from_automations: None,
            enabled: None,
            url: None,
            image: None,
            short_description: None,
            description: None,
            resource_id: None,
            currency: None,
            kind: None,
            settings: None,
            discounts: None,
            assets: None,
            categories: None,
            variants: None,
        }
    }
    /// Set price.
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set exclude from automations.
    pub fn exclude_from_automations(mut self, value: bool) -> Self {
        self.exclude_from_automations = Some(value);
        self
    }
    /// Set enabled.
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Set url.
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    /// Set image.
    pub fn image(mut self, value: impl Into<String>) -> Self {
        self.image = Some(value.into());
        self
    }
    /// Set short description.
    pub fn short_description(mut self, value: impl Into<String>) -> Self {
        self.short_description = Some(value.into());
        self
    }
    /// Set description.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Set resource id.
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }
    /// Set currency.
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }
    /// Set kind.
    pub fn kind(mut self, value: ProductType) -> Self {
        self.kind = Some(value);
        self
    }
    /// Set settings.
    pub fn settings(mut self, value: ProductSettings) -> Self {
        self.settings = Some(value);
        self
    }
    /// Set discounts.
    pub fn discounts(mut self, value: Vec<SaveDiscount>) -> Self {
        self.discounts = Some(value);
        self
    }
    /// Supply URL assets for an internal digital product.
    ///
    /// File uploads require multipart transport and are not represented by this
    /// JSON request. Booking products have an asset generated by the server.
    pub fn assets(mut self, value: Vec<SaveProductAsset>) -> Self {
        self.assets = Some(value);
        self
    }
    /// Assign category identifiers to a product in an external shop.
    pub fn categories(mut self, value: Vec<String>) -> Self {
        self.categories = Some(value);
        self
    }
    /// Supply variants for an external-shop product.
    pub fn variants(mut self, value: Vec<SaveVariant>) -> Self {
        self.variants = Some(value);
        self
    }
}
/// A URL asset supplied when creating or updating an internal digital product.
#[derive(Debug, Clone, Serialize)]
pub struct SaveProductAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    #[serde(rename = "type")]
    kind: &'static str,
    url: String,
}
impl SaveProductAsset {
    /// Name an asset and supply the URL delivered to the customer.
    pub fn url(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            kind: "url",
            url: url.into(),
        }
    }
    /// Identify an existing asset while updating a product.
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }
}
/// Parameters for SaveDiscount; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize, Default)]
pub struct SaveDiscount {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    value: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_percentage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<String>,
}
impl SaveDiscount {
    /// Supply the required request fields.
    pub fn new() -> Self {
        Self {
            name: None,
            value: None,
            is_percentage: None,
            enabled: None,
            start_date: None,
            end_date: None,
        }
    }
    /// Set name.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Set value.
    pub fn value(mut self, value: Decimal) -> Self {
        self.value = Some(value);
        self
    }
    /// Set is percentage.
    pub fn is_percentage(mut self, value: bool) -> Self {
        self.is_percentage = Some(value);
        self
    }
    /// Set enabled.
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Set start date.
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }
    /// Set end date.
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }
}
/// Parameters for SaveCustomer; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize)]
pub struct SaveCustomer {
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepts_marketing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    total_spent: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_subscriber: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
}
impl SaveCustomer {
    /// Supply the required request fields.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            accepts_marketing: None,
            total_spent: None,
            create_subscriber: None,
            name: None,
            last_name: None,
            resource_id: None,
        }
    }
    /// Set accepts marketing.
    pub fn accepts_marketing(mut self, value: bool) -> Self {
        self.accepts_marketing = Some(value);
        self
    }
    /// Set total spent.
    pub fn total_spent(mut self, value: Decimal) -> Self {
        self.total_spent = Some(value);
        self
    }
    /// Set create subscriber.
    pub fn create_subscriber(mut self, value: bool) -> Self {
        self.create_subscriber = Some(value);
        self
    }
    /// Set name.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Set last name.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }
    /// Set resource id.
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }
}
/// Parameters for UpdateCart; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCart {
    #[serde(skip_serializing_if = "Option::is_none")]
    checkout_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    cart_total: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecommerce_customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
}
impl UpdateCart {
    /// Supply the required request fields.
    pub fn new() -> Self {
        Self {
            checkout_url: None,
            cart_total: None,
            ecommerce_customer_id: None,
            resource_id: None,
        }
    }
    /// Set checkout url.
    pub fn checkout_url(mut self, value: impl Into<String>) -> Self {
        self.checkout_url = Some(value.into());
        self
    }
    /// Set cart total.
    pub fn cart_total(mut self, value: Decimal) -> Self {
        self.cart_total = Some(value);
        self
    }
    /// Set ecommerce customer id.
    pub fn ecommerce_customer_id(mut self, value: impl Into<String>) -> Self {
        self.ecommerce_customer_id = Some(value.into());
        self
    }
    /// Set resource id.
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }
}
/// Parameters for SaveCartItem; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize)]
pub struct SaveCartItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    ecommerce_product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecommerce_product_variant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
}
impl SaveCartItem {
    /// Reference an external product ID when the shop scope uses resource-ID addressing.
    pub fn external_product(id: impl Into<String>) -> Self {
        let mut item = Self::new("");
        item.ecommerce_product_id = None;
        item.product_resource_id = Some(id.into());
        item
    }

    /// Supply the required request fields.
    pub fn new(ecommerce_product_id: impl Into<String>) -> Self {
        Self {
            ecommerce_product_id: Some(ecommerce_product_id.into()),
            product_resource_id: None,
            ecommerce_product_variant_id: None,
            variant: None,
            quantity: None,
            price: None,
            resource_id: None,
        }
    }
    /// Set ecommerce product variant id.
    pub fn ecommerce_product_variant_id(mut self, value: impl Into<String>) -> Self {
        self.ecommerce_product_variant_id = Some(value.into());
        self
    }
    /// Set variant.
    pub fn variant(mut self, value: impl Into<String>) -> Self {
        self.variant = Some(value.into());
        self
    }
    /// Set quantity.
    pub fn quantity(mut self, value: u64) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set price.
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set resource id.
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }
}
/// Parameters for OrderCart; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize)]
pub struct OrderCart {
    items: Vec<SaveCartItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checkout_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    cart_total: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
}
impl OrderCart {
    /// Supply the required request fields.
    pub fn new(items: Vec<SaveCartItem>) -> Self {
        Self {
            items,
            checkout_url: None,
            cart_total: None,
            resource_id: None,
        }
    }
    /// Set checkout url.
    pub fn checkout_url(mut self, value: impl Into<String>) -> Self {
        self.checkout_url = Some(value.into());
        self
    }
    /// Set cart total.
    pub fn cart_total(mut self, value: Decimal) -> Self {
        self.cart_total = Some(value);
        self
    }
    /// Set resource id.
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }
}
/// Parameters for CreateOrder; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrder {
    customer: SaveCustomer,
    cart: OrderCart,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    total_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
}
impl CreateOrder {
    /// Supply the required request fields.
    pub fn new(customer: SaveCustomer, cart: OrderCart) -> Self {
        Self {
            customer,
            cart,
            status: None,
            total_price: None,
            resource_id: None,
        }
    }
    /// Set status.
    pub fn status(mut self, value: OrderStatus) -> Self {
        self.status = Some(value);
        self
    }
    /// Set total price.
    pub fn total_price(mut self, value: Decimal) -> Self {
        self.total_price = Some(value);
        self
    }
    /// Set resource id.
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }
}
/// Parameters for UpdateOrder; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    total_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
}
impl UpdateOrder {
    /// Supply the required request fields.
    pub fn new() -> Self {
        Self {
            status: None,
            total_price: None,
            resource_id: None,
        }
    }
    /// Set status.
    pub fn status(mut self, value: OrderStatus) -> Self {
        self.status = Some(value);
        self
    }
    /// Set total price.
    pub fn total_price(mut self, value: Decimal) -> Self {
        self.total_price = Some(value);
        self
    }
    /// Set resource id.
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }
}
/// Parameters for SaveVariant; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize)]
pub struct SaveVariant {
    name: String,
    attributes: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_type: Option<PriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
}
impl SaveVariant {
    /// Supply the required request fields.
    pub fn new(name: impl Into<String>, attributes: BTreeMap<String, String>) -> Self {
        Self {
            name: name.into(),
            attributes,
            price_type: None,
            price: None,
            resource_id: None,
            image_url: None,
        }
    }
    /// Set price type.
    pub fn price_type(mut self, value: PriceType) -> Self {
        self.price_type = Some(value);
        self
    }
    /// Set price.
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set resource id.
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }
    /// Set image url.
    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }
}
/// Parameters for ProductSettings; omitted optional fields are not sent.
#[derive(Debug, Clone, Serialize)]
pub struct ProductSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    checkout_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_tax_id: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_terms: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    booking_group_max_participants: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    booking_group_display_remaining_slots: Option<bool>,
    action: ProductAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_on_field: Option<String>,
    #[serde(skip_serializing_if = "crate::models::Patch::is_unset")]
    field_value: crate::models::Patch<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    free: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    booking_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "wire::serialize_optional_decimal")]
    duration: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_integration_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    booking_window_days: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_between_meetings: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_between_meetings_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_notice_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_notice_period_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_availability_days: Option<BTreeMap<String, Option<[String; 2]>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    booking_group_date_time_utc: Option<String>,
}
impl ProductSettings {
    /// Select the checkout provider, for example `stripe`.
    pub fn checkout_type(mut self, value: impl Into<String>) -> Self {
        self.checkout_type = Some(value.into());
        self
    }
    /// Request a tax identifier at checkout.
    pub fn enable_tax_id(mut self, value: bool) -> Self {
        self.enable_tax_id = Some(value);
        self
    }
    /// Require acceptance of checkout terms.
    pub fn enable_terms(mut self, value: bool) -> Self {
        self.enable_terms = Some(value);
        self
    }
    /// Set maximum group-booking participants (server accepts 1 through 1000).
    pub fn booking_group_max_participants(mut self, value: u64) -> Self {
        self.booking_group_max_participants = Some(value);
        self
    }
    /// Show remaining booking slots at checkout.
    pub fn booking_group_display_remaining_slots(mut self, value: bool) -> Self {
        self.booking_group_display_remaining_slots = Some(value);
        self
    }

    /// Supply the required request fields.
    pub fn new(action: ProductAction) -> Self {
        Self {
            checkout_type: None,
            enable_tax_id: None,
            enable_terms: None,
            booking_group_max_participants: None,
            booking_group_display_remaining_slots: None,
            action,
            group_id: None,
            field_id: None,
            action_on_field: None,
            field_value: crate::models::Patch::Unset,
            free: None,
            booking_type: None,
            duration: None,
            duration_unit: None,
            calendar_integration_type: None,
            external_link: None,
            location: None,
            location_address: None,
            location_link: None,
            location_instructions: None,
            timezone_id: None,
            booking_window_days: None,
            buffer_between_meetings: None,
            buffer_between_meetings_unit: None,
            minimum_notice_period: None,
            minimum_notice_period_unit: None,
            custom_availability_days: None,
            booking_group_date_time_utc: None,
        }
    }
    /// Set group id.
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }
    /// Set field id.
    pub fn field_id(mut self, value: impl Into<String>) -> Self {
        self.field_id = Some(value.into());
        self
    }
    /// Set action on field.
    pub fn action_on_field(mut self, value: impl Into<String>) -> Self {
        self.action_on_field = Some(value.into());
        self
    }
    /// Set field value.
    pub fn field_value(mut self, value: impl Into<String>) -> Self {
        self.field_value = crate::models::Patch::Value(value.into());
        self
    }
    /// Send an explicit null field value for an action that permits it, such as
    /// `set_to_current_date`. The server validates action-specific requirements.
    pub fn clear_field_value(mut self) -> Self {
        self.field_value = crate::models::Patch::Null;
        self
    }
    /// Set free.
    pub fn free(mut self, value: bool) -> Self {
        self.free = Some(value);
        self
    }
    /// Set booking type.
    pub fn booking_type(mut self, value: impl Into<String>) -> Self {
        self.booking_type = Some(value.into());
        self
    }
    /// Set duration.
    pub fn duration(mut self, value: Decimal) -> Self {
        self.duration = Some(value);
        self
    }
    /// Set duration unit.
    pub fn duration_unit(mut self, value: impl Into<String>) -> Self {
        self.duration_unit = Some(value.into());
        self
    }
    /// Set calendar integration type.
    pub fn calendar_integration_type(mut self, value: impl Into<String>) -> Self {
        self.calendar_integration_type = Some(value.into());
        self
    }
    /// Set external link.
    pub fn external_link(mut self, value: impl Into<String>) -> Self {
        self.external_link = Some(value.into());
        self
    }
    /// Set location.
    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }
    /// Set location address.
    pub fn location_address(mut self, value: impl Into<String>) -> Self {
        self.location_address = Some(value.into());
        self
    }
    /// Set location link.
    pub fn location_link(mut self, value: impl Into<String>) -> Self {
        self.location_link = Some(value.into());
        self
    }
    /// Set location instructions.
    pub fn location_instructions(mut self, value: impl Into<String>) -> Self {
        self.location_instructions = Some(value.into());
        self
    }
    /// Set timezone id.
    pub fn timezone_id(mut self, value: u64) -> Self {
        self.timezone_id = Some(value);
        self
    }
    /// Set booking window days.
    pub fn booking_window_days(mut self, value: u64) -> Self {
        self.booking_window_days = Some(value);
        self
    }
    /// Set buffer between meetings.
    pub fn buffer_between_meetings(mut self, value: u64) -> Self {
        self.buffer_between_meetings = Some(value);
        self
    }
    /// Set buffer between meetings unit.
    pub fn buffer_between_meetings_unit(mut self, value: impl Into<String>) -> Self {
        self.buffer_between_meetings_unit = Some(value.into());
        self
    }
    /// Set minimum notice period.
    pub fn minimum_notice_period(mut self, value: u64) -> Self {
        self.minimum_notice_period = Some(value);
        self
    }
    /// Set minimum notice period unit.
    pub fn minimum_notice_period_unit(mut self, value: impl Into<String>) -> Self {
        self.minimum_notice_period_unit = Some(value.into());
        self
    }
    /// Set custom availability days.
    pub fn custom_availability_days(mut self, value: BTreeMap<String, [String; 2]>) -> Self {
        self.custom_availability_days = Some(
            value
                .into_iter()
                .map(|(day, times)| (day, Some(times)))
                .collect(),
        );
        self
    }
    /// Mark a weekday unavailable by sending an explicit null time range.
    pub fn unavailable_day(mut self, day: impl Into<String>) -> Self {
        self.custom_availability_days
            .get_or_insert_with(BTreeMap::new)
            .insert(day.into(), None);
        self
    }
    /// Set booking group date time utc.
    pub fn booking_group_date_time_utc(mut self, value: impl Into<String>) -> Self {
        self.booking_group_date_time_utc = Some(value.into());
        self
    }
}
/// A product imported together with category assignments.
#[derive(Debug, Clone, Serialize)]
pub struct ImportProduct {
    #[serde(flatten)]
    product: SaveProduct,
}
impl ImportProduct {
    /// Supply the product definition.
    pub fn new(product: SaveProduct) -> Self {
        Self { product }
    }
    /// Assign category IDs, or external IDs with resource-ID addressing enabled.
    pub fn categories(mut self, categories: Vec<String>) -> Self {
        self.product.categories = Some(categories);
        self
    }
}
/// A customer accepted for background import.
#[derive(Debug, Clone, Serialize)]
pub struct ImportCustomer {
    #[serde(flatten)]
    customer: SaveCustomer,
    #[serde(skip_serializing_if = "Option::is_none")]
    orders_count: Option<u64>,
}
impl ImportCustomer {
    /// Supply customer details.
    pub fn new(customer: SaveCustomer) -> Self {
        Self {
            customer,
            orders_count: None,
        }
    }
    /// Set historical number of orders.
    pub fn orders_count(mut self, count: u64) -> Self {
        self.orders_count = Some(count);
        self
    }
}

fn serialize_access_data<S: serde::Serializer>(
    value: &Option<crate::models::SecretString>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(secret) => serializer.serialize_str(secret.expose_secret()),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::str::FromStr;

    #[test]
    fn external_product_id_uses_distinct_wire_key() {
        assert_eq!(
            serde_json::to_value(SaveCartItem::external_product("sku-1")).unwrap(),
            json!({"product_resource_id":"sku-1"})
        );
    }

    #[test]
    fn product_update_preserves_explicit_empty_assets() {
        assert_eq!(
            serde_json::to_value(SaveProduct::new("Guide").assets(vec![])).unwrap(),
            json!({"name":"Guide","assets":[]})
        );
    }

    #[test]
    fn product_settings_distinguish_omission_from_explicit_null_field_value() {
        let omitted = ProductSettings::new(ProductAction::UpdateCustomField).field_id("3");
        assert_eq!(
            serde_json::to_value(&omitted).unwrap(),
            json!({"action":"update_custom_field","field_id":"3"})
        );
        assert_eq!(
            serde_json::to_value(
                omitted
                    .action_on_field("set_to_current_date")
                    .clear_field_value()
            )
            .unwrap(),
            json!({"action":"update_custom_field","field_id":"3","action_on_field":"set_to_current_date","field_value":null})
        );
    }

    #[test]
    fn product_settings_serialize_unavailable_days_as_null() {
        let settings = ProductSettings::new(ProductAction::MoveToGroup)
            .custom_availability_days(BTreeMap::from([(
                "monday".into(),
                ["09:00".into(), "17:00".into()],
            )]))
            .unavailable_day("sunday");
        assert_eq!(
            serde_json::to_value(settings).unwrap(),
            json!({"action":"move_to_group","custom_availability_days":{"monday":["09:00","17:00"],"sunday":null}})
        );
    }

    #[test]
    fn import_category_override_does_not_emit_duplicate_keys() {
        let request = ImportProduct::new(SaveProduct::new("Shirt").categories(vec!["old".into()]))
            .categories(vec!["new".into()]);
        let body = serde_json::to_string(&request).unwrap();
        assert_eq!(body.matches("\"categories\"").count(), 1);
        assert_eq!(
            serde_json::from_str::<Value>(&body).unwrap(),
            json!({"name":"Shirt","categories":["new"]})
        );
    }

    #[test]
    fn decimal_request_serialization_is_exact() {
        let product = SaveProduct::new("Product")
            .price(Decimal::from_str("1234567890.123456789012345678").unwrap());
        assert!(
            serde_json::to_string(&product)
                .unwrap()
                .contains("1234567890.123456789012345678")
        );
    }

    #[test]
    fn access_credentials_are_serialized_but_redacted_from_debug() {
        let shop =
            SaveShop::new("Shop", "https://example.com", "USD").access_data("secret-platform-key");
        assert!(!format!("{shop:?}").contains("secret-platform-key"));
        assert_eq!(
            serde_json::to_value(&shop).unwrap()["access_data"],
            "secret-platform-key"
        );
    }
}
