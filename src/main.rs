// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::State, response::Html, routing::get, Json, Router};
use clause_obligation_ledger_rs::{
    core::sample_payload,
    render::{render_docs, render_ledger_lane, render_obligation_events, render_overview, render_verification},
};
use std::{net::SocketAddr, sync::Arc};

#[derive(Clone)]
struct AppState(Arc<clause_obligation_ledger_rs::core::Payload>);

#[tokio::main]
async fn main() {
    let state = AppState(Arc::new(sample_payload()));
    let app = Router::new()
        .route("/", get(|| async { Html(render_overview()) }))
        .route("/ledger-lane", get(|| async { Html(render_ledger_lane()) }))
        .route("/obligation-events", get(|| async { Html(render_obligation_events()) }))
        .route("/verification", get(|| async { Html(render_verification()) }))
        .route("/docs", get(|| async { Html(render_docs()) }))
        .route("/api/dashboard/summary", get(summary))
        .route("/api/ledger-lane", get(ledger_lane))
        .route("/api/obligation-events", get(obligation_events))
        .route("/api/verification", get(verification))
        .route("/api/sample", get(sample))
        .with_state(state);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(5532);
    let addr: SocketAddr = format!("{host}:{port}").parse().expect("valid bind address");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app).await.expect("server");
}

async fn summary(State(state): State<AppState>) -> Json<clause_obligation_ledger_rs::core::Summary> {
    Json(state.0.summary.clone())
}

async fn ledger_lane(State(state): State<AppState>) -> Json<Vec<clause_obligation_ledger_rs::core::LedgerRecord>> {
    Json(state.0.ledger_lane.clone())
}

async fn obligation_events(State(state): State<AppState>) -> Json<Vec<clause_obligation_ledger_rs::core::ObligationEvent>> {
    Json(state.0.obligation_events.clone())
}

async fn verification(State(state): State<AppState>) -> Json<Vec<clause_obligation_ledger_rs::core::VerificationGate>> {
    Json(state.0.verification.clone())
}

async fn sample(State(state): State<AppState>) -> Json<clause_obligation_ledger_rs::core::Payload> {
    Json((*state.0).clone())
}
