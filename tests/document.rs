use aeo_protocol::{well_known_url, Document, EntityType};
use pretty_assertions::assert_eq;

const FIXTURE: &str = include_str!("fixtures/aeo-person.json");

#[test]
fn loads_canonical_person_example() {
    let doc = Document::from_json(FIXTURE).expect("fixture should parse");
    assert_eq!(doc.aeo_version, "0.1");
    assert_eq!(doc.entity.entity_type, EntityType::Person);
    assert_eq!(doc.entity.name, "Miz Causevic");
    assert_eq!(doc.claims.len(), 6);
}

#[test]
fn claim_ids_round_trip() {
    let doc = Document::from_json(FIXTURE).unwrap();
    let mut ids: Vec<&str> = doc.claim_ids();
    ids.sort();
    assert_eq!(
        ids,
        vec![
            "authored-spec",
            "current-role",
            "live-products",
            "location",
            "primary-stack",
            "years-experience",
        ]
    );
}

#[test]
fn find_claim_returns_claim() {
    let doc = Document::from_json(FIXTURE).unwrap();
    let claim = doc.find_claim("years-experience").expect("present");
    assert_eq!(claim.predicate, "aeo:yearsOfExperience");
    assert_eq!(claim.value, serde_json::json!(30));
}

#[test]
fn find_claim_missing_is_none() {
    let doc = Document::from_json(FIXTURE).unwrap();
    assert!(doc.find_claim("does-not-exist").is_none());
}

#[test]
fn round_trip_serialization_preserves_structure() {
    let doc = Document::from_json(FIXTURE).unwrap();
    let re_serialized = doc.to_json().unwrap();
    let re_parsed = Document::from_json(&re_serialized).unwrap();
    assert_eq!(re_parsed.entity.name, doc.entity.name);
    assert_eq!(re_parsed.claim_ids(), doc.claim_ids());
    assert_eq!(
        re_parsed.authority.primary_sources,
        doc.authority.primary_sources
    );
}

#[test]
fn rejects_unknown_top_level_field() {
    let bad = FIXTURE.replacen(
        "\"aeo_version\": \"0.1\"",
        "\"aeo_version\": \"0.1\",\n  \"unexpected_field\": \"nope\"",
        1,
    );
    let result = Document::from_json(&bad);
    assert!(result.is_err(), "unexpected_field must cause parse to fail");
}

#[test]
fn well_known_url_strips_trailing_slashes() {
    assert_eq!(
        well_known_url("https://example.com"),
        "https://example.com/.well-known/aeo.json"
    );
    assert_eq!(
        well_known_url("https://example.com/"),
        "https://example.com/.well-known/aeo.json"
    );
    assert_eq!(
        well_known_url("https://example.com///"),
        "https://example.com/.well-known/aeo.json"
    );
}
