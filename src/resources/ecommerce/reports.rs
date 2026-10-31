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
    ///
    /// The shop ID is a query parameter; its value is validated by MailerLite.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let campaign_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .campaign_stats(campaign_id, shop_id)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
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
    ///
    /// The shop ID is a query parameter; its value is validated by MailerLite.
    ///
    /// Constructing this builder makes no request. Fetch a page with [`List::send`], or
    /// enable `stream` for lazy consumption.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let campaign_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .campaign_orders(campaign_id, shop_id)
    ///     .limit(25)
    ///     .send()
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Builder construction is infallible. Parameter validation and request failures
    /// are reported when fetching pages; see [`List::send`].
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
    ///
    /// The shop ID is a query parameter; its value is validated by MailerLite.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let subscriber_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .subscriber_stats(subscriber_id, shop_id)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
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
    ///
    /// The shop ID is a query parameter; its value is validated by MailerLite.
    ///
    /// Constructing this builder makes no request. Fetch a page with [`List::send`], or
    /// enable `stream` for lazy consumption.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let subscriber_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .subscriber_orders(subscriber_id, shop_id)
    ///     .limit(25)
    ///     .send()
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Builder construction is infallible. Parameter validation and request failures
    /// are reported when fetching pages; see [`List::send`].
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
    ///
    /// Dates and account access are validated by the server. External-resource-ID mode
    /// is not applied to this report.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let start_date = "2026-01-01";
    /// let end_date = "2026-01-31";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .stats(start_date, end_date)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
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
