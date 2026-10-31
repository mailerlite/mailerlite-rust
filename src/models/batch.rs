//! Ordered batch results with independently inspectable failures.
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use std::fmt;

/// Result of a batch request; outer success does not imply item success.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchResult {
    /// Number of submitted operations.
    pub total: u64,
    /// Successful operations.
    pub successful: u64,
    /// Failed operations.
    pub failed: u64,
    /// Results in submission order.
    pub responses: Vec<BatchItemResult>,
}
/// One batch operation's HTTP status and endpoint-specific JSON body.
#[derive(Clone, Deserialize)]
pub struct BatchItemResult {
    /// HTTP status of this operation.
    pub code: u16,
    /// Endpoint-specific body, including validation errors for failed items.
    pub body: Value,
}
impl BatchItemResult {
    /// Whether this individual operation succeeded.
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.code)
    }
    /// Decode this item's body without assuming a common envelope.
    ///
    /// The body remains available after either success or failure. Decoding
    /// borrows the JSON tree rather than cloning it before deserialization.
    /// Check [`Self::is_success`] separately: a failed operation's error body can
    /// also deserialize successfully. This method never sends an HTTP request.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::{
    ///     models::{batch::BatchItemResult, groups::Group},
    ///     pagination::NumberedPage,
    /// };
    ///
    /// # fn main() -> Result<(), serde_json::Error> {
    /// let item = BatchItemResult {
    ///     code: 200,
    ///     body: serde_json::json!({"data": [{"id": "123", "name": "Newsletter"}]}),
    /// };
    ///
    /// if item.is_success() {
    ///     let groups: NumberedPage<Group> = item.decode()?;
    ///
    ///     println!("{} groups", groups.data.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`serde_json::Error`] when the item's body does not match `T`,
    /// including missing fields or incompatible known field types. Choose the
    /// endpoint's actual envelope rather than assuming every body contains
    /// `data`. The original body and HTTP status remain available on this item.
    pub fn decode<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        T::deserialize(&self.body)
    }
}
impl fmt::Debug for BatchItemResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BatchItemResult")
            .field("code", &self.code)
            .field("body", &"[OMITTED]")
            .finish()
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use serde_json::json;

    #[test]
    fn item_decodes_a_typed_numbered_page_without_consuming_its_body() {
        use crate::{
            models::{batch::BatchItemResult, groups::Group},
            pagination::NumberedPage,
        };
        let body = json!({"data":[{"id":"00042","name":"Readers"}],"meta":{"current_page":1,"last_page":2}});
        let item = BatchItemResult {
            code: 200,
            body: body.clone(),
        };
        let page: NumberedPage<Group> = item.decode().unwrap();
        assert_eq!(page.data[0].id.as_str(), "00042");
        assert_eq!(page.next_page(), Some(2));
        assert_eq!(item.body, body);
    }

    #[test]
    fn item_decode_failure_retains_the_original_body_for_another_schema() {
        use crate::{
            models::{batch::BatchItemResult, groups::Group},
            pagination::NumberedPage,
            response::Count,
        };
        let item = BatchItemResult {
            code: 200,
            body: json!({"total":7}),
        };
        assert!(item.decode::<NumberedPage<Group>>().is_err());
        assert_eq!(item.decode::<Count>().unwrap().total, 7);
        assert_eq!(item.body, json!({"total":7}));
    }

    #[test]
    fn failed_item_can_decode_its_own_validation_schema() {
        use crate::models::batch::BatchItemResult;
        #[derive(serde::Deserialize)]
        struct Validation {
            message: String,
            errors: std::collections::BTreeMap<String, Vec<String>>,
        }
        let item = BatchItemResult {
            code: 422,
            body: json!({"message":"Invalid input","errors":{"email":["Not a valid email"]}}),
        };
        assert!(!item.is_success());
        let error: Validation = item.decode().unwrap();
        assert_eq!(error.message, "Invalid input");
        assert_eq!(error.errors["email"], ["Not a valid email"]);
    }

    #[test]
    fn empty_item_body_decodes_only_to_a_compatible_schema() {
        use crate::models::batch::BatchItemResult;
        let item = BatchItemResult {
            code: 204,
            body: json!(null),
        };
        assert!(item.is_success());
        item.decode::<()>().unwrap();
        assert!(item.decode::<String>().is_err());
    }
}
