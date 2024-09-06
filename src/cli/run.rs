// Imports
use super::*;
use crate::card::cards::Cards;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::OnceLock;
use tower_http::{services::ServeDir, trace::TraceLayer};

static HTML_PAGE: OnceLock<&'static str> = OnceLock::new();

pub(super) fn subcommand() -> Command {
    Command::new("run")
}

pub(super) fn process(_arg_matches: &ArgMatches) -> eyre::Result<()> {
    let cards = Cards::load()?;
    let response = cards.generate_static_html_page();
    HTML_PAGE.set(Box::leak(response.into_boxed_str())).unwrap();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = Router::new()
                .layer(TraceLayer::new_for_http())
                .route("/", get(root))
                .nest_service(
                    "/res",
                    ServeDir::new(crate::config::get().target_dir.as_ref().unwrap()),
                );
            tracing::info!("Binding application to port {}", crate::config::get().port);
            let address = format!("0.0.0.0:{}", crate::config::get().port);
            let listener = tokio::net::TcpListener::bind(&address).await?;
            axum::serve(listener, app).await?;
            Ok(())
        })
}

async fn root() -> impl IntoResponse {
    let st: &'static str = HTML_PAGE.get().unwrap();
    Html(st)
}
