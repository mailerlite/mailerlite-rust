//! Numbered pages, aggregation counts, and continuation metadata.
use super::PageLinks;
use serde::Deserialize;
use std::collections::BTreeMap;

/// Metadata for an endpoint using numbered pages.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct NumberedMeta {
    /// Current page number, starting at one.
    pub current_page: Option<u64>,
    /// Last available page number.
    pub last_page: Option<u64>,
    /// Requested page size, when reported.
    pub per_page: Option<u64>,
    /// Total matching resources, when reported.
    pub total: Option<u64>,
    /// Index of the first resource on this page.
    pub from: Option<u64>,
    /// Index of the last resource on this page.
    pub to: Option<u64>,
    /// Counts keyed by resource type or status, such as `embedded` or `draft`.
    ///
    /// Keys remain open for new server categories. Integer counts and digit text
    /// from database aggregates are decoded exactly; arbitrary JSON is rejected.
    #[serde(default, deserialize_with = "aggregation_counts")]
    pub aggregations: Option<BTreeMap<String, u64>>,
}

fn aggregation_counts<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<BTreeMap<String, u64>>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Count {
        Integer(u64),
        Text(String),
    }
    Option::<BTreeMap<String, Count>>::deserialize(deserializer)?
        .map(|counts| {
            counts
                .into_iter()
                .map(|(name, count)| {
                    let value = match count {
                        Count::Integer(value) => value,
                        Count::Text(value)
                            if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) =>
                        {
                            value.parse().map_err(serde::de::Error::custom)?
                        }
                        Count::Text(_) => {
                            return Err(serde::de::Error::custom(
                                "aggregation counts must be nonnegative integers",
                            ));
                        }
                    };
                    Ok((name, value))
                })
                .collect()
        })
        .transpose()
}

/// A page from an endpoint that uses page numbers.
#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct NumberedPage<T> {
    /// Resources in this page.
    pub data: Vec<T>,
    /// Navigation links, retained but never fetched directly.
    #[serde(default)]
    pub links: PageLinks,
    /// Numbered metadata. Missing metadata means no known continuation.
    #[serde(default)]
    pub meta: NumberedMeta,
}

impl<T> NumberedPage<T> {
    /// Return the next page only when metadata establishes one exists.
    pub fn next_page(&self) -> Option<u64> {
        let current = self.meta.current_page?;
        let last = self.meta.last_page?;
        if current > 0 && current < last {
            current.checked_add(1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregation_counts_preserve_exact_large_counts_and_future_categories() {
        let page: NumberedPage<u32> = serde_json::from_str(
            r#"{"data":[],"meta":{"aggregations":{"embedded":3,"future":"9007199254740993"}}}"#,
        )
        .unwrap();
        let counts = page.meta.aggregations.unwrap();
        assert_eq!(counts["embedded"], 3);
        assert_eq!(counts["future"], 9_007_199_254_740_993);
    }

    #[test]
    fn null_aggregations_remain_absent() {
        let meta: NumberedMeta = serde_json::from_str(r#"{"aggregations":null}"#).unwrap();
        assert!(meta.aggregations.is_none());
    }

    #[test]
    fn aggregation_rejects_nested_objects() {
        assert!(
            serde_json::from_str::<NumberedMeta>(r#"{"aggregations":{"draft":{"count":1}}}"#)
                .is_err()
        );
    }

    #[test]
    fn aggregation_rejects_negative_counts() {
        assert!(serde_json::from_str::<NumberedMeta>(r#"{"aggregations":{"draft":-1}}"#).is_err());
    }

    #[test]
    fn aggregation_rejects_fractional_counts() {
        assert!(serde_json::from_str::<NumberedMeta>(r#"{"aggregations":{"draft":1.5}}"#).is_err());
    }

    #[test]
    fn aggregation_rejects_non_digit_text() {
        assert!(
            serde_json::from_str::<NumberedMeta>(r#"{"aggregations":{"draft":"unknown"}}"#)
                .is_err()
        );
    }

    #[test]
    fn aggregation_rejects_overflowing_text() {
        assert!(
            serde_json::from_str::<NumberedMeta>(
                r#"{"aggregations":{"draft":"18446744073709551616"}}"#
            )
            .is_err()
        );
    }

    #[test]
    fn missing_metadata_does_not_guess_a_continuation() {
        let page: NumberedPage<u32> = serde_json::from_str(
            r#"{"data":[1],"links":{"next":"https://unexpected.example/?page=2"}}"#,
        )
        .unwrap();
        assert_eq!(page.next_page(), None);
    }

    #[test]
    fn numbered_continuations_need_complete_valid_metadata() {
        for (current, last, expected) in [
            (1, 3, Some(2)),
            (3, 3, None),
            (0, 3, None),
            (4, 3, None),
            (u64::MAX, u64::MAX, None),
        ] {
            let page = NumberedPage::<u32> {
                data: vec![],
                links: PageLinks::default(),
                meta: NumberedMeta {
                    current_page: Some(current),
                    last_page: Some(last),
                    ..Default::default()
                },
            };
            assert_eq!(page.next_page(), expected);
        }
    }
}
