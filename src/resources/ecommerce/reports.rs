//! Campaign, subscriber, and shop e-commerce reports.
use super::{Ecommerce, List, Scope, ShopScope};
use crate::{
    Response, Result,
    http::OperationPolicy,
    models::ecommerce::{ReportOrder, StatsReport},
};
use reqwest::Method;
impl<'a> Ecommerce<'a> {
    /// Report campaign revenue in a shop.
    pub async fn campaign_stats(
        &self,
        campaign: impl AsRef<str>,
        shop: impl AsRef<str>,
    ) -> Result<Response<StatsReport>> {
        self.client
            .data(
                Method::GET,
                &["campaigns", campaign.as_ref(), "reports", "ecommerce"],
                &[("shop_id".into(), shop.as_ref().into())],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// List orders earned by a campaign in a shop.
    pub fn campaign_orders(
        &self,
        campaign: impl AsRef<str>,
        shop: impl AsRef<str>,
    ) -> List<'a, ReportOrder> {
        let mut scope = Scope::new(
            self.client,
            vec![
                "campaigns".into(),
                campaign.as_ref().into(),
                "reports".into(),
                "ecommerce".into(),
                "orders".into(),
            ],
        );
        scope.query.push(("shop_id".into(), shop.as_ref().into()));
        List::new(scope, false)
    }
    /// Report a subscriber's attributed purchases in a shop.
    pub async fn subscriber_stats(
        &self,
        subscriber: impl AsRef<str>,
        shop: impl AsRef<str>,
    ) -> Result<Response<StatsReport>> {
        self.client
            .data(
                Method::GET,
                &["subscribers", subscriber.as_ref(), "reports", "ecommerce"],
                &[("shop_id".into(), shop.as_ref().into())],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// List a subscriber's attributed orders in a shop.
    pub fn subscriber_orders(
        &self,
        subscriber: impl AsRef<str>,
        shop: impl AsRef<str>,
    ) -> List<'a, ReportOrder> {
        let mut scope = Scope::new(
            self.client,
            vec![
                "subscribers".into(),
                subscriber.as_ref().into(),
                "reports".into(),
                "ecommerce".into(),
                "orders".into(),
            ],
        );
        scope.query.push(("shop_id".into(), shop.as_ref().into()));
        List::new(scope, false)
    }
}
impl ShopScope<'_> {
    /// Report shop revenue over an inclusive date range (`YYYY-MM-DD`).
    pub async fn stats(
        &self,
        start_date: impl Into<String>,
        end_date: impl Into<String>,
    ) -> Result<Response<StatsReport>> {
        let mut scope = self.scope.child("reports").child("stats");
        scope.query.retain(|(key, _)| key != "with_resource_id");
        scope.query.extend([
            ("start_date".into(), start_date.into()),
            ("end_date".into(), end_date.into()),
        ]);
        scope.data(Method::GET, None).await
    }
}
