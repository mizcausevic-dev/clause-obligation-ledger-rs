# Architecture

`clause-obligation-ledger-rs` is a Rust + Axum control plane for turning contractual obligations into durable, append-only operating events.

Core layers:

- `overview`
  - aggregate renewal posture
  - evidence completeness
  - overdue acknowledgements
- `ledger-lane`
  - agreement-level obligation ledger
  - owner lanes
  - renewal windows
- `obligation-events`
  - append-only event history
  - evidence references
  - status transitions
- `verification`
  - review packet readiness
  - blocker and next-action posture

The repo complements `contract-clause-obligation-graph`:

- `clauses.kineticgain.com` shows clause and dependency visibility
- `ledger.kineticgain.com` shows obligation-event durability and audit posture

Together they keep the primitive coherent without duplicating the same operator story.
