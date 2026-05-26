// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::core::sample_payload;

const STYLE: &str = r#"
  :root{
    --bg:#070a0f; --panel:#0b1220; --panel2:#0a1426;
    --line:rgba(120,255,170,.18); --line2:rgba(120,255,170,.10);
    --text:#e9f3ff; --muted:rgba(233,243,255,.72); --muted2:rgba(233,243,255,.55);
    --bert:#37ff8b; --bert2:#19c7ff;
    --warn:#ffcc66; --bad:#ff5c7a; --good:#37ff8b; --plum:#b88cff;
    --shadow: 0 18px 60px rgba(0,0,0,.55);
    --radius: 18px;
    --mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    --sans: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, "Apple Color Emoji", "Segoe UI Emoji";
  }
  *{box-sizing:border-box} html,body{height:100%} body{
    margin:0; font-family:var(--sans); color:var(--text);
    background:
      radial-gradient(1200px 600px at 20% -10%, rgba(55,255,139,.18), transparent 60%),
      radial-gradient(900px 520px at 90% 0%, rgba(25,199,255,.16), transparent 55%),
      radial-gradient(1000px 600px at 50% 110%, rgba(55,255,139,.10), transparent 60%),
      linear-gradient(180deg, #05070c 0%, #070a0f 35%, #05070c 100%);
    overflow-x:hidden;
  }
  .grid-bg{position:fixed; inset:0; pointer-events:none; opacity:.12; z-index:-1;background-image:linear-gradient(to right, rgba(55,255,139,.14) 1px, transparent 1px),linear-gradient(to bottom, rgba(55,255,139,.10) 1px, transparent 1px);background-size:46px 46px;mask-image:radial-gradient(900px 600px at 40% 10%, #000 60%, transparent 100%);}
  .wrap{max-width:1280px; margin:0 auto; padding:24px 22px 80px}
  .topbar{display:flex; justify-content:space-between; align-items:flex-start; gap:14px;border-bottom:1px solid var(--line2); padding-bottom:14px; margin-bottom:22px;font-family:var(--mono); font-size:11px; letter-spacing:.16em; color:var(--muted);text-transform:uppercase;}
  .topbar .left{color:var(--bert)} .topbar .right{text-align:right; color:var(--muted)} .topbar .right div{margin-bottom:4px}
  .herorow{display:grid; grid-template-columns: 1.5fr .9fr; gap:18px} @media (max-width:1000px){.herorow{grid-template-columns:1fr}}
  .hero{background: linear-gradient(180deg, rgba(11,18,32,.95), rgba(8,14,26,.92));border:1px solid var(--line); border-radius:22px; padding:28px 28px 24px;box-shadow: var(--shadow); position:relative; overflow:hidden;border-top:2px solid var(--bert2);}
  .hero h1{ font-size:64px; line-height:.95; margin:0 0 18px; letter-spacing:-.5px; font-weight:800; } @media (max-width:700px){.hero h1{font-size:42px}}
  .hero p{color:var(--muted); font-size:15px; line-height:1.55; max-width:680px; margin:0 0 18px}
  .chiprow{display:flex; flex-wrap:wrap; gap:8px} .meta-chip{font-family:var(--mono); font-size:11px; color:var(--muted);padding:7px 12px; border-radius:999px; border:1px solid var(--line);background:rgba(6,10,18,.4);}
  .side{display:flex; flex-direction:column; gap:14px}
  .corr{border:1px solid var(--bert); border-left:4px solid var(--bert);background: linear-gradient(180deg, rgba(55,255,139,.06), rgba(11,18,32,.92));border-radius:14px; padding:16px 18px;}
  .corr .lbl{font-family:var(--mono); font-size:10px; letter-spacing:.18em; text-transform:uppercase;color:var(--bert)}
  .corr p{color:var(--muted); font-size:13.5px; line-height:1.55; margin:6px 0 0}
  .section{margin-top:34px}
  .sh{display:flex; justify-content:space-between; align-items:baseline; gap:14px;padding-bottom:10px; border-bottom:1px solid var(--line2); margin-bottom:14px;}
  .sh h2{margin:0; font-size:24px; font-weight:600; letter-spacing:-.2px}
  .sh .note{font-family:var(--mono); font-size:11px; color:var(--muted2); letter-spacing:.16em; text-transform:uppercase}
  .kpis{display:grid; grid-template-columns: repeat(5, 1fr); gap:12px} @media (max-width:1100px){.kpis{grid-template-columns: repeat(3, 1fr)}} @media (max-width:640px){.kpis{grid-template-columns: repeat(2, 1fr)}}
  .kpi{border:1px solid var(--line); border-radius:14px; padding:14px 14px 12px;background: linear-gradient(180deg, rgba(11,18,32,.85), rgba(8,14,26,.65));}
  .kpi .v{font-family:var(--mono); font-size:26px; font-weight:600; letter-spacing:-.5px;color:var(--bert2)}
  .kpi .lbl{font-family:var(--mono); font-size:10px; letter-spacing:.18em; text-transform:uppercase; color:var(--muted); margin-top:6px}
  .kpi .h{font-size:12px; color:var(--muted); line-height:1.45; margin-top:8px}
  .cards{display:grid; grid-template-columns: repeat(3,1fr); gap:14px} @media (max-width:1000px){.cards{grid-template-columns:1fr}}
  .card{border:1px solid var(--line); border-radius:16px; padding:18px 20px;background: linear-gradient(180deg, rgba(11,18,32,.85), rgba(8,14,26,.65));}
  .card h3{margin:8px 0 10px; font-size:19px}
  .eyebrow{font-family:var(--mono); font-size:11px; color:var(--bert2); letter-spacing:.18em; text-transform:uppercase}
  .card p, .card li, table td, table th {color:var(--muted); line-height:1.55}
  table{width:100%; border-collapse:separate; border-spacing:0; border:1px solid var(--line); border-radius:14px; overflow:hidden}
  th,td{padding:13px 14px; text-align:left; font-size:13.5px; vertical-align:top}
  thead th{font-family:var(--mono); font-size:11px; letter-spacing:.16em; text-transform:uppercase;color:var(--muted2); border-bottom:1px solid var(--line); background:rgba(11,18,32,.5);}
  .tag{display:inline-block; font-family:var(--mono); font-size:10px;padding:4px 9px; border-radius:6px; letter-spacing:.1em; text-transform:uppercase; border:1px solid currentColor;}
  .healthy{color:var(--good)} .watch{color:var(--warn)} .critical{color:var(--bad)}
  code{font-family:var(--mono); font-size:12px; color:var(--bert2);background:rgba(25,199,255,.08); padding:1px 6px; border-radius:5px;border:1px solid rgba(25,199,255,.18);}
"#;

fn shell(active: &str, title: &str, eyebrow: &str, blurb: &str, body: &str, side: &str) -> String {
    let nav = [
        ("/", "Overview"),
        ("/ledger-lane", "Ledger Lane"),
        ("/obligation-events", "Obligation Events"),
        ("/verification", "Verification"),
        ("/docs", "Docs"),
    ]
    .iter()
    .map(|(href, label)| {
        let current = if *href == active {
            "style=\"color:#070a0f;background:#37ff8b;border-color:#37ff8b;\""
        } else {
            ""
        };
        format!("<a href=\"{href}\" class=\"meta-chip\" {current}>{label}</a>")
    })
    .collect::<Vec<_>>()
    .join("");

    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\" /><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" /><title>{title}</title><meta name=\"description\" content=\"Rust control plane for obligation ledgers, append-only review events, and renewal-safe legal operations.\" /><style>{STYLE}</style></head><body><div class=\"grid-bg\"></div><div class=\"wrap\"><div class=\"topbar\"><div class=\"left\">Kinetic Gain OS · Rust language atlas lane</div><div class=\"right\"><div>clause-obligation-ledger-rs</div><div>ledger.kineticgain.com</div></div></div><div class=\"herorow\"><section class=\"hero\"><div class=\"eyebrow\">{eyebrow}</div><h1>{title}</h1><p>{blurb}</p><div class=\"chiprow\">{nav}</div></section><aside class=\"side\">{side}</aside></div>{body}</div></body></html>"
    )
}

fn sidecards() -> String {
    [
        ("Primitive split", "The TypeScript clause graph shows discovery and dependency mapping. This Rust surface shows durable obligation-event history and audit posture."),
        ("Ledger focus", "Append-only events, evidence references, acknowledgement state, and renewal-safe review packets."),
        ("Buyer fit", "Legal ops, procurement governance, customer assurance, and executive renewal review teams."),
    ]
    .iter()
    .map(|(label, body)| format!("<div class=\"corr\"><div class=\"lbl\">{label}</div><p>{body}</p></div>"))
    .collect::<Vec<_>>()
    .join("")
}

pub fn render_overview() -> String {
    let payload = sample_payload();
    let rows = payload
        .ledger_lane
        .iter()
        .map(|item| format!("<div class=\"card\"><div class=\"eyebrow\">{}</div><h3>{}</h3><p>{}</p><p><strong>{}</strong> · {} days to renewal window · <span class=\"tag {}\">{}</span></p></div>", item.agreement_id, item.counterparty, item.clause_focus, item.owner, item.renewal_window_days, item.status, item.status))
        .collect::<Vec<_>>()
        .join("");

    let body = format!(
        "<section class=\"section\"><div class=\"sh\"><h2>Overview</h2><div class=\"note\">append-only obligation posture</div></div><div class=\"kpis\"><div class=\"kpi\"><div class=\"v\">{}</div><div class=\"lbl\">Agreements</div><div class=\"h\">Active agreement lanes modeled through the ledger.</div></div><div class=\"kpi\"><div class=\"v\">{}</div><div class=\"lbl\">Active obligations</div><div class=\"h\">Tracked duties with event history and owners.</div></div><div class=\"kpi\"><div class=\"v\">{}</div><div class=\"lbl\">Overdue events</div><div class=\"h\">Acknowledgements or evidence updates already late.</div></div><div class=\"kpi\"><div class=\"v\">{}</div><div class=\"lbl\">Renewal windows</div><div class=\"h\">Open renewal-sensitive packets still needing proof.</div></div><div class=\"kpi\"><div class=\"v\">{}</div><div class=\"lbl\">Evidence gaps</div><div class=\"h\">Material proof holes still visible to reviewers.</div></div></div><div class=\"card\" style=\"margin-top:14px\"><p>{}</p></div></section><section class=\"section\"><div class=\"sh\"><h2>Agreements under obligation pressure</h2><div class=\"note\">ledger lane snapshot</div></div><div class=\"cards\">{}</div></section>",
        payload.summary.agreements,
        payload.summary.active_obligations,
        payload.summary.overdue_events,
        payload.summary.renewal_windows,
        payload.summary.evidence_gaps,
        payload.summary.signal,
        rows
    );

    shell(
        "/",
        "Clause obligation ledger",
        "LegalTech ledger lane",
        "Append-only obligation events, evidence posture, and renewal-safe review visibility in one Rust operator surface.",
        &body,
        &sidecards(),
    )
}

pub fn render_ledger_lane() -> String {
    let rows = sample_payload()
        .ledger_lane
        .iter()
        .map(|item| format!("<tr><td><strong>{}</strong><br />{}<br />{}</td><td>{}</td><td>{}</td><td>{}</td><td><span class=\"tag {}\">{}</span></td><td>{}</td></tr>", item.counterparty, item.agreement_id, item.clause_focus, item.lane, item.owner, item.renewal_window_days, item.status, item.status, item.next_action))
        .collect::<Vec<_>>()
        .join("");

    shell(
        "/ledger-lane",
        "Ledger lane",
        "Agreement posture",
        "This lane turns clause obligations into durable ownership rows with renewal windows, next actions, and status posture.",
        &format!("<section class=\"section\"><table><thead><tr><th>Agreement</th><th>Lane</th><th>Owner</th><th>Renewal days</th><th>Status</th><th>Next action</th></tr></thead><tbody>{rows}</tbody></table></section>"),
        &sidecards(),
    )
}

pub fn render_obligation_events() -> String {
    let rows = sample_payload()
        .obligation_events
        .iter()
        .map(|item| format!("<tr><td><strong>{}</strong><br />{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><span class=\"tag {}\">{}</span></td><td>{}</td></tr>", item.event_id, item.agreement_id, item.event_type, item.actor, item.due_in_days, item.evidence_state, item.status, item.status, item.note))
        .collect::<Vec<_>>()
        .join("");

    shell(
        "/obligation-events",
        "Obligation events",
        "Append-only history",
        "Every state change should be durable enough for audit, review, and renewal committees to replay without guessing what happened.",
        &format!("<section class=\"section\"><table><thead><tr><th>Event</th><th>Type</th><th>Actor</th><th>Due in days</th><th>Evidence</th><th>Status</th><th>Note</th></tr></thead><tbody>{rows}</tbody></table></section>"),
        &sidecards(),
    )
}

pub fn render_verification() -> String {
    let cards = sample_payload()
        .verification
        .iter()
        .map(|packet| format!("<div class=\"card\"><div class=\"eyebrow\">{}</div><h3>{}</h3><p>Completeness: <strong>{}%</strong></p><p>Blocker: {}</p><p><span class=\"tag {}\">{}</span> · {}</p></div>", packet.packet_id, packet.audience, packet.completeness, packet.blocker, packet.status, packet.status, packet.next_action))
        .collect::<Vec<_>>()
        .join("");
    shell(
        "/verification",
        "Verification posture",
        "Review packet readiness",
        "This lane shows whether the obligation ledger is strong enough for sign-off, audit, and renewal decisions without missing evidence context.",
        &format!("<section class=\"section\"><div class=\"cards\">{cards}</div></section>"),
        &sidecards(),
    )
}

pub fn render_docs() -> String {
    shell(
        "/docs",
        "Docs",
        "System docs",
        "The Rust ledger lane complements the clause graph by making state transitions, evidence, and acknowledgements durable enough for downstream review.",
        "<section class=\"section\"><div class=\"card\"><p><strong>Routes:</strong> <code>/</code> · <code>/ledger-lane</code> · <code>/obligation-events</code> · <code>/verification</code> · <code>/docs</code></p><p><strong>APIs:</strong> <code>/api/dashboard/summary</code> · <code>/api/ledger-lane</code> · <code>/api/obligation-events</code> · <code>/api/verification</code> · <code>/api/sample</code></p></div></section>",
        &sidecards(),
    )
}
