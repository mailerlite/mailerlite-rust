//! Receive MailerLite webhook events and optionally verify their signatures.
//!
//! Payload types support the documented single and batched delivery formats.
//! Enable `webhook-signatures` to authenticate the **original body bytes** with
//! `verify_signature` before deserializing them. Deserialization alone does not
//! authenticate a delivery. Unknown event names remain explicitly inspectable;
//! malformed payloads for known events return a decoding error.

mod automations;
mod campaigns;
mod events;
mod groups;
mod payload;
mod subscribers;

pub use automations::{AutomationEvent, WebhookAutomation};
pub use campaigns::{CampaignClickEvent, CampaignOpenEvent, CampaignSentEvent, WebhookCampaign};
pub use events::{UnknownEvent, WebhookEvent};
pub use groups::{GroupEvent, WebhookGroup};
pub use payload::{WebhookBatch, WebhookPayload};
pub use subscribers::{
    DeletedSubscriberEvent, SubscriberEvent, WebhookLocation, WebhookSubscriber,
};

#[cfg(feature = "webhook-signatures")]
mod signature;
#[cfg(feature = "webhook-signatures")]
pub use signature::{SignatureError, verify_signature};
