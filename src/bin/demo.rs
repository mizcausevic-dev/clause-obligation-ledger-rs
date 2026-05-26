// SPDX-License-Identifier: AGPL-3.0-or-later

use clause_obligation_ledger_rs::core::sample_payload;

fn main() {
    let payload = sample_payload();
    println!(
        "agreements={} active_obligations={} overdue_events={} renewal_windows={} evidence_gaps={}",
        payload.summary.agreements,
        payload.summary.active_obligations,
        payload.summary.overdue_events,
        payload.summary.renewal_windows,
        payload.summary.evidence_gaps
    );
}
