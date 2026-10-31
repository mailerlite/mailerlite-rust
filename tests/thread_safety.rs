//! Compile-time concurrency contracts. Futures and streams are never polled.
#![allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]

use mailerlite_rs::{
    ApiError, DecodeError, Error, MailerLite, MailerLiteBuilder, ResponseMetadata,
    models::imports::ImportReceipt,
    resources::{
        automation_actions::UpdateAutomation,
        automation_steps::{CreateStep, StepType},
        automation_triggers::CreateAutomationTrigger,
        automations::CreateAutomation,
        batch::{BatchMethod, BatchRequest},
        campaigns::{CreateCampaign, Email},
        ecommerce::params::SaveProduct,
        forms::FormType,
        groups::CreateGroup,
        imports::ImportSubscriber,
        subscribers::{UpdateSubscriber, UpsertSubscriber},
    },
};
use std::{future::Future, time::Duration};

fn client() -> MailerLite {
    MailerLite::new("unused-test-token").unwrap()
}

fn assert_send_sync<T: Send + Sync>() {}

// Check the future itself, not a reference to it (which would require Sync).
// Its output must also be transferable, as required by tokio::spawn.
fn assert_send_future<F: Future + Send>(_: F)
where
    F::Output: Send,
{
}

fn assert_spawnable<F: Future + Send + 'static>(_: F)
where
    F::Output: Send + 'static,
{
}

#[test]
fn client_and_configuration_are_send_and_sync() {
    assert_send_sync::<MailerLite>();
    assert_send_sync::<MailerLiteBuilder>();
}

#[test]
fn errors_and_metadata_are_send_and_sync() {
    assert_send_sync::<Error>();
    assert_send_sync::<ApiError>();
    assert_send_sync::<DecodeError>();
    assert_send_sync::<mailerlite_rs::JsonDecodeError>();
    assert_send_sync::<ResponseMetadata>();
}

#[test]
fn subscriber_read_write_and_empty_response_futures_are_send() {
    let client = client();
    let subscribers = client.subscribers();

    assert_send_future(subscribers.get("123"));
    assert_send_future(subscribers.upsert(UpsertSubscriber::new("reader@example.test")));
    assert_send_future(subscribers.update("123", UpdateSubscriber::new().field("name", "Reader")));
    assert_send_future(subscribers.delete("123"));
    assert_send_future(subscribers.count());
    assert_send_future(subscribers.list().send());
    assert_send_future(subscribers.activity("123").send());
}

#[test]
fn group_request_futures_are_send() {
    let client = client();

    assert_send_future(client.groups().create(CreateGroup::new("Readers")));
    assert_send_future(client.groups().list().send());
    assert_send_future(client.groups().subscribers("123").send());
}

#[test]
fn segment_request_futures_are_send() {
    let client = client();

    assert_send_future(client.segments().list().send());
    assert_send_future(client.segments().subscribers("123").send());
}

#[test]
fn field_request_future_is_send() {
    assert_send_future(client().fields().list().send());
}

#[test]
fn campaign_request_futures_are_send() {
    let client = client();
    let email = Email::new("Newsletter", "Example", "sender@example.test");

    assert_send_future(
        client
            .campaigns()
            .create(CreateCampaign::new("News", email)),
    );
    assert_send_future(client.campaigns().list().send());
    assert_send_future(client.campaigns().activity("123").send());
}

#[test]
fn automation_and_action_request_futures_are_send() {
    use mailerlite_rs::resources::automations::ActivityStatus;

    let client = client();
    let automations = client.automations();

    assert_send_future(automations.create(CreateAutomation::new("Welcome")));
    assert_send_future(automations.update("123", UpdateAutomation::new("Welcome back")));
    assert_send_future(automations.list().send());
    assert_send_future(
        automations
            .activity("123", ActivityStatus::Completed)
            .send(),
    );
}

#[test]
fn automation_step_request_futures_are_send() {
    let client = client();

    assert_send_future(client.automation_steps().list("123"));
    assert_send_future(
        client
            .automation_steps()
            .create("123", CreateStep::new(StepType::Delay)),
    );
}

#[test]
fn automation_trigger_request_futures_are_send() {
    use mailerlite_rs::resources::automation_triggers::TriggerSubscriberStatus;

    let client = client();
    let triggers = client.automation_triggers();

    assert_send_future(triggers.list("123"));
    assert_send_future(triggers.create(
        "123",
        CreateAutomationTrigger::new("subscriber_joins_group"),
    ));
    assert_send_future(
        triggers
            .subscribers("123", "456", TriggerSubscriberStatus::Completed)
            .send(),
    );
}

#[test]
fn automation_template_request_futures_are_send() {
    let client = client();

    assert_send_future(client.automation_templates().list().send());
    assert_send_future(client.automation_templates().select("123"));
}

#[test]
fn form_request_futures_are_send() {
    let client = client();

    assert_send_future(client.forms().list(FormType::Embedded).send());
    assert_send_future(client.forms().subscribers("123").send());
}

#[test]
fn reference_request_futures_are_send() {
    let client = client();

    assert_send_future(client.reference().timezones());
    assert_send_future(client.reference().campaign_languages());
}

#[test]
fn webhook_request_future_is_send() {
    assert_send_future(client().webhooks().list());
}

#[test]
fn batch_request_future_is_send() {
    let request = BatchRequest::new(BatchMethod::Get, "api/groups").unwrap();

    assert_send_future(client().batch().send(vec![request]));
}

#[test]
fn import_and_polling_futures_are_send() {
    let client = client();
    let imports = client.imports();
    let receipt = ImportReceipt {
        import_progress_url: "https://connect.mailerlite.com/api/subscribers/import/123".into(),
    };

    assert_send_future(
        imports.import_to_group("123", vec![ImportSubscriber::new("reader@example.test")]),
    );
    assert_send_future(imports.get("123"));
    assert_send_future(imports.progress(&receipt));
    assert_send_future(imports.wait(&receipt, Duration::from_secs(1), Duration::from_secs(60)));
}

#[test]
fn ecommerce_request_futures_are_send() {
    let client = client();
    let shop = client.ecommerce().shop("123");

    assert_send_future(client.ecommerce().list().send());
    assert_send_future(client.ecommerce().settings());
    assert_send_future(shop.products().create(SaveProduct::new("Guide")));
    assert_send_future(shop.products().list().send());
    assert_send_future(shop.products().variants("456").list());
    assert_send_future(shop.categories().list().send());
    assert_send_future(shop.customers().list().send());
    assert_send_future(shop.orders().list().send());
    assert_send_future(shop.carts().list().send());
    assert_send_future(shop.carts().items("456").list().send());
}

#[test]
fn owned_client_task_satisfies_tokio_spawn_bounds() {
    let client = client();
    let task_client = client.clone();
    let id = String::from("123");

    assert_spawnable(async move {
        task_client.subscribers().get(id).await?;
        task_client
            .subscribers()
            .upsert(UpsertSubscriber::new("reader@example.test"))
            .await
    });

    assert_send_future(client.subscribers().get("456"));
}

#[cfg(feature = "stream")]
mod streams {
    use super::*;
    use futures_core::Stream;
    use mailerlite_rs::resources::{
        automation_triggers::TriggerSubscriberStatus, automations::ActivityStatus,
    };

    // Streams need Send, but neither Sync nor Unpin is required for task use.
    fn assert_send_stream<S: Stream + Send>(_: S)
    where
        S::Item: Send,
    {
    }

    #[test]
    fn subscriber_cursor_stream_is_send() {
        assert_send_stream(client().subscribers().list().into_stream());
    }

    #[test]
    fn group_numbered_and_membership_cursor_streams_are_send() {
        let client = client();

        assert_send_stream(client.groups().list().into_stream());
        assert_send_stream(client.groups().subscribers("123").into_stream());
    }

    #[test]
    fn segment_numbered_and_membership_cursor_streams_are_send() {
        let client = client();

        assert_send_stream(client.segments().list().into_stream());
        assert_send_stream(client.segments().subscribers("123").into_stream());
    }

    #[test]
    fn field_stream_is_send() {
        assert_send_stream(client().fields().list().into_stream());
    }

    #[test]
    fn campaign_and_activity_streams_are_send() {
        let client = client();

        assert_send_stream(client.campaigns().list().into_stream());
        assert_send_stream(client.campaigns().activity("123").into_stream());
    }

    #[test]
    fn automation_and_activity_streams_are_send() {
        let client = client();

        assert_send_stream(client.automations().list().into_stream());
        assert_send_stream(
            client
                .automations()
                .activity("123", ActivityStatus::Completed)
                .into_stream(),
        );
    }

    #[test]
    fn automation_trigger_subscriber_stream_is_send() {
        assert_send_stream(
            client()
                .automation_triggers()
                .subscribers("123", "456", TriggerSubscriberStatus::Completed)
                .into_stream(),
        );
    }

    #[test]
    fn automation_template_stream_is_send() {
        assert_send_stream(client().automation_templates().list().into_stream());
    }

    #[test]
    fn form_and_subscriber_streams_are_send() {
        let client = client();

        assert_send_stream(client.forms().list(FormType::Embedded).into_stream());
        assert_send_stream(client.forms().subscribers("123").into_stream());
    }

    #[test]
    fn ecommerce_streams_are_send_for_each_resource_type() {
        let client = client();
        let shop = client.ecommerce().shop("123");

        assert_send_stream(client.ecommerce().list().into_stream());
        assert_send_stream(shop.products().list().into_stream());
        assert_send_stream(shop.categories().list().into_stream());
        assert_send_stream(shop.customers().list().into_stream());
        assert_send_stream(shop.orders().list().into_stream());
        assert_send_stream(shop.carts().list().into_stream());
        assert_send_stream(shop.carts().items("456").list().into_stream());
    }

    #[test]
    fn owned_stream_consumer_satisfies_tokio_spawn_bounds() {
        use futures_util::TryStreamExt;

        let client = client();

        assert_spawnable(async move {
            let stream = client.subscribers().list().into_stream();
            tokio::pin!(stream);

            stream.try_next().await
        });
    }
}
