//! Typed automation configuration verified against the backend step and trigger classes.
//!
//! Request configuration is deliberately separate from enriched response fields.
//! Identifier values retain their original string or integer representation. Known
//! fields are decoded strictly; only future extension fields retain arbitrary JSON.
mod common;
mod steps;
mod triggers;
mod wire;

pub use common::{
    ConfigurationId, ConfigurationValue, ExtensionFields, RelatedObject, TimezoneMode,
};
pub use steps::{
    AbPath, AbTestConfiguration, ActionConfiguration, ActionType, Condition,
    ConditionConfiguration, ConditionMatching, ConditionType, DelayConfiguration, DelayUnit,
    ElapsedDateAction, EmailConfiguration, NotificationField, NotificationFields, PathAllocation,
    StepConfiguration, StepKind, StepProperties, WebhookConfiguration, Weekday,
};
pub use triggers::{
    AbandonedTriggerConfiguration, CategoryTriggerConfiguration, DateOffsetUnit,
    DateTriggerConfiguration, DurationUnit, FieldTriggerConfiguration, FormTriggerConfiguration,
    GroupTriggerConfiguration, LinkTriggerConfiguration, ProductTriggerConfiguration,
    PurchaseFrequencyConfiguration, SegmentDisplayRule, SegmentTriggerConfiguration,
    ShopTriggerConfiguration, TriggerConfiguration, TriggerProperties,
};

pub(crate) use wire::{STEP_FIELDS, TRIGGER_FIELDS, configuration_fields};
