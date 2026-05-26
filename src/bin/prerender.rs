// SPDX-License-Identifier: AGPL-3.0-or-later

use clause_obligation_ledger_rs::core::sample_payload;
use clause_obligation_ledger_rs::render::{
    render_docs, render_ledger_lane, render_obligation_events, render_overview, render_verification,
};
use std::fs;
use std::path::Path;

fn main() {
    let site = Path::new("site");
    let _ = fs::remove_dir_all(site);
    fs::create_dir_all(site.join("api").join("dashboard").join("summary")).unwrap();

    write("site/index.html", &render_overview());
    write("site/ledger-lane.html", &render_ledger_lane());
    write("site/obligation-events.html", &render_obligation_events());
    write("site/verification.html", &render_verification());
    write("site/docs.html", &render_docs());

    let payload = sample_payload();
    write("site/api/dashboard/summary/index.json", &serde_json::to_string_pretty(&payload.summary).unwrap());
    write("site/api/ledger-lane.json", &serde_json::to_string_pretty(&payload.ledger_lane).unwrap());
    write(
        "site/api/obligation-events.json",
        &serde_json::to_string_pretty(&payload.obligation_events).unwrap(),
    );
    write("site/api/verification.json", &serde_json::to_string_pretty(&payload.verification).unwrap());
    write("site/api/sample.json", &serde_json::to_string_pretty(&payload).unwrap());

    if let Ok(cname) = fs::read_to_string("CNAME") {
        write("site/CNAME", &cname);
    }
}

fn write(path: &str, content: &str) {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}
