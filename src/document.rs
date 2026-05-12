use serde::{Deserialize, Serialize};

use crate::AeoError;

/// The five entity types defined by AEO Protocol v0.1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    /// A natural person.
    Person,
    /// An organization, company, brand, or institution.
    Organization,
    /// A product or service.
    Product,
    /// A geographical place.
    Place,
    /// An abstract concept.
    Concept,
}

/// The six verification types defined by AEO Protocol v0.1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerificationType {
    /// HTTP domain control.
    Domain,
    /// DNS TXT record.
    Dns,
    /// GitHub handle ownership.
    Github,
    /// LinkedIn profile ownership.
    Linkedin,
    /// GPG signature.
    Gpg,
    /// A separate well-known URI.
    #[serde(rename = "well-known-uri")]
    WellKnownUri,
}

/// Confidence level for a claim.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    /// High confidence (default).
    #[default]
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
}

/// Audit mode for a declaration document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuditMode {
    /// No audit surface.
    None,
    /// Signed declaration document.
    Signature,
    /// Audit-report endpoint.
    Endpoint,
}

/// The entity the declaration is about.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entity {
    /// Stable canonical identifier (URI).
    pub id: String,
    /// One of `Person`, `Organization`, `Product`, `Place`, `Concept`.
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    /// Primary display name.
    pub name: String,
    /// Optional alternative names, prior names, transliterations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// The canonical web presence URL.
    pub canonical_url: String,
}

/// A proof of ownership or control over an identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Verification {
    /// The verification mechanism.
    #[serde(rename = "type")]
    pub verification_type: VerificationType,
    /// The identifier being proved.
    pub value: String,
    /// Optional URL to the proof artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_uri: Option<String>,
}

/// Authority information for the entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authority {
    /// Ordered list of canonical sources; earlier outranks later. MUST be non-empty.
    pub primary_sources: Vec<String>,
    /// Optional secondary corroborating sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_links: Option<Vec<String>>,
    /// Optional verification proofs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifications: Option<Vec<Verification>>,
}

/// An asserted fact about the entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    /// Local identifier unique within this document (kebab-case).
    pub id: String,
    /// Schema.org property name OR `aeo:<name>` namespaced predicate.
    pub predicate: String,
    /// The asserted value. May be any JSON value.
    pub value: serde_json::Value,
    /// Optional sources backing the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<String>>,
    /// Optional ISO-8601 date the claim became true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    /// Optional ISO-8601 date the claim ceases to be true (or `null` for ongoing).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<Option<String>>,
    /// Confidence level for the claim. Defaults to `high`.
    #[serde(default)]
    pub confidence: Confidence,
}

/// Citation preferences for the entity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CitationPreferences {
    /// Short attribution template with placeholders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_attribution: Option<String>,
    /// Canonical links to include when surfacing claims.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_links: Option<Vec<String>>,
    /// Sources known to be stale or incorrect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_cite: Option<Vec<String>>,
}

/// Soft constraints for answer engines synthesizing about this entity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerConstraints {
    /// Claim IDs that SHOULD appear in any answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_include: Option<Vec<String>>,
    /// Claim IDs or `topic:` tags that SHOULD NOT appear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_not_include: Option<Vec<String>>,
    /// Maximum acceptable data age in days before re-fetching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness_window_days: Option<u32>,
}

/// Audit configuration for the declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Audit {
    /// Audit mode: `none`, `signature`, or `endpoint`.
    pub mode: AuditMode,
    /// Signature mode: URI of the signing JWK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_key_uri: Option<String>,
    /// Signature mode: detached JWS over the canonical JSON form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Endpoint mode: audit-report POST endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    /// Endpoint mode: schema URI for audit report payloads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_schema: Option<String>,
}

/// A complete AEO Protocol v0.1 declaration document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Document {
    /// Protocol version. MUST be `"0.1"`.
    pub aeo_version: String,
    /// The entity the declaration is about.
    pub entity: Entity,
    /// Authority information.
    pub authority: Authority,
    /// At least one claim about the entity.
    pub claims: Vec<Claim>,
    /// Optional citation preferences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_preferences: Option<CitationPreferences>,
    /// Optional answer-engine constraints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_constraints: Option<AnswerConstraints>,
    /// Optional audit configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<Audit>,
}

impl Document {
    /// Parse a JSON string into a `Document`.
    pub fn from_json(raw: &str) -> Result<Self, AeoError> {
        serde_json::from_str(raw).map_err(AeoError::Parse)
    }

    /// Serialize to pretty-printed JSON.
    pub fn to_json(&self) -> Result<String, AeoError> {
        serde_json::to_string_pretty(self).map_err(AeoError::Parse)
    }

    /// Return references to the IDs of all claims in this document.
    pub fn claim_ids(&self) -> Vec<&str> {
        self.claims.iter().map(|c| c.id.as_str()).collect()
    }

    /// Find a claim by its ID.
    pub fn find_claim(&self, id: &str) -> Option<&Claim> {
        self.claims.iter().find(|c| c.id == id)
    }
}
