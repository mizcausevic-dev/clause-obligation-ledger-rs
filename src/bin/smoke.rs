// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

fn main() {
    let required = [
        ("site/index.html", "Clause obligation ledger"),
        ("site/ledger-lane.html", "Ledger lane"),
        ("site/obligation-events.html", "Obligation events"),
        ("site/verification.html", "Verification posture"),
        ("site/docs.html", "System docs"),
        ("site/api/sample.json", "Northbridge Health Exchange"),
    ];

    for (path, needle) in required {
        let data = fs::read_to_string(path).unwrap();
        assert!(data.contains(needle), "{path} missing {needle}");
    }

    println!("smoke ok");
}
