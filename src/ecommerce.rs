use reqwest::Method;
use serde_json::Value;

use crate::{parameter::Parameter, response::Response, MailerLite, BASE_PATH};

#[cfg(test)]
thread_local! {
    pub(crate) static TEST_BASE_PATH: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

pub(crate) fn shop_path(shop_id: &str, path: &str) -> String {
    format!("ecommerce/shops/{}{}", shop_id, path)
}

pub(crate) fn query_of(query: Parameter) -> Vec<(&'static str, String)> {
    query
        .data
        .into_iter()
        .map(|(key, value)| (key, value.to_string()))
        .collect()
}

fn url(path: &str) -> String {
    #[cfg(test)]
    if let Some(base) = TEST_BASE_PATH.with(|base| base.borrow().clone()) {
        return format!("{}/{}", base, path);
    }

    format!("{}{}", BASE_PATH, path)
}

pub(crate) async fn send(
    mailerlite: &MailerLite,
    method: Method,
    path: String,
    query: Vec<(&'static str, String)>,
    body: Option<Value>,
) -> Response {
    let mut request = mailerlite
        .client
        .request
        .request(method, url(&path))
        .query(&query);

    if let Some(body) = body {
        request = request.json(&body);
    }

    Response::new(request.send().await.expect("Failed to send request")).await
}

#[cfg(test)]
mod tests {
    use super::TEST_BASE_PATH;
    use crate::{parameter::Parameter, response::Response, MailerLite};
    use mockito::{Matcher, Server};
    use serde_json::{json, Value};
    use std::future::Future;

    async fn check<F, Fut>(
        method: &str,
        path: &str,
        query: Vec<(&str, &str)>,
        body: Option<Value>,
        call: F,
    ) where
        F: FnOnce(MailerLite) -> Fut,
        Fut: Future<Output = Response>,
    {
        let mut server = Server::new_async().await;
        TEST_BASE_PATH.with(|base| *base.borrow_mut() = Some(server.url()));

        let query_matcher = if query.is_empty() {
            Matcher::Missing
        } else {
            Matcher::AllOf(
                query
                    .into_iter()
                    .map(|(key, value)| Matcher::UrlEncoded(key.into(), value.into()))
                    .collect(),
            )
        };

        let mut mock = server
            .mock(method, path)
            .match_query(query_matcher)
            .with_status(200)
            .with_body("{}");
        if let Some(body) = body {
            mock = mock.match_body(Matcher::Json(body));
        }
        let mock = mock.create_async().await;

        let response = call(MailerLite::new("api_key".to_string())).await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    fn id(value: &str) -> String {
        value.to_string()
    }

    #[tokio::test]
    async fn shops_and_settings() {
        check("GET", "/ecommerce/shops", vec![("limit", "10")], None, |ml| async move {
            ml.ecommerce_shop().get(Parameter::new().add("limit", "10")).await
        })
        .await;
        check("GET", "/ecommerce/shops/1", vec![], None, |ml| async move {
            ml.ecommerce_shop().find(id("1")).await
        })
        .await;
        check("POST", "/ecommerce/shops", vec![], Some(json!({ "name": "Shop" })), |ml| async move {
            ml.ecommerce_shop().create(json!({ "name": "Shop" })).await
        })
        .await;
        check("PUT", "/ecommerce/shops/1", vec![], Some(json!({ "name": "New" })), |ml| async move {
            ml.ecommerce_shop().update(id("1"), json!({ "name": "New" })).await
        })
        .await;
        check("DELETE", "/ecommerce/shops/1", vec![], None, |ml| async move {
            ml.ecommerce_shop().delete(id("1")).await
        })
        .await;
        check(
            "GET",
            "/ecommerce/shops/1/reports/stats",
            vec![("start_date", "2026-10-01"), ("end_date", "2026-10-31")],
            None,
            |ml| async move {
                ml.ecommerce_shop()
                    .stats(id("1"), id("2026-10-01"), id("2026-10-31"))
                    .await
            },
        )
        .await;
        check("GET", "/ecommerce/settings", vec![], None, |ml| async move {
            ml.ecommerce_setting().find().await
        })
        .await;
    }

    #[tokio::test]
    async fn products_and_variants() {
        let variant = json!({ "name": "Large", "attributes": { "size": "L" } });

        check(
            "GET",
            "/ecommerce/shops/1/products",
            vec![("filter[enabled]", "true"), ("sort", "-name")],
            None,
            |ml| async move {
                ml.ecommerce_product()
                    .get(id("1"), Parameter::new().add("filter[enabled]", "true").add("sort", "-name"))
                    .await
            },
        )
        .await;
        check("GET", "/ecommerce/shops/1/products/7", vec![], None, |ml| async move {
            ml.ecommerce_product().find(id("1"), id("7")).await
        })
        .await;
        check("POST", "/ecommerce/shops/1/products", vec![], Some(json!({ "name": "Hat" })), |ml| async move {
            ml.ecommerce_product().create(id("1"), json!({ "name": "Hat" })).await
        })
        .await;
        check("PUT", "/ecommerce/shops/1/products/7", vec![], Some(json!({ "name": "Cap" })), |ml| async move {
            ml.ecommerce_product().update(id("1"), id("7"), json!({ "name": "Cap" })).await
        })
        .await;
        check("DELETE", "/ecommerce/shops/1/products/7", vec![], None, |ml| async move {
            ml.ecommerce_product().delete(id("1"), id("7")).await
        })
        .await;
        check(
            "PUT",
            "/ecommerce/shops/1/products/7/change-status",
            vec![],
            Some(json!({ "status": false })),
            |ml| async move { ml.ecommerce_product().change_status(id("1"), id("7"), false).await },
        )
        .await;
        check(
            "PUT",
            "/ecommerce/shops/1/products/7/categories/multiple",
            vec![],
            Some(json!({ "categories": ["5"], "replace": true })),
            |ml| async move {
                ml.ecommerce_product()
                    .assign_categories(id("1"), id("7"), vec![id("5")], true)
                    .await
            },
        )
        .await;

        check("GET", "/ecommerce/shops/1/products/7/variants", vec![], None, |ml| async move {
            ml.ecommerce_product_variant().get(id("1"), id("7")).await
        })
        .await;
        let sync_body = json!([variant.clone()]);
        check("POST", "/ecommerce/shops/1/products/7/variants", vec![], Some(sync_body.clone()), |ml| async move {
            ml.ecommerce_product_variant().sync(id("1"), id("7"), sync_body).await
        })
        .await;
        let update_body = variant.clone();
        check("PUT", "/ecommerce/shops/1/products/7/variants/3", vec![], Some(variant), |ml| async move {
            ml.ecommerce_product_variant().update(id("1"), id("7"), id("3"), update_body).await
        })
        .await;
        check("DELETE", "/ecommerce/shops/1/products/7/variants/3", vec![], None, |ml| async move {
            ml.ecommerce_product_variant().delete(id("1"), id("7"), id("3")).await
        })
        .await;
    }

    #[tokio::test]
    async fn categories() {
        check("GET", "/ecommerce/shops/1/categories", vec![], None, |ml| async move {
            ml.ecommerce_category().get(id("1"), Parameter::new()).await
        })
        .await;
        check("GET", "/ecommerce/shops/1/categories/5", vec![], None, |ml| async move {
            ml.ecommerce_category().find(id("1"), id("5")).await
        })
        .await;
        check("POST", "/ecommerce/shops/1/categories", vec![], Some(json!({ "name": "Hats" })), |ml| async move {
            ml.ecommerce_category().create(id("1"), json!({ "name": "Hats" })).await
        })
        .await;
        check("PUT", "/ecommerce/shops/1/categories/5", vec![], Some(json!({ "name": "Caps" })), |ml| async move {
            ml.ecommerce_category().update(id("1"), id("5"), json!({ "name": "Caps" })).await
        })
        .await;
        check("DELETE", "/ecommerce/shops/1/categories/5", vec![], None, |ml| async move {
            ml.ecommerce_category().delete(id("1"), id("5")).await
        })
        .await;
        check("GET", "/ecommerce/shops/1/categories/5/products", vec![], None, |ml| async move {
            ml.ecommerce_category().products(id("1"), id("5"), Parameter::new()).await
        })
        .await;
        check("PUT", "/ecommerce/shops/1/categories/5/products/7", vec![], None, |ml| async move {
            ml.ecommerce_category().assign_product(id("1"), id("5"), id("7")).await
        })
        .await;
        check("DELETE", "/ecommerce/shops/1/categories/5/products/7", vec![], None, |ml| async move {
            ml.ecommerce_category().unassign_product(id("1"), id("5"), id("7")).await
        })
        .await;
    }

    #[tokio::test]
    async fn customers_orders_and_carts() {
        check(
            "GET",
            "/ecommerce/shops/1/customers",
            vec![("filter[email]", "a@b.test")],
            None,
            |ml| async move {
                ml.ecommerce_customer()
                    .get(id("1"), Parameter::new().add("filter[email]", "a@b.test"))
                    .await
            },
        )
        .await;
        check("GET", "/ecommerce/shops/1/customers/3", vec![], None, |ml| async move {
            ml.ecommerce_customer().find(id("1"), id("3")).await
        })
        .await;
        check("POST", "/ecommerce/shops/1/customers", vec![], Some(json!({ "email": "a@b.test" })), |ml| async move {
            ml.ecommerce_customer().create(id("1"), json!({ "email": "a@b.test" })).await
        })
        .await;
        check("PUT", "/ecommerce/shops/1/customers/3", vec![], Some(json!({ "name": "Jane" })), |ml| async move {
            ml.ecommerce_customer().update(id("1"), id("3"), json!({ "name": "Jane" })).await
        })
        .await;
        check("DELETE", "/ecommerce/shops/1/customers/3", vec![], None, |ml| async move {
            ml.ecommerce_customer().delete(id("1"), id("3")).await
        })
        .await;

        check(
            "GET",
            "/ecommerce/shops/1/orders",
            vec![("filter[attributed]", "true"), ("filter[email_id]", "55")],
            None,
            |ml| async move {
                ml.ecommerce_order()
                    .get(id("1"), Parameter::new().add("filter[attributed]", "true").add("filter[email_id]", "55"))
                    .await
            },
        )
        .await;
        check("GET", "/ecommerce/shops/1/orders/9", vec![], None, |ml| async move {
            ml.ecommerce_order().find(id("1"), id("9")).await
        })
        .await;
        check("PUT", "/ecommerce/shops/1/orders/9", vec![], Some(json!({ "status": "complete" })), |ml| async move {
            ml.ecommerce_order().update(id("1"), id("9"), json!({ "status": "complete" })).await
        })
        .await;
        check("DELETE", "/ecommerce/shops/1/orders/9", vec![], None, |ml| async move {
            ml.ecommerce_order().delete(id("1"), id("9")).await
        })
        .await;

        check("GET", "/ecommerce/shops/1/carts", vec![], None, |ml| async move {
            ml.ecommerce_cart().get(id("1"), Parameter::new()).await
        })
        .await;
        check("GET", "/ecommerce/shops/1/carts/4", vec![], None, |ml| async move {
            ml.ecommerce_cart().find(id("1"), id("4")).await
        })
        .await;
        check("PUT", "/ecommerce/shops/1/carts/4", vec![], Some(json!({ "cart_total": 10 })), |ml| async move {
            ml.ecommerce_cart().update(id("1"), id("4"), json!({ "cart_total": 10 })).await
        })
        .await;
        check("GET", "/ecommerce/shops/1/carts/4/items", vec![], None, |ml| async move {
            ml.ecommerce_cart_item().get(id("1"), id("4"), Parameter::new()).await
        })
        .await;
        check("GET", "/ecommerce/shops/1/carts/4/items/8", vec![], None, |ml| async move {
            ml.ecommerce_cart_item().find(id("1"), id("4"), id("8")).await
        })
        .await;
        check(
            "POST",
            "/ecommerce/shops/1/carts/4/items",
            vec![],
            Some(json!({ "ecommerce_product_id": "7" })),
            |ml| async move {
                ml.ecommerce_cart_item()
                    .create(id("1"), id("4"), json!({ "ecommerce_product_id": "7" }))
                    .await
            },
        )
        .await;
        check("PUT", "/ecommerce/shops/1/carts/4/items/8", vec![], Some(json!({ "quantity": 2 })), |ml| async move {
            ml.ecommerce_cart_item().update(id("1"), id("4"), id("8"), json!({ "quantity": 2 })).await
        })
        .await;
        check("DELETE", "/ecommerce/shops/1/carts/4/items/8", vec![], None, |ml| async move {
            ml.ecommerce_cart_item().delete(id("1"), id("4"), id("8")).await
        })
        .await;
    }

    #[tokio::test]
    async fn imports() {
        check("POST", "/ecommerce/shops/1/products/import", vec![], Some(json!([{ "name": "Hat" }])), |ml| async move {
            ml.ecommerce_import().products(id("1"), json!([{ "name": "Hat" }])).await
        })
        .await;
        check("POST", "/ecommerce/shops/1/categories/import", vec![], Some(json!([{ "name": "Hats" }])), |ml| async move {
            ml.ecommerce_import().categories(id("1"), json!([{ "name": "Hats" }])).await
        })
        .await;
        check(
            "POST",
            "/ecommerce/shops/1/customers/import",
            vec![],
            Some(json!([{ "email": "a@b.test" }])),
            |ml| async move { ml.ecommerce_import().customers(id("1"), json!([{ "email": "a@b.test" }])).await },
        )
        .await;
        let order = json!({ "customer": { "email": "a@b.test" }, "cart": { "items": [{ "ecommerce_product_id": "7" }] } });
        let orders = json!([order.clone()]);
        check(
            "POST",
            "/ecommerce/shops/1/orders/import",
            vec![],
            Some(json!({ "orders": [order] })),
            |ml| async move { ml.ecommerce_import().orders(id("1"), orders).await },
        )
        .await;
    }

    #[tokio::test]
    async fn revenue_reports() {
        check("GET", "/campaigns/2/reports/ecommerce", vec![("shop_id", "1")], None, |ml| async move {
            ml.campaign().ecommerce_stats(id("2"), id("1")).await
        })
        .await;
        check(
            "GET",
            "/campaigns/2/reports/ecommerce/orders",
            vec![("shop_id", "1"), ("page", "2")],
            None,
            |ml| async move {
                ml.campaign()
                    .ecommerce_orders(id("2"), id("1"), Parameter::new().add("page", "2"))
                    .await
            },
        )
        .await;
        check("GET", "/subscribers/6/reports/ecommerce", vec![("shop_id", "1")], None, |ml| async move {
            ml.subscriber().ecommerce_stats(id("6"), id("1")).await
        })
        .await;
        check("GET", "/subscribers/6/reports/ecommerce/orders", vec![("shop_id", "1")], None, |ml| async move {
            ml.subscriber().ecommerce_orders(id("6"), id("1"), Parameter::new()).await
        })
        .await;
        check("GET", "/automations/8/monthly-totals/ecommerce", vec![], None, |ml| async move {
            ml.automation().ecommerce_stats(id("8"), Parameter::new()).await
        })
        .await;
        check(
            "GET",
            "/automations/8/emails/9/ecommerce-overview-stats",
            vec![("shop_id", "1")],
            None,
            |ml| async move {
                ml.automation()
                    .email_ecommerce_stats(id("8"), id("9"), Parameter::new().add("shop_id", "1"))
                    .await
            },
        )
        .await;
        check(
            "GET",
            "/automations/8/order-activity",
            vec![("time_frame", "all_time")],
            None,
            |ml| async move {
                ml.automation()
                    .ecommerce_orders(id("8"), Parameter::new().add("time_frame", "all_time"))
                    .await
            },
        )
        .await;
    }
}
