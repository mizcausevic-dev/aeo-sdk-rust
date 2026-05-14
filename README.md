# aeo-sdk-rust

Rust SDK for the [AEO Protocol v0.1](https://github.com/mizcausevic-dev/aeo-protocol-spec) — parse, build, validate, and fetch AEO declaration documents.

[![Crates.io](https://img.shields.io/crates/v/aeo-protocol.svg)](https://crates.io/crates/aeo-protocol)

## Install

```toml
[dependencies]
aeo-protocol = "0.1"
```

## Quickstart

```rust
use aeo_protocol::{Document, fetch_well_known};

fn main() -> Result<(), aeo_protocol::AeoError> {
    // Fetch and parse from a live well-known URL
    let doc = fetch_well_known("https://mizcausevic-dev.github.io")?;
    println!("{}", doc.entity.name);                    // "Miz Causevic"
    println!("{:?}", doc.claim_ids());                  // ["current-role", ...]
    println!("{:?}", doc.find_claim("years-experience").map(|c| &c.value));

    // Parse from a string
    let raw = std::fs::read_to_string("aeo.json")?;
    let parsed = Document::from_json(&raw)?;
    println!("{}", parsed.to_json()?);
    Ok(())
}
```

## What it does

- **Parse** — `Document::from_json(&str)` returns a strongly-typed `Document`
- **Build** — `Document`, `Entity`, `Authority`, `Claim`, `Verification`, `CitationPreferences`, `AnswerConstraints`, `Audit` are all public struct types with `serde::Serialize` + `serde::Deserialize`
- **Serialize** — `doc.to_json()` returns canonical pretty-printed JSON
- **Fetch** — `fetch_well_known(origin)` performs HTTP discovery against `/.well-known/aeo.json` with `Accept: application/aeo+json, application/json` (feature `client`, on by default)
- **Query** — `doc.claim_ids()` and `doc.find_claim(id)` helpers

## Features

- `client` (default) — HTTP discovery via [ureq](https://crates.io/crates/ureq). Disable with `default-features = false` for a pure-serde build (no networking).

## Conformance

Supports the AEO Protocol at **conformance Level 1 (Declare)**. Signature verification (L2) and audit-endpoint posting (L3) deferred to v0.2.

## Dependencies

- [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) — JSON model
- [thiserror](https://crates.io/crates/thiserror) — error type
- [ureq](https://crates.io/crates/ureq) — HTTP client (optional, `client` feature only)

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Specification

Full spec at [github.com/mizcausevic-dev/aeo-protocol-spec](https://github.com/mizcausevic-dev/aeo-protocol-spec).

## License

MIT-licensed. Free for commercial and non-commercial use with attribution. The AEO Protocol specification this SDK implements is also MIT (see [aeo-protocol-spec](https://github.com/mizcausevic-dev/aeo-protocol-spec)).

## Kinetic Gain Protocol Suite

| Spec | Implementation |
|---|---|
| [AEO Protocol](https://github.com/mizcausevic-dev/aeo-protocol-spec) | [aeo-sdk-python](https://github.com/mizcausevic-dev/aeo-sdk-python) · [aeo-sdk-typescript](https://github.com/mizcausevic-dev/aeo-sdk-typescript) · **aeo-sdk-rust** (this) · [aeo-sdk-go](https://github.com/mizcausevic-dev/aeo-sdk-go) · [aeo-cli](https://github.com/mizcausevic-dev/aeo-cli) · [aeo-crawler](https://github.com/mizcausevic-dev/aeo-crawler) |
| [Prompt Provenance](https://github.com/mizcausevic-dev/prompt-provenance-spec) | — |
| [Agent Cards](https://github.com/mizcausevic-dev/agent-cards-spec) | — |
| [AI Evidence Format](https://github.com/mizcausevic-dev/ai-evidence-format-spec) | — |
| [MCP Tool Cards](https://github.com/mizcausevic-dev/mcp-tool-card-spec) | — |

---

**Connect:** [LinkedIn](https://www.linkedin.com/in/mirzacausevic/) · [Kinetic Gain](https://kineticgain.com) · [Medium](https://medium.com/@mizcausevic/) · [Skills](https://mizcausevic.com/skills/)
