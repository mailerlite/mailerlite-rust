//! Categories: <https://developers.mailerlite.com/ecommerce/categories> and
//! <https://developers.mailerlite.com/ecommerce/categories-products>

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::{Product, Scope};
use crate::{
    Page, Result,
    types::{Id, Sort, Timestamp, string_enum},
};

/// A product category.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Category {
    /// Category id.
    pub id: Id,
    /// Your own id for the category, if any.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// Category name.
    pub name: String,
    /// If `true`, products in this category never trigger automations.
    #[serde(default)]
    pub exclude_from_automations: bool,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
}

string_enum! {
    /// Sort field for [`ListCategories`].
    pub enum CategorySort {
        /// Id (API default).
        Id => "id",
        /// Name.
        Name => "name",
        /// Creation time.
        CreatedAt => "created_at",
    }
}

/// Query for [`Categories::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListCategories {
    /// Page size (API default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<CategorySort>>,
}

impl ListCategories {
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

    /// Sort order, e.g. `Sort::desc(CategorySort::CreatedAt)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<CategorySort>) -> Self {
        self.sort = Some(sort);
        self
    }
}

/// Query for [`Categories::products`] (products come back in ascending id order).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListCategoryProducts {
    /// Page size (API default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ListCategoryProducts {
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

/// Body for [`Categories::create`], [`Categories::update`] and one row of
/// [`Imports::categories`](super::Imports::categories).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CreateCategory {
    /// Category name.
    pub name: String,
    /// If `true`, products in this category never trigger automations. API default `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_automations: Option<bool>,
    /// Your own id for the category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl CreateCategory {
    /// A category called `name`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            exclude_from_automations: None,
            resource_id: None,
        }
    }

    /// Keep products in this category from triggering automations.
    #[must_use]
    pub fn exclude_from_automations(mut self, exclude: bool) -> Self {
        self.exclude_from_automations = Some(exclude);
        self
    }

    /// Your own id for the category.
    #[must_use]
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }
}

/// Category endpoints of a shop. Obtained with
/// [`ShopScope::categories`](super::ShopScope::categories).
#[derive(Clone, Copy, Debug)]
pub struct Categories<'a> {
    pub(super) scope: Scope<'a>,
}

impl<'a> Categories<'a> {
    /// `GET /ecommerce/shops/{shop_id}/categories`: one page of categories.
    pub async fn list(&self, query: &ListCategories) -> Result<Page<Category>> {
        self.scope
            .call(Method::GET, &["categories"])
            .query(query)
            .send()
            .await
    }

    /// Every category matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListCategories,
    ) -> impl futures_util::Stream<Item = Result<Category>> + 'a {
        let categories = *self;
        crate::pagination::stream::pages(query.page.unwrap_or(1), move |page| {
            let query = ListCategories {
                page: Some(page),
                ..query.clone()
            };
            async move { categories.list(&query).await }
        })
    }

    /// `GET /ecommerce/shops/{shop_id}/categories?limit=0`: number of categories.
    pub async fn count(&self) -> Result<u64> {
        self.scope.count(&["categories"]).await
    }

    /// `GET /ecommerce/shops/{shop_id}/categories/{id}`: one category.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Category> {
        self.scope
            .call(Method::GET, &["categories", id.as_ref()])
            .data()
            .await
    }

    /// `POST /ecommerce/shops/{shop_id}/categories`: creates a category.
    pub async fn create(&self, category: &CreateCategory) -> Result<Category> {
        self.scope
            .call(Method::POST, &["categories"])
            .json(category)
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/categories/{id}`: updates a category.
    pub async fn update(&self, id: impl AsRef<str>, category: &CreateCategory) -> Result<Category> {
        self.scope
            .call(Method::PUT, &["categories", id.as_ref()])
            .json(category)
            .data()
            .await
    }

    /// `DELETE /ecommerce/shops/{shop_id}/categories/{id}`: deletes a category and returns
    /// it.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Category> {
        // The API returns the bare category here, without the `data` envelope; `data()`
        // accepts both.
        self.scope
            .call(Method::DELETE, &["categories", id.as_ref()])
            .data()
            .await
    }

    /// `GET /ecommerce/shops/{shop_id}/categories/{id}/products`: one page of the category's
    /// products.
    pub async fn products(
        &self,
        id: impl AsRef<str>,
        query: &ListCategoryProducts,
    ) -> Result<Page<Product>> {
        self.scope
            .call(Method::GET, &["categories", id.as_ref(), "products"])
            .query(query)
            .send()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/categories/{id}/products/{product_id}`: adds a
    /// product to the category and returns the product.
    pub async fn assign_product(
        &self,
        id: impl AsRef<str>,
        product_id: impl AsRef<str>,
    ) -> Result<Product> {
        self.scope
            .call(
                Method::PUT,
                &["categories", id.as_ref(), "products", product_id.as_ref()],
            )
            .data()
            .await
    }

    /// `DELETE /ecommerce/shops/{shop_id}/categories/{id}/products/{product_id}`: removes a
    /// product from the category and returns the product.
    pub async fn unassign_product(
        &self,
        id: impl AsRef<str>,
        product_id: impl AsRef<str>,
    ) -> Result<Product> {
        // Bare product, no `data` envelope.
        self.scope
            .call(
                Method::DELETE,
                &["categories", id.as_ref(), "products", product_id.as_ref()],
            )
            .data()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::json;

    use super::*;
    use crate::http::mock::{client, fixture};

    #[tokio::test]
    async fn list_and_count() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_categories_list"))
            .respond(200, json!({"total": 123}));
        let shop = client.ecommerce().shop("9");

        let page = shop
            .categories()
            .list(
                &ListCategories::new()
                    .limit(5)
                    .page(2)
                    .sort(Sort::desc(CategorySort::CreatedAt)),
            )
            .await
            .unwrap();
        let total = shop.categories().count().await.unwrap();

        assert_eq!(page.data[0].id, "1234566457534276");
        assert_eq!(page.data[0].name, "Category 1");
        assert_eq!(page.data[0].resource_id, None);
        assert_eq!(total, 123);
        let requests = mock.requests();
        requests[0].assert(
            Method::GET,
            "ecommerce/shops/9/categories?limit=5&page=2&sort=-created_at",
        );
        requests[1].assert(Method::GET, "ecommerce/shops/9/categories?limit=0");
    }

    #[tokio::test]
    async fn get_create_update_delete() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_category"))
            .respond(201, fixture("ecommerce_category"))
            .respond(200, fixture("ecommerce_category"))
            .respond(200, fixture("ecommerce_category_deleted"));
        let shop = client.ecommerce().shop("9");
        let categories = shop.categories();

        let fetched = categories.get("1").await.unwrap();
        categories
            .create(&CreateCategory::new("Category 1"))
            .await
            .unwrap();
        categories
            .update(
                "1",
                &CreateCategory::new("Category 1")
                    .exclude_from_automations(true)
                    .resource_id("cat-1"),
            )
            .await
            .unwrap();
        let deleted = categories.delete("1").await.unwrap();

        assert_eq!(fetched, deleted);
        assert_eq!(deleted.created_at.unwrap().as_str(), "2021-09-01 14:03:50");
        let requests = mock.requests();
        requests[0]
            .assert(Method::GET, "ecommerce/shops/9/categories/1")
            .no_body();
        requests[1]
            .assert(Method::POST, "ecommerce/shops/9/categories")
            .body(&json!({"name": "Category 1"}));
        requests[2]
            .assert(Method::PUT, "ecommerce/shops/9/categories/1")
            .body(&json!({
                "name": "Category 1",
                "exclude_from_automations": true,
                "resource_id": "cat-1"
            }));
        requests[3]
            .assert(Method::DELETE, "ecommerce/shops/9/categories/1")
            .no_body();
    }

    #[tokio::test]
    async fn products_assign_unassign() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_category_products"))
            .respond(200, fixture("ecommerce_product"))
            .respond(200, fixture("ecommerce_product_unassigned"));
        let shop = client.ecommerce().shop("9");
        let categories = shop.categories();

        let page = categories
            .products("3", &ListCategoryProducts::new().limit(10).page(1))
            .await
            .unwrap();
        let assigned = categories.assign_product("3", "4").await.unwrap();
        let unassigned = categories.unassign_product("3", "4").await.unwrap();

        assert_eq!(page.data[0].categories.len(), 2);
        assert_eq!(assigned.id, "31986843543567834");
        assert_eq!(unassigned.id, "31986843543567834");
        assert!((unassigned.price - 123.4).abs() < f64::EPSILON);
        let requests = mock.requests();
        requests[0].assert(
            Method::GET,
            "ecommerce/shops/9/categories/3/products?limit=10&page=1",
        );
        requests[1]
            .assert(Method::PUT, "ecommerce/shops/9/categories/3/products/4")
            .no_body();
        requests[2]
            .assert(Method::DELETE, "ecommerce/shops/9/categories/3/products/4")
            .no_body();
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn list_all_walks_pages() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let category = fixture("ecommerce_category")["data"].clone();
        mock.respond(
            200,
            json!({"data": [category], "meta": {"current_page": 1, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [category], "meta": {"current_page": 2, "last_page": 2}}),
        );

        let shop = client.ecommerce().shop("9").with_resource_id();
        let all: Vec<Category> = shop
            .categories()
            .list_all(ListCategories::new().limit(1))
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(
            Method::GET,
            "ecommerce/shops/9/categories?with_resource_id=&limit=1&page=1",
        );
        requests[1].assert(
            Method::GET,
            "ecommerce/shops/9/categories?with_resource_id=&limit=1&page=2",
        );
    }
}
