use crate::{AeoError, Document};

const WELL_KNOWN_PATH: &str = "/.well-known/aeo.json";
const ACCEPT_HEADER: &str = "application/aeo+json, application/json";

/// Build the canonical well-known URL for an origin.
///
/// Strips any trailing slashes from the origin and appends the AEO
/// Protocol well-known path.
pub fn well_known_url(origin: &str) -> String {
    format!("{}{}", origin.trim_end_matches('/'), WELL_KNOWN_PATH)
}

/// Fetch and parse the AEO declaration at `origin`'s well-known URL.
///
/// Returns an [`AeoError`] for non-2xx responses, network errors,
/// or malformed documents.
pub fn fetch_well_known(origin: &str) -> Result<Document, AeoError> {
    let url = well_known_url(origin);
    let response = ureq::get(&url)
        .set("Accept", ACCEPT_HEADER)
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(status, _) => AeoError::HttpStatus {
                status,
                url: url.clone(),
            },
            other => AeoError::Http(Box::new(other)),
        })?;

    let body = response.into_string()?;
    Document::from_json(&body)
}
