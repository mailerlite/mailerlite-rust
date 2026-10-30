//! Shops: <https://developers.mailerlite.com/ecommerce/shops>

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    MailerLite, Page, Result,
    reports::EcommerceSummary,
    types::{COUNT_QUERY, Id, Rate, Timestamp, Total, de},
};

/// A connected shop.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Shop {
    /// Shop id.
    pub id: Id,
    /// Shop name.
    pub name: String,
    /// Shop URL.
    #[serde(default)]
    pub url: String,
    /// ISO 4217 currency code.
    #[serde(default)]
    pub currency: String,
    /// Whether e-commerce features are enabled for the shop.
    #[serde(default, deserialize_with = "de::null_default")]
    pub enabled: bool,
    /// Whether pop-ups are enabled on the shop.
    #[serde(default)]
    pub enable_popups: Option<bool>,
    /// Group the shop's customers are added to.
    #[serde(default)]
    pub group: Option<ShopGroup>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// Currency symbol, e.g. `"$"`.
    #[serde(default)]
    pub currency_sign: Option<String>,
    /// Platform the shop runs on (e.g. `shopify`, or `mailerlite` for the MailerLite shop).
    #[serde(default)]
    pub platform: Option<String>,
    /// `true` for the MailerLite shop, which cannot be updated or deleted through the API.
    #[serde(default)]
    pub internal: Option<bool>,
    /// Your own id for the shop, if any.
    #[serde(default)]
    pub resource_id: Option<String>,
}

/// The group a [`Shop`] adds its customers to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ShopGroup {
    /// Group id.
    pub id: Id,
    /// Group name.
    pub name: String,
    /// Active subscribers in the group. The API sends `active_count` (the docs say `total`;
    /// both are accepted).
    #[serde(default, alias = "total", deserialize_with = "de::null_default")]
    pub active_count: u64,
    /// Open rate.
    #[serde(default)]
    pub open_rate: Rate,
    /// Click rate.
    #[serde(default)]
    pub click_rate: Rate,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
}

/// Shop revenue report from [`Shops::stats`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ShopStats {
    /// Totals for the whole period.
    #[serde(default)]
    pub ecommerce: EcommerceSummary,
    /// One entry per day in the period.
    #[serde(default, deserialize_with = "de::null_default")]
    pub series: Vec<ShopStatsDay>,
}

/// One day of [`ShopStats::series`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ShopStatsDay {
    /// `YYYY-MM-DD`.
    pub date: String,
    /// Orders attributed to campaigns.
    #[serde(default)]
    pub campaign_orders_count: u64,
    /// Orders attributed to automations.
    #[serde(default)]
    pub automation_orders_count: u64,
    /// Revenue attributed to automations.
    #[serde(default, deserialize_with = "de::number")]
    pub automation_revenue: f64,
    /// Revenue attributed to campaigns.
    #[serde(default, deserialize_with = "de::number")]
    pub campaign_revenue: f64,
}

/// Query for [`Shops::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListShops {
    /// Page size (API default 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ListShops {
    /// Default query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Page size.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 1-based page number.
    #[must_use]
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

/// Body for [`Shops::create`] and [`Shops::update`] (both require name, URL and currency).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CreateShop {
    /// Shop name.
    pub name: String,
    /// Shop URL.
    pub url: String,
    /// ISO 4217 currency code, e.g. `"EUR"`.
    pub currency: String,
    /// Platform the shop runs on, e.g. `"shopify"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Group the shop's customers are added to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Id>,
    /// Enable pop-ups (the shop must include the universal script). API default `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_popups: Option<bool>,
    /// Enable e-commerce features for the shop. API default `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Platform-specific credentials (e.g. a Shopify access token). Not needed by third
    /// party integrations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_data: Option<String>,
}

impl CreateShop {
    /// A shop called `name` at `url` selling in `currency` (ISO 4217).
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        currency: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            currency: currency.into(),
            platform: None,
            group_id: None,
            enable_popups: None,
            enabled: None,
            access_data: None,
        }
    }

    /// Platform the shop runs on.
    #[must_use]
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    /// Group the shop's customers are added to.
    #[must_use]
    pub fn group_id(mut self, group_id: impl Into<Id>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    /// Enable or disable pop-ups.
    #[must_use]
    pub fn enable_popups(mut self, enable_popups: bool) -> Self {
        self.enable_popups = Some(enable_popups);
        self
    }

    /// Enable or disable e-commerce features.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Platform-specific credentials.
    #[must_use]
    pub fn access_data(mut self, access_data: impl Into<String>) -> Self {
        self.access_data = Some(access_data.into());
        self
    }
}

#[derive(Serialize)]
struct StatsQuery<'a> {
    start_date: &'a str,
    end_date: &'a str,
}

/// Shop endpoints. Obtained with [`Ecommerce::shops`](super::Ecommerce::shops).
#[derive(Clone, Copy, Debug)]
pub struct Shops<'a> {
    pub(super) client: &'a MailerLite,
}

impl<'a> Shops<'a> {
    /// `GET /ecommerce/shops`: one page of shops. The first call creates the account's
    /// MailerLite shop (`platform: "mailerlite"`, `internal: true`).
    pub async fn list(self, query: &ListShops) -> Result<Page<Shop>> {
        self.client
            .call(Method::GET, &["ecommerce", "shops"])
            .query(query)
            .send()
            .await
    }

    /// Every shop, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(self, query: ListShops) -> impl futures_util::Stream<Item = Result<Shop>> + 'a {
        let shops = self;
        crate::pagination::stream::paged(
            query,
            move |query| async move { shops.list(&query).await },
        )
    }

    /// `GET /ecommerce/shops?limit=0`: total number of shops.
    pub async fn count(self) -> Result<u64> {
        let total: Total = self
            .client
            .call(Method::GET, &["ecommerce", "shops"])
            .query(&COUNT_QUERY)
            .send()
            .await?;
        Ok(total.total)
    }

    /// `GET /ecommerce/shops/{id}`: one shop.
    pub async fn get(self, id: impl AsRef<str>) -> Result<Shop> {
        self.client
            .call(Method::GET, &["ecommerce", "shops", id.as_ref()])
            .data()
            .await
    }

    /// `POST /ecommerce/shops`: connects a shop.
    pub async fn create(self, shop: &CreateShop) -> Result<Shop> {
        self.client
            .call(Method::POST, &["ecommerce", "shops"])
            .json(shop)
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{id}`: updates a shop. The MailerLite shop cannot be updated
    /// (`403`).
    pub async fn update(self, id: impl AsRef<str>, shop: &CreateShop) -> Result<Shop> {
        self.client
            .call(Method::PUT, &["ecommerce", "shops", id.as_ref()])
            .json(shop)
            .data()
            .await
    }

    /// `DELETE /ecommerce/shops/{id}`: deletes a shop and returns it. The MailerLite shop
    /// cannot be deleted (`403`).
    pub async fn delete(self, id: impl AsRef<str>) -> Result<Shop> {
        self.client
            .call(Method::DELETE, &["ecommerce", "shops", id.as_ref()])
            .data()
            .await
    }

    /// `GET /ecommerce/shops/{id}/reports/stats`: orders and revenue your emails earned in
    /// the shop between `start_date` and `end_date` (inclusive, `YYYY-MM-DD`).
    pub async fn stats(
        self,
        id: impl AsRef<str>,
        start_date: &str,
        end_date: &str,
    ) -> Result<ShopStats> {
        self.client
            .call(
                Method::GET,
                &["ecommerce", "shops", id.as_ref(), "reports", "stats"],
            )
            .query(&StatsQuery {
                start_date,
                end_date,
            })
            .data()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::{Value, json};

    use super::*;
    use crate::http::mock::{client, fixture};

    #[tokio::test]
    async fn list_and_count() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_shops_list"))
            .respond(200, json!({"total": 100}));

        let page = client
            .ecommerce()
            .shops()
            .list(&ListShops::new().limit(10).page(2))
            .await
            .unwrap();
        let total = client.ecommerce().shops().count().await.unwrap();

        let shop = &page.data[0];
        assert_eq!(shop.id, "31986843064993537");
        assert_eq!(shop.currency_sign.as_deref(), Some("$"));
        let group = shop.group.as_ref().unwrap();
        assert_eq!((group.id.as_str(), group.active_count), ("1", 20));
        assert_eq!(group.open_rate.string, "0%");
        assert_eq!(page.meta.per_page, 50);
        assert_eq!(total, 100);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "ecommerce/shops?limit=10&page=2");
        requests[1].assert(Method::GET, "ecommerce/shops?limit=0");
    }

    #[tokio::test]
    async fn get_create_update_delete() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_shop"))
            .respond(201, fixture("ecommerce_guide_shop"))
            .respond(200, fixture("ecommerce_shop"))
            .respond(200, fixture("ecommerce_shop"));
        let shops = client.ecommerce().shops();

        let fetched = shops.get("31986843064993537").await.unwrap();
        let created = shops
            .create(
                &CreateShop::new("Super Amazing T-Shirts", "superamazingtshirts.com", "USD")
                    .enabled(true),
            )
            .await
            .unwrap();
        shops
            .update(
                "1",
                &CreateShop::new("shop name", "https://shop-url.com", "EUR")
                    .group_id("1234567890")
                    .enable_popups(false)
                    .enabled(true)
                    .platform("shopify")
                    .access_data("token"),
            )
            .await
            .unwrap();
        let deleted = shops.delete("1").await.unwrap();

        assert_eq!(fetched.name, "shop name");
        assert_eq!(created.id, "134");
        assert_eq!(created.enable_popups, None);
        assert_eq!(created.group, None);
        assert_eq!(deleted, fetched);
        let requests = mock.requests();
        requests[0]
            .assert(Method::GET, "ecommerce/shops/31986843064993537")
            .no_body();
        requests[1]
            .assert(Method::POST, "ecommerce/shops")
            .body(&json!({
                "name": "Super Amazing T-Shirts",
                "url": "superamazingtshirts.com",
                "currency": "USD",
                "enabled": true
            }));
        requests[2]
            .assert(Method::PUT, "ecommerce/shops/1")
            .body(&json!({
                "name": "shop name",
                "url": "https://shop-url.com",
                "currency": "EUR",
                "platform": "shopify",
                "group_id": "1234567890",
                "enable_popups": false,
                "enabled": true,
                "access_data": "token"
            }));
        requests[3]
            .assert(Method::DELETE, "ecommerce/shops/1")
            .no_body();
    }

    #[tokio::test]
    async fn stats() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_shop_stats"));

        let stats = client
            .ecommerce()
            .shops()
            .stats("7", "2026-01-01", "2026-01-31")
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "ecommerce/shops/7/reports/stats?start_date=2026-01-01&end_date=2026-01-31",
        );
        assert_eq!(stats.ecommerce.orders_count, 4);
        assert!((stats.ecommerce.revenue - 40.0).abs() < f64::EPSILON);
        assert_eq!(stats.series.len(), 1);
        assert_eq!(stats.series[0].date, "2026-01-01");
        assert_eq!(stats.series[0].automation_orders_count, 1);
        assert!((stats.series[0].campaign_revenue - 30.0).abs() < f64::EPSILON);
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn list_all_walks_pages() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let shop = fixture("ecommerce_shop")["data"].clone();
        mock.respond(
            200,
            json!({"data": [shop], "meta": {"current_page": 1, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [shop], "meta": {"current_page": 2, "last_page": 2}}),
        );

        let all: Vec<Shop> = client
            .ecommerce()
            .shops()
            .list_all(ListShops::new())
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "ecommerce/shops?page=1");
        requests[1].assert(Method::GET, "ecommerce/shops?page=2");
    }

    #[test]
    fn null_flags_decode_as_false() {
        let mut shop = fixture("ecommerce_shop")["data"].clone();
        shop["enabled"] = Value::Null;
        shop["enable_popups"] = Value::Null;

        let shop: Shop = serde_json::from_value(shop).unwrap();
        assert!(!shop.enabled);
    }
}
