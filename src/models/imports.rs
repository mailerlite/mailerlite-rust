//! Subscriber import receipts and progress reports.
use crate::models::{ImportId, SegmentId};
use serde::Deserialize;

/// Acceptance receipt; this does not mean the import has completed.
#[derive(Debug, Clone, Deserialize)]
pub struct ImportReceipt {
    /// URL of the documented status endpoint. The SDK validates it before polling.
    pub import_progress_url: String,
}
/// Progress of a subscriber import.
#[derive(Debug, Clone, Deserialize)]
pub struct ImportReport {
    /// Import identifier.
    pub id: ImportId,
    /// Account identifier, when supplied by the backend.
    pub account_id: Option<String>,
    /// Number of submitted subscribers.
    pub total: u64,
    /// Number processed so far.
    pub processed: u64,
    /// Number newly imported.
    pub imported: u64,
    /// Number updated.
    pub updated: u64,
    /// Number rejected.
    pub errored: u64,
    /// Percentage processed.
    pub percent: f64,
    /// Whether processing is complete.
    pub done: bool,
    /// Report update timestamp.
    pub updated_at: Option<String>,
    /// Completion timestamp.
    pub finished_at: Option<String>,
    /// Stop timestamp, if applicable.
    pub stopped_at: Option<String>,
    /// Completion timestamp of an undo operation.
    pub undone_at: Option<String>,
    /// Start timestamp of an undo operation.
    pub undo_started_at: Option<String>,
    /// Whether the free-account limit stopped the import.
    pub is_stopped_due_to_free_account_limit_exceeded: Option<bool>,
    /// Up to five invalid input samples; absent differs from an empty sample.
    pub invalid: Option<Vec<ImportLogSample>>,
    /// Total invalid inputs, which can exceed the number of samples.
    pub invalid_count: Option<u64>,
    /// Up to five addresses classified as mistyped.
    pub mistyped: Option<Vec<ImportLogSample>>,
    /// Total mistyped inputs.
    pub mistyped_count: Option<u64>,
    /// Up to five inputs that changed existing subscribers.
    pub changed: Option<Vec<ImportLogSample>>,
    /// Total changed inputs.
    pub changed_count: Option<u64>,
    /// Up to five unchanged inputs.
    pub unchanged: Option<Vec<ImportLogSample>>,
    /// Total unchanged inputs.
    pub unchanged_count: Option<u64>,
    /// Up to five unsubscribed input samples.
    pub unsubscribed: Option<Vec<ImportLogSample>>,
    /// Total unsubscribed inputs.
    pub unsubscribed_count: Option<u64>,
    /// Up to five role-based address samples.
    pub role_based: Option<Vec<ImportLogSample>>,
    /// Total role-based addresses.
    pub role_based_count: Option<u64>,
    /// Up to five suspicious-format address samples.
    pub suspicious_format: Option<Vec<ImportLogSample>>,
    /// Total suspicious-format inputs.
    pub suspicious_format_count: Option<u64>,
    /// Up to five duplicate input samples.
    pub duplicate: Option<Vec<ImportLogSample>>,
    /// Total duplicate inputs.
    pub duplicate_count: Option<u64>,
    /// Up to five inputs rejected by the subscriber limit.
    pub subscriber_limit: Option<Vec<ImportLogSample>>,
    /// Total inputs rejected by the subscriber limit.
    pub subscriber_limit_count: Option<u64>,
    /// Total addresses rejected by the import blocklist.
    pub banned_import_emails_count: Option<u64>,
    /// Uploaded file location, returned only for authorized internal requests.
    pub file_path: Option<String>,
    /// Column-matching route, returned only for authorized internal requests.
    pub match_route: Option<String>,
    /// Import source label, returned only for authorized internal requests.
    pub source_label: Option<String>,
    /// Whether undo also removes imported groups, when supplied.
    pub undo_will_remove_groups: Option<bool>,
    /// Number of groups an undo would remove, when supplied.
    pub undo_groups_count: Option<u64>,
    /// Number of column-created groups an undo would remove, when supplied.
    pub undo_column_groups_count: Option<u64>,
    /// Internal column-to-group assignment progress, when supplied.
    pub column_groups: Option<ImportColumnGroups>,
    /// Mailchimp migration details, present only for that import source.
    pub meta: Option<MailchimpImportMetadata>,
}

/// One sampled import log row, whose ID is distinct from a subscriber ID.
#[derive(Debug, Clone, Deserialize)]
pub struct ImportLogSample {
    /// Opaque import-log identifier, preserving large values and leading zeroes.
    pub id: String,
    /// Submitted address, or empty text when the input omitted it.
    pub email: String,
}

/// Progress of assigning imported column values to subscriber groups.
#[derive(Debug, Clone, Deserialize)]
pub struct ImportColumnGroups {
    /// Whether mapping awaits review.
    pub awaits_review: bool,
    /// Whether group assignment is currently running.
    pub assigning: bool,
    /// Percentage completed, or null when assignment is not running.
    pub progress: Option<u64>,
    /// Whether assignment failed.
    pub failed: bool,
    /// Mapping totals, when review has been applied.
    pub stats: Option<ImportColumnGroupStats>,
}

/// Totals recorded when an import's column-to-group mapping is applied.
#[derive(Debug, Clone, Deserialize)]
pub struct ImportColumnGroupStats {
    /// Number of groups created by the mapping.
    pub created: u64,
    /// Number of existing groups matched by the mapping.
    pub matched: u64,
    /// Total distinct groups assigned by the mapping.
    pub assigned: u64,
}

/// Additional details of a Mailchimp-source import.
#[derive(Debug, Clone, Deserialize)]
pub struct MailchimpImportMetadata {
    /// Requested segment-migration mode.
    pub mailchimp_segment_option: Option<String>,
    /// IDs of newly created segments; the backend stores numeric IDs here.
    #[serde(default, deserialize_with = "segment_ids")]
    pub mailchimp_created_segments: Vec<SegmentId>,
    /// Segment names skipped during migration and their reasons.
    #[serde(default)]
    pub mailchimp_skipped_segments: Vec<SkippedImportSegment>,
    /// Requested field-mapping mode.
    pub mailchimp_mapping_option: Option<String>,
    /// Whether mapping has been submitted.
    pub mapping_submitted: bool,
}

/// A segment that could not be recreated during migration.
#[derive(Debug, Clone, Deserialize)]
pub struct SkippedImportSegment {
    /// Source segment name.
    pub name: String,
    /// Backend reason, such as `duplicate` or `unmappable`.
    pub reason: String,
}

fn segment_ids<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<SegmentId>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StoredId {
        Text(String),
        Integer(u64),
    }
    Vec::<StoredId>::deserialize(d).map(|ids| {
        ids.into_iter()
            .map(|id| {
                SegmentId::new(match id {
                    StoredId::Text(value) => value,
                    StoredId::Integer(value) => value.to_string(),
                })
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    fn report(done: bool) -> Value {
        json!({"data":{"id":"123","total":1,"processed":1,"imported":1,"updated":0,"errored":0,"percent":100,"done":done}})
    }

    #[test]
    fn report_schema_distinguishes_missing_outcome_samples_from_empty_samples() {
        use crate::models::imports::ImportReport;
        let mut value = report(true)["data"].clone();
        let sparse: ImportReport = serde_json::from_value(value.clone()).unwrap();
        assert!(sparse.invalid.is_none());
        assert!(sparse.invalid_count.is_none());
        value["invalid"] = json!([]);
        value["invalid_count"] = json!(0);
        let complete: ImportReport = serde_json::from_value(value).unwrap();
        assert!(complete.invalid.unwrap().is_empty());
        assert_eq!(complete.invalid_count, Some(0));
    }

    #[test]
    fn report_schema_types_outcome_counts_without_confusing_samples_with_totals() {
        use crate::models::imports::ImportReport;
        let mut value = report(true)["data"].clone();
        value["account_id"] = json!("00007");
        value["invalid"] = json!([{"id":"900719925474099312345","email":"bad-address"}]);
        value["invalid_count"] = json!(100);
        value["mistyped"] = json!([]);
        value["mistyped_count"] = json!(2);
        value["changed"] = json!([]);
        value["changed_count"] = json!(3);
        value["unchanged"] = json!([]);
        value["unchanged_count"] = json!(4);
        value["unsubscribed"] = json!([]);
        value["unsubscribed_count"] = json!(5);
        value["role_based"] = json!([]);
        value["role_based_count"] = json!(6);
        value["suspicious_format"] = json!([]);
        value["suspicious_format_count"] = json!(7);
        value["duplicate"] = json!([]);
        value["duplicate_count"] = json!(8);
        value["subscriber_limit"] = json!([]);
        value["subscriber_limit_count"] = json!(9);
        value["banned_import_emails_count"] = json!(10);
        value["is_stopped_due_to_free_account_limit_exceeded"] = json!(false);
        let result: ImportReport = serde_json::from_value(value).unwrap();
        assert_eq!(result.account_id.as_deref(), Some("00007"));
        assert_eq!(result.invalid.unwrap()[0].id, "900719925474099312345");
        assert_eq!(result.invalid_count, Some(100));
        assert_eq!(result.mistyped_count, Some(2));
        assert_eq!(result.changed_count, Some(3));
        assert_eq!(result.unchanged_count, Some(4));
        assert_eq!(result.unsubscribed_count, Some(5));
        assert_eq!(result.role_based_count, Some(6));
        assert_eq!(result.suspicious_format_count, Some(7));
        assert_eq!(result.duplicate_count, Some(8));
        assert_eq!(result.subscriber_limit_count, Some(9));
        assert_eq!(result.banned_import_emails_count, Some(10));
        assert_eq!(
            result.is_stopped_due_to_free_account_limit_exceeded,
            Some(false)
        );
    }

    #[test]
    fn report_schema_types_conditional_migration_and_group_assignment_metadata() {
        use crate::models::imports::ImportReport;
        let mut value = report(true)["data"].clone();
        value["column_groups"] = json!({"awaits_review":false,"assigning":true,"progress":42,"failed":false,"stats":{"created":2,"matched":3,"assigned":5}});
        value["meta"] = json!({"mailchimp_segment_option":"all","mailchimp_created_segments":[9007199254740993u64,"0002"],"mailchimp_skipped_segments":[{"name":"Existing","reason":"duplicate"}],"mailchimp_mapping_option":"auto","mapping_submitted":true});
        let result: ImportReport = serde_json::from_value(value).unwrap();
        let groups = result.column_groups.unwrap();
        assert_eq!(groups.progress, Some(42));
        let stats = groups.stats.unwrap();
        assert_eq!((stats.created, stats.matched, stats.assigned), (2, 3, 5));
        let migration = result.meta.unwrap();
        assert_eq!(
            migration.mailchimp_created_segments[0].as_str(),
            "9007199254740993"
        );
        assert_eq!(migration.mailchimp_created_segments[1].as_str(), "0002");
        assert_eq!(migration.mailchimp_skipped_segments[0].reason, "duplicate");
    }

    #[test]
    fn report_schema_rejects_fractional_migration_segment_identifiers() {
        use crate::models::imports::ImportReport;
        let mut value = report(true)["data"].clone();
        value["meta"] = json!({"mailchimp_created_segments":[1.5],"mapping_submitted":false});
        assert!(serde_json::from_value::<ImportReport>(value).is_err());
    }
}
