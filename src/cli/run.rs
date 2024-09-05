use std::sync::OnceLock;

use axum::{
    body::Body,
    http::header::{CONTENT_LENGTH, CONTENT_TYPE},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::card::{cards::Cards, Card};

use super::*;

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
                .route("/", get(one))
                .nest_service(
                    "/res/",
                    ServeDir::new(crate::config::get().target_dir.as_ref().unwrap()),
                );
            // run our app with hyper, listening globally on port 3000
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    Ok(())
}

async fn one() -> impl IntoResponse {
    let st: &'static str = HTML_PAGE.get().unwrap();
    Html(st)
}
