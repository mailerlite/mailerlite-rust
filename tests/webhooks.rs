//! Token-free incoming webhook schema and signature verification scenarios.
#![allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
use mailerlite_rs::webhooks::{WebhookEvent, WebhookPayload};
use serde_json::{Value, json};

fn subscriber() -> Value {
    json!({
        "id":"000900719925474099312345", "email":"reader@example.test", "status":"active",
        "source":"api", "sent":5, "opens_count":1, "clicks_count":0,
        "open_rate":20.0, "click_rate":0, "ip_address":null,
        "subscribed_at":"2026-01-01T00:00:00Z", "unsubscribed_at":null,
        "created_at":"2026-01-01T00:00:00Z", "updated_at":"2026-01-02T00:00:00Z",
        "deleted_at":null, "forget_at":null, "fields":{"name":"Reader","score":7,"city":null},
        "opted_in_at":null, "optin_ip":null
    })
}
fn campaign() -> Value {
    json!({"id":"00123","name":"Campaign","total_recipients":20,"preview_url":"https://example.test/preview","date":"2026-01-02T00:00:00Z"})
}
fn flat_subscriber(name: &str) -> Value {
    let mut payload = subscriber();
    payload["event"] = json!(name);
    payload["account_id"] = json!(9007199254740993_u64);
    payload
}
fn nested_subscriber(name: &str) -> Value {
    json!({"type":name,"subscriber":subscriber(),"account_id":42})
}
fn group_event(name: &str) -> Value {
    let mut payload = nested_subscriber(name);
    payload["group"] = json!({"id":"0007","name":"Readers"});
    payload
}
fn automation_event(name: &str) -> Value {
    let mut payload = nested_subscriber(name);
    payload["automation"] = json!({"id":"0008","name":"Welcome"});
    payload
}
fn campaign_event(name: &str) -> Value {
    let mut payload = nested_subscriber(name);
    payload["campaign"] = campaign();
    payload
}
fn event(value: Value) -> WebhookEvent {
    serde_json::from_value(value).unwrap()
}

#[test]
fn created_event_preserves_identifier_scalar_fields_and_optional_location() {
    let mut value = flat_subscriber("subscriber.created");
    value["location"] = json!({"id":"009","title":"Dhaka","country_name":"Bangladesh","continent_name":"Asia","state_name":null,"time_zone":"Asia/Dhaka"});
    let WebhookEvent::SubscriberCreated(payload) = event(value) else {
        panic!("created event")
    };
    assert_eq!(payload.subscriber.id.as_str(), "000900719925474099312345");
    assert_eq!(payload.account_id, Some(9007199254740993));
    assert_eq!(
        payload.subscriber.location.unwrap().time_zone.as_deref(),
        Some("Asia/Dhaka")
    );
    assert_eq!(payload.subscriber.fields["score"], 7_i64.into());
    assert_eq!(
        payload.subscriber.fields["city"],
        mailerlite_rs::models::subscribers::FieldValue::Null
    );
}
#[test]
fn updated_event_uses_flat_subscriber_schema() {
    assert!(matches!(
        event(flat_subscriber("subscriber.updated")),
        WebhookEvent::SubscriberUpdated(_)
    ));
}
#[test]
fn unsubscribed_event_uses_flat_subscriber_schema() {
    assert!(matches!(
        event(flat_subscriber("subscriber.unsubscribed")),
        WebhookEvent::SubscriberUnsubscribed(_)
    ));
}
#[test]
fn active_event_uses_flat_subscriber_schema() {
    assert!(matches!(
        event(flat_subscriber("subscriber.active")),
        WebhookEvent::SubscriberActive(_)
    ));
}
#[test]
fn bounced_event_uses_flat_subscriber_schema() {
    assert!(matches!(
        event(flat_subscriber("subscriber.bounced")),
        WebhookEvent::SubscriberBounced(_)
    ));
}
#[test]
fn spam_event_uses_flat_subscriber_schema() {
    assert!(matches!(
        event(flat_subscriber("subscriber.spam_reported")),
        WebhookEvent::SubscriberSpamReported(_)
    ));
}
#[test]
fn deleted_event_uses_nested_subscriber_schema() {
    assert!(matches!(
        event(nested_subscriber("subscriber.deleted")),
        WebhookEvent::SubscriberDeleted(_)
    ));
}
#[test]
fn added_to_group_event_preserves_typed_group_identity() {
    let WebhookEvent::SubscriberAddedToGroup(payload) =
        event(group_event("subscriber.added_to_group"))
    else {
        panic!("group event")
    };
    assert_eq!(payload.group.id.as_str(), "0007");
    assert_eq!(payload.group.name, "Readers");
}
#[test]
fn removed_from_group_event_uses_nested_group_schema() {
    assert!(matches!(
        event(group_event("subscriber.removed_from_group")),
        WebhookEvent::SubscriberRemovedFromGroup(_)
    ));
}
#[test]
fn triggered_automation_event_preserves_typed_automation_identity() {
    let WebhookEvent::SubscriberAutomationTriggered(payload) =
        event(automation_event("subscriber.automation_triggered"))
    else {
        panic!("automation event")
    };
    assert_eq!(payload.automation.id.as_str(), "0008");
    assert_eq!(payload.automation.name, "Welcome");
}
#[test]
fn completed_automation_event_uses_nested_automation_schema() {
    assert!(matches!(
        event(automation_event("subscriber.automation_completed")),
        WebhookEvent::SubscriberAutomationCompleted(_)
    ));
}
#[test]
fn sent_campaign_event_uses_flat_campaign_schema() {
    let mut value = campaign();
    value["event"] = json!("campaign.sent");
    let WebhookEvent::CampaignSent(payload) = event(value) else {
        panic!("campaign event")
    };
    assert_eq!(payload.campaign.id.as_str(), "00123");
    assert_eq!(payload.campaign.total_recipients, 20);
    assert_eq!(payload.account_id, None);
}
#[test]
fn opened_campaign_event_uses_nested_campaign_schema() {
    assert!(matches!(
        event(campaign_event("campaign.open")),
        WebhookEvent::CampaignOpen(_)
    ));
}
#[test]
fn clicked_campaign_event_preserves_link_destination() {
    let mut value = campaign_event("campaign.click");
    value["link_url"] = json!("https://example.test/welcome");
    let WebhookEvent::CampaignClick(payload) = event(value) else {
        panic!("click event")
    };
    assert_eq!(payload.link_url, "https://example.test/welcome");
}
#[test]
fn single_delivery_decodes_without_a_batch_wrapper() {
    let payload: WebhookPayload =
        serde_json::from_value(flat_subscriber("subscriber.created")).unwrap();
    assert!(matches!(
        payload,
        WebhookPayload::Single(WebhookEvent::SubscriberCreated(_))
    ));
}
#[test]
fn batch_preserves_event_order_and_optional_account_identifiers() {
    let mut open = campaign_event("campaign.open");
    open.as_object_mut().unwrap().remove("account_id");
    let payload: WebhookPayload = serde_json::from_value(
        json!({"events":[open,flat_subscriber("subscriber.updated")],"total":2}),
    )
    .unwrap();
    let WebhookPayload::Batch(batch) = payload else {
        panic!("batch")
    };
    assert_eq!(batch.total, 2);
    assert_eq!(batch.events[0].name(), "campaign.open");
    assert_eq!(batch.events[1].name(), "subscriber.updated");
    let WebhookEvent::CampaignOpen(open) = &batch.events[0] else {
        panic!("open")
    };
    assert_eq!(open.account_id, None);
}
#[test]
fn unknown_event_is_explicit_and_its_raw_data_is_redacted() {
    let payload = event(json!({"event":"future.event","secret":"private-marker"}));
    assert_eq!(payload.name(), "future.event");
    assert!(!format!("{payload:?}").contains("private-marker"));
    let WebhookEvent::Unknown(unknown) = payload else {
        panic!("unknown")
    };
    assert_eq!(unknown.payload()["secret"], "private-marker");
    assert!(!format!("{unknown:?}").contains("private-marker"));
}
#[test]
fn backend_only_form_submitted_event_remains_explicitly_unknown() {
    assert!(matches!(
        event(flat_subscriber("subscriber.form_submitted")),
        WebhookEvent::Unknown(_)
    ));
}
#[test]
fn malformed_known_subscriber_event_does_not_become_unknown() {
    let mut value = flat_subscriber("subscriber.created");
    value["email"] = json!(["invalid"]);
    assert!(serde_json::from_value::<WebhookEvent>(value).is_err());
}
#[test]
fn missing_known_group_resource_is_a_decode_error() {
    assert!(
        serde_json::from_value::<WebhookEvent>(nested_subscriber("subscriber.added_to_group"))
            .is_err()
    );
}
#[test]
fn missing_click_link_is_a_decode_error() {
    assert!(serde_json::from_value::<WebhookEvent>(campaign_event("campaign.click")).is_err());
}
#[test]
fn malformed_known_event_inside_batch_rejects_the_batch() {
    assert!(
        serde_json::from_value::<WebhookPayload>(
            json!({"events":[{"event":"subscriber.created"}],"total":1})
        )
        .is_err()
    );
}
#[test]
fn wrong_discriminator_for_known_event_is_rejected() {
    let mut value = subscriber();
    value["type"] = json!("subscriber.created");
    assert!(serde_json::from_value::<WebhookEvent>(value).is_err());
}
#[test]
fn conflicting_discriminators_are_rejected() {
    let mut value = flat_subscriber("subscriber.created");
    value["type"] = json!("future.event");
    assert!(serde_json::from_value::<WebhookEvent>(value).is_err());
}
#[test]
fn numeric_discriminator_is_rejected() {
    assert!(serde_json::from_value::<WebhookEvent>(json!({"event":1})).is_err());
}
#[test]
fn missing_discriminator_is_rejected() {
    assert!(serde_json::from_value::<WebhookEvent>(subscriber()).is_err());
}
#[test]
fn array_root_is_rejected() {
    assert!(serde_json::from_value::<WebhookPayload>(json!([])).is_err());
}
#[test]
fn ambiguous_single_and_batch_envelopes_are_rejected() {
    assert!(
        serde_json::from_value::<WebhookPayload>(
            json!({"event":"future.event","events":[],"total":0})
        )
        .is_err()
    );
}
#[test]
fn batch_requires_an_array_of_events() {
    assert!(serde_json::from_value::<WebhookPayload>(json!({"events":{},"total":0})).is_err());
}
#[test]
fn batch_requires_a_numeric_total() {
    assert!(serde_json::from_value::<WebhookPayload>(json!({"events":[],"total":"zero"})).is_err());
}
#[test]
fn account_identifier_rejects_negative_numbers() {
    let mut value = flat_subscriber("subscriber.created");
    value["account_id"] = json!(-1);
    assert!(serde_json::from_value::<WebhookEvent>(value).is_err());
}
#[test]
fn known_payload_debug_does_not_expose_custom_fields() {
    let mut value = flat_subscriber("subscriber.created");
    value["fields"]["name"] = json!("private-marker");
    let payload = event(value);
    assert!(!format!("{payload:?}").contains("private-marker"));
    let WebhookEvent::SubscriberCreated(created) = payload else {
        panic!("created")
    };
    assert!(!format!("{created:?} {:?}", created.subscriber).contains("private-marker"));
}

#[cfg(feature = "webhook-signatures")]
mod signatures {
    use mailerlite_rs::webhooks::{SignatureError, WebhookPayload, verify_signature};
    const SECRET: &str = "test-webhook-secret";
    const BODY: &[u8] = br#"{"event":"future.event","note":"raw bytes"}"#;
    // Generated independently with Python stdlib hmac/hashlib, not this Rust implementation.
    const TAG: &str = "1b7c95ba98c90ff8e90d9206c689dfa17bb0c42b7b9d542a3a2a8fa58bad53c6";

    #[test]
    fn verifies_rfc4231_case_two_known_answer() {
        // https://www.rfc-editor.org/rfc/rfc4231.html#section-4.3
        assert_eq!(
            verify_signature(
                "Jefe",
                b"what do ya want for nothing?",
                "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
            ),
            Ok(())
        );
    }
    #[test]
    fn verifies_independently_generated_json_signature() {
        assert_eq!(verify_signature(SECRET, BODY, TAG), Ok(()));
        assert!(serde_json::from_slice::<WebhookPayload>(BODY).is_ok());
    }
    #[test]
    fn accepts_uppercase_hexadecimal_signature() {
        assert_eq!(verify_signature(SECRET, BODY, &TAG.to_uppercase()), Ok(()));
    }
    #[test]
    fn changing_raw_whitespace_invalidates_signature() {
        let mut modified = BODY.to_vec();
        modified.push(b'\n');
        assert_eq!(
            verify_signature(SECRET, &modified, TAG),
            Err(SignatureError::Mismatch)
        );
    }
    #[test]
    fn wrong_secret_is_rejected_without_echoing_it() {
        let error = verify_signature("private-marker", BODY, TAG).unwrap_err();
        assert_eq!(error, SignatureError::Mismatch);
        assert!(!format!("{error:?} {error}").contains("private-marker"));
    }
    #[test]
    fn empty_secret_is_rejected() {
        assert_eq!(
            verify_signature("", BODY, TAG),
            Err(SignatureError::EmptySecret)
        );
    }
    #[test]
    fn non_hexadecimal_signature_is_rejected() {
        assert_eq!(
            verify_signature(SECRET, BODY, &"z".repeat(64)),
            Err(SignatureError::InvalidEncoding)
        );
    }
    #[test]
    fn truncated_signature_is_rejected() {
        assert_eq!(
            verify_signature(SECRET, BODY, &TAG[..32]),
            Err(SignatureError::InvalidEncoding)
        );
    }
    #[test]
    fn signature_prefix_is_rejected() {
        assert_eq!(
            verify_signature(SECRET, BODY, &format!("sha256={TAG}")),
            Err(SignatureError::InvalidEncoding)
        );
    }
    #[test]
    fn header_whitespace_is_rejected() {
        assert_eq!(
            verify_signature(SECRET, BODY, &format!(" {TAG}")),
            Err(SignatureError::InvalidEncoding)
        );
    }
    #[test]
    fn correct_signature_does_not_claim_json_validity() {
        let body = b"not JSON";
        let tag = "75829e412e2a289b8289c9602203e710c95d89d9a17163616c249e8cffd09caf";
        assert_eq!(verify_signature(SECRET, body, tag), Ok(()));
        assert!(serde_json::from_slice::<WebhookPayload>(body).is_err());
    }
}

#[test]
fn sent_campaign_without_a_remaining_email_accepts_null_preview() {
    let mut value = campaign();
    value["event"] = json!("campaign.sent");
    value["preview_url"] = Value::Null;
    let WebhookEvent::CampaignSent(payload) = event(value) else {
        panic!("expected typed campaign sent event")
    };
    assert_eq!(payload.campaign.id.as_str(), "00123");
    assert_eq!(payload.campaign.preview_url, None);
}

#[test]
fn opened_campaign_accepts_null_preview_without_losing_its_typed_event() {
    let mut value = campaign_event("campaign.open");
    value["campaign"]["preview_url"] = Value::Null;
    let WebhookEvent::CampaignOpen(payload) = event(value) else {
        panic!("expected typed campaign open event")
    };
    assert_eq!(payload.campaign.preview_url, None);
    assert_eq!(payload.subscriber.id.as_str(), "000900719925474099312345");
}

#[test]
fn clicked_campaign_accepts_null_preview_and_preserves_the_clicked_url() {
    let mut value = campaign_event("campaign.click");
    value["campaign"]["preview_url"] = Value::Null;
    value["link_url"] = json!("https://example.test/selected-link");
    let WebhookEvent::CampaignClick(payload) = event(value) else {
        panic!("expected typed campaign click event")
    };
    assert_eq!(payload.campaign.preview_url, None);
    assert_eq!(payload.link_url, "https://example.test/selected-link");
}
