//! Timezones: <https://developers.mailerlite.com/api/timezones>
//!
//! Timezone ids are used when scheduling campaigns.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    MailerLite, Result,
    types::{Id, de},
};

/// A timezone MailerLite accepts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Timezone {
    /// Timezone id, e.g. `"370"`.
    pub id: Id,
    /// IANA name, e.g. `Europe/Vilnius`.
    pub name: String,
    /// Display name, e.g. `Europe/Vilnius (+03:00)`.
    #[serde(default)]
    pub name_for_humans: String,
    /// UTC offset, e.g. `+03:00`.
    #[serde(default, deserialize_with = "de::null_default")]
    pub offset_name: String,
    /// UTC offset in minutes, e.g. `180`.
    #[serde(default, deserialize_with = "de::null_default")]
    pub offset: i32,
}

/// Timezone endpoints. Obtained with [`MailerLite::timezones`].
#[derive(Clone, Copy, Debug)]
pub struct Timezones<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// Timezone endpoints.
    #[must_use]
    pub fn timezones(&self) -> Timezones<'_> {
        Timezones { client: self }
    }
}

impl Timezones<'_> {
    /// `GET /timezones`: every available timezone (not paginated).
    pub async fn list(self) -> Result<Vec<Timezone>> {
        self.client.call(Method::GET, &["timezones"]).data().await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;

    use crate::http::mock::{client, fixture};

    #[tokio::test]
    async fn list() {
        let (client, mock) = client();
        mock.respond(200, fixture("timezones_list"));

        let timezones = client.timezones().list().await.unwrap();

        mock.request().assert(Method::GET, "timezones").no_body();
        let timezone = &timezones[0];
        assert_eq!(timezone.id, "370");
        assert_eq!(timezone.name, "Europe/Vilnius");
        assert_eq!(timezone.name_for_humans, "Europe/Vilnius (+03:00)");
        assert_eq!(timezone.offset_name, "+03:00");
        assert_eq!(timezone.offset, 180);
    }
}
