//! Construct endpoint URLs while preserving the configured base path.
use crate::{Error, Result};
use reqwest::Url;

pub(super) fn endpoint(base: &Url, path: &[&str]) -> Result<Url> {
    if path
        .iter()
        .any(|segment| segment.is_empty() || *segment == "." || *segment == "..")
    {
        return Err(Error::Config(
            "API path segments must be nonempty and cannot be dot segments".into(),
        ));
    }
    let mut url = base.clone();
    let mut segments = url
        .path_segments_mut()
        .map_err(|_| Error::Config("base URL cannot contain path segments".into()))?;
    segments.pop_if_empty();
    for segment in path {
        segments.push(segment);
    }
    drop(segments);
    Ok(url)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::*;

    #[test]
    fn paths_preserve_base_and_encode_identifiers() {
        let base = Url::parse("https://example.test/api/").unwrap();
        let url = endpoint(&base, &["subscribers", "a/b+tag@example.test"]).unwrap();
        assert_eq!(url.path(), "/api/subscribers/a%2Fb+tag@example.test");
        assert!(endpoint(&base, &[".."]).is_err());
        assert!(endpoint(&base, &["."]).is_err());
    }
}
