//! Discoverable resource accessors on the shared MailerLite client.
use crate::{MailerLite, resources::*};

impl MailerLite {
    /// Manage shops, products, orders, and e-commerce reporting.
    pub fn ecommerce(&self) -> ecommerce::Ecommerce<'_> {
        ecommerce::Ecommerce::new(self)
    }
    /// Manage subscribers and their activity.
    pub fn subscribers(&self) -> subscribers::Subscribers<'_> {
        subscribers::Subscribers::new(self)
    }
    /// Manage groups and subscriber membership.
    pub fn groups(&self) -> groups::Groups<'_> {
        groups::Groups::new(self)
    }
    /// Manage existing subscriber segments.
    pub fn segments(&self) -> segments::Segments<'_> {
        segments::Segments::new(self)
    }
    /// Manage custom subscriber fields.
    pub fn fields(&self) -> fields::Fields<'_> {
        fields::Fields::new(self)
    }
    /// Manage campaigns and scheduling.
    pub fn campaigns(&self) -> campaigns::Campaigns<'_> {
        campaigns::Campaigns::new(self)
    }
    /// Manage automations, their actions, and reports.
    pub fn automations(&self) -> automations::Automations<'_> {
        automations::Automations::new(self)
    }
    /// Manage automation steps, their execution counts, and subscribers.
    pub fn automation_steps(&self) -> automation_steps::AutomationSteps<'_> {
        automation_steps::AutomationSteps::new(self)
    }
    /// Manage automation triggers and their subscriber activity.
    pub fn automation_triggers(&self) -> automation_triggers::AutomationTriggers<'_> {
        automation_triggers::AutomationTriggers::new(self)
    }
    /// List automation templates and create automations from them.
    pub fn automation_templates(&self) -> automation_templates::AutomationTemplates<'_> {
        automation_templates::AutomationTemplates::new(self)
    }
    /// Manage existing forms and their subscribers.
    pub fn forms(&self) -> forms::Forms<'_> {
        forms::Forms::new(self)
    }
    /// Retrieve reference data such as timezones and languages.
    pub fn reference(&self) -> reference::Reference<'_> {
        reference::Reference::new(self)
    }
    /// Manage webhook subscriptions.
    pub fn webhooks(&self) -> webhooks::Webhooks<'_> {
        webhooks::Webhooks::new(self)
    }
    /// Submit batches of API operations.
    pub fn batch(&self) -> batch::Batch<'_> {
        batch::Batch::new(self)
    }
    /// Start group imports and inspect subscriber import progress.
    pub fn imports(&self) -> imports::Imports<'_> {
        imports::Imports::new(self)
    }
}
