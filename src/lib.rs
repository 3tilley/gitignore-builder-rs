pub mod github;
mod stuff;
pub mod telemetry;

use axum::body::HttpBody;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum_extra::extract::Query;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use futures::future;
use futures::future::join_all;
use futures::future::FutureExt;
use serde::Deserialize;
use std::collections::HashMap;
use std::future::Future;
use tracing::info;

use crate::github::Tree;

const IGNORE_LIST: &str = include_str!("../data/gitignore-tree.json");

pub async fn hello_world() -> &'static str {
    "Welcome to the gitignore builder service"
}

pub async fn updates(Path(id): Path<Vec<i32>>) -> impl IntoResponse {
    let string = format!("Hello, world {:?}!", id);
    (StatusCode::OK, string)
}
#[derive(Deserialize, Debug)]
pub struct Gitignore {
    pub lang: Vec<String>,
}

pub async fn get_ignore(lang: &str) -> Result<String, ()> {
    let url = format!(
        "https://raw.githubusercontent.com/github/gitignore/master/{}",
        lang
    );
    let resp = reqwest::get(&url).await.unwrap();
    match resp.status() {
        reqwest::StatusCode::OK => {
            let body = resp.text().await.unwrap();
            Ok(body)
        }
        _ => Err(()),
    }
}

pub async fn get_ignores(Query(params): Query<Gitignore>) -> impl IntoResponse {
    let igs = fetch_ignores(params).await;

    (StatusCode::OK, igs)
}

async fn turn_lang_to_gitignore_block(lang: String) -> String {
    match get_ignore(&lang).await {
        Ok(ig) => format!(
            "# Start of .gitignore for {}\n{}\n# End of .gitignore for {}\n",
            lang, ig, lang
        ),
        Err(e) => {
            tracing::error!("err = {:?}", e);
            format!("#####\n# Failure fetching .gitignore for {}\n####\n", lang)
        }
    }
}

async fn err_lang(lang: String) -> String {
    tracing::error!("Couldn't find a matching .gitignore for {}", lang);
    format!(
        "#####\n# Couldn't find a matching .gitignore for {}\n####\n",
        lang
    )
    .into()
}

pub async fn fetch_ignores(params: Gitignore) -> String {
    // TODO: Work out why the below doesn't work as a closure
    // let igs = join_all(params.langs.into_iter().map(async move |lang| {
    //     match get_ignore(&lang).await {
    //         Ok(ig) => format!("# Start of .gitignore for {}\n{}\n# End of .gitignore for {}\n", lang, ig, lang),
    //         Err(e) => {
    //             println!("err = {:?}", e);
    //             format!("#####\n# Failure finding .gitignore for {}\n####\n", lang)
    //         }
    //     }
    // }).collect::<Vec<String>>().join("\n"));
    let matching = get_matching_ignores(available_ignores_from_file(), &params.lang);
    let igs = join_all(
        matching
            .into_iter()
            .map(|lang_result| match lang_result {
                Ok(lang) => turn_lang_to_gitignore_block(lang).boxed(),
                Err(lang) => err_lang(lang).boxed(),
            })
            // .collect::<Vec<Box<dyn futures::Future<Output = String>>>>(),
            .collect::<Vec<_>>(),
    )
    .await
    .join("\n");
    igs
}

pub fn make_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/ignores", get(get_ignores))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
}

pub fn available_ignores_from_file() -> Vec<Tree> {
    // Use fs_err to read a file from disk and deserialise with serde
    let j: crate::github::Root = serde_json::from_str(IGNORE_LIST).unwrap();
    j.tree
}

pub fn get_matching_ignores(
    all_ignores: Vec<Tree>,
    matching: &Vec<String>,
) -> Vec<Result<String, String>> {
    let lower_map = all_ignores
        .into_iter()
        .map(|x| {
            (
                x.path.to_ascii_lowercase().replace(".gitignore", ""),
                x.path,
            )
        })
        .collect::<HashMap<String, String>>();
    info!("Matching = {:?}", matching);
    matching
        .into_iter()
        .map(|x| match lower_map.get(&*x.to_ascii_lowercase()) {
            Some(m) => Ok(m.clone()),
            None => Err(x.clone()),
        })
        .collect::<Vec<_>>()
}

pub async fn shutdown_signal() {
    opentelemetry::global::shutdown_tracer_provider();
}
