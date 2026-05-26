// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Summary {
    pub agreements: usize,
    pub active_obligations: usize,
    pub overdue_events: usize,
    pub renewal_windows: usize,
    pub evidence_gaps: usize,
    pub signal: &'static str,
}

#[derive(Clone, Serialize)]
pub struct LedgerRecord {
    pub agreement_id: &'static str,
    pub counterparty: &'static str,
    pub lane: &'static str,
    pub clause_focus: &'static str,
    pub owner: &'static str,
    pub renewal_window_days: i32,
    pub status: &'static str,
    pub next_action: &'static str,
}

#[derive(Clone, Serialize)]
pub struct ObligationEvent {
    pub event_id: &'static str,
    pub agreement_id: &'static str,
    pub event_type: &'static str,
    pub actor: &'static str,
    pub due_in_days: i32,
    pub evidence_state: &'static str,
    pub status: &'static str,
    pub note: &'static str,
}

#[derive(Clone, Serialize)]
pub struct VerificationGate {
    pub packet_id: &'static str,
    pub audience: &'static str,
    pub completeness: i32,
    pub blocker: &'static str,
    pub status: &'static str,
    pub next_action: &'static str,
}

#[derive(Clone, Serialize)]
pub struct Payload {
    pub summary: Summary,
    pub ledger_lane: Vec<LedgerRecord>,
    pub obligation_events: Vec<ObligationEvent>,
    pub verification: Vec<VerificationGate>,
}

pub fn sample_payload() -> Payload {
    Payload {
        summary: Summary {
            agreements: 6,
            active_obligations: 18,
            overdue_events: 3,
            renewal_windows: 4,
            evidence_gaps: 2,
            signal: "Append-only obligation events keep renewal and audit posture legible after clause discovery is done.",
        },
        ledger_lane: vec![
            LedgerRecord {
                agreement_id: "AGR-114",
                counterparty: "Northbridge Health Exchange",
                lane: "privacy / security",
                clause_focus: "Retention attestation cadence",
                owner: "security governance",
                renewal_window_days: 42,
                status: "watch",
                next_action: "Attach fresh deletion attestation before review closes.",
            },
            LedgerRecord {
                agreement_id: "AGR-227",
                counterparty: "Summit Freight Systems",
                lane: "sla / commercial",
                clause_focus: "Service-credit notice timing",
                owner: "revops ops",
                renewal_window_days: 21,
                status: "critical",
                next_action: "Log notice delivery event with signatory proof.",
            },
            LedgerRecord {
                agreement_id: "AGR-318",
                counterparty: "Union Grid Services",
                lane: "compliance evidence",
                clause_focus: "Audit export retention",
                owner: "platform reliability",
                renewal_window_days: 33,
                status: "healthy",
                next_action: "Preserve monthly export checksum in the packet.",
            },
            LedgerRecord {
                agreement_id: "AGR-442",
                counterparty: "LexPoint Advisory",
                lane: "vendor governance",
                clause_focus: "Subprocessor refresh notice",
                owner: "legal ops",
                renewal_window_days: 18,
                status: "watch",
                next_action: "Acknowledge registry delta and update customer notice event.",
            },
            LedgerRecord {
                agreement_id: "AGR-590",
                counterparty: "Vertex Care Network",
                lane: "renewal / notice",
                clause_focus: "Opt-out and delivery sequencing",
                owner: "executive review",
                renewal_window_days: 12,
                status: "critical",
                next_action: "Lock signatory path and courier evidence now.",
            },
        ],
        obligation_events: vec![
            ObligationEvent {
                event_id: "EV-901",
                agreement_id: "AGR-114",
                event_type: "attestation requested",
                actor: "security governance",
                due_in_days: 5,
                evidence_state: "partial",
                status: "watch",
                note: "Customer-facing retention packet still missing the signed deletion report.",
            },
            ObligationEvent {
                event_id: "EV-918",
                agreement_id: "AGR-227",
                event_type: "notice queued",
                actor: "revops ops",
                due_in_days: 2,
                evidence_state: "missing",
                status: "critical",
                note: "Delivery mechanism selected but proof artifact not attached.",
            },
            ObligationEvent {
                event_id: "EV-930",
                agreement_id: "AGR-318",
                event_type: "evidence archived",
                actor: "platform reliability",
                due_in_days: 14,
                evidence_state: "ready",
                status: "healthy",
                note: "Checksum, audit export, and reviewer note all preserved.",
            },
            ObligationEvent {
                event_id: "EV-947",
                agreement_id: "AGR-442",
                event_type: "registry delta acknowledged",
                actor: "legal ops",
                due_in_days: 7,
                evidence_state: "partial",
                status: "watch",
                note: "Vendor notice drafted, but downstream customer disclosure event is still open.",
            },
            ObligationEvent {
                event_id: "EV-963",
                agreement_id: "AGR-590",
                event_type: "renewal opt-out staged",
                actor: "executive review",
                due_in_days: 1,
                evidence_state: "missing",
                status: "critical",
                note: "Signer path and courier tracking still not bound to the ledger.",
            },
            ObligationEvent {
                event_id: "EV-972",
                agreement_id: "AGR-590",
                event_type: "counsel review completed",
                actor: "outside counsel",
                due_in_days: 0,
                evidence_state: "ready",
                status: "healthy",
                note: "Interpretation is settled; only delivery evidence remains.",
            },
        ],
        verification: vec![
            VerificationGate {
                packet_id: "PK-17",
                audience: "renewal committee",
                completeness: 81,
                blocker: "Notice delivery proof still open",
                status: "watch",
                next_action: "Attach courier ID and signer artifact.",
            },
            VerificationGate {
                packet_id: "PK-31",
                audience: "customer assurance",
                completeness: 93,
                blocker: "No active blocker",
                status: "healthy",
                next_action: "Hold packet for governed publication.",
            },
            VerificationGate {
                packet_id: "PK-44",
                audience: "legal leadership",
                completeness: 67,
                blocker: "Cross-team event chain still fragmented",
                status: "critical",
                next_action: "Reconcile missing acknowledgement events before sign-off.",
            },
        ],
    }
}
