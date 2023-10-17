pub mod github;
mod stuff;

use std::collections::HashMap;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum_extra::extract::Query;
use futures::future::join_all;
use serde::Deserialize;
use std::path;
use std::pin::pin;
use crate::github::Tree;

const IGNORE_LIST: &str = include_str!("../data/gitignore-tree.json");

pub async fn hello_world() -> &'static str {
    "Hello from a non-standard package layout"
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
            println!("ok");
            let body = resp.text().await.unwrap();
            println!("{}", body);
            Ok(body)
        }
        _ => {
            println!("err {}", resp.status());
            Err(())
        }
    }
}

pub async fn get_ignores(Query(params): Query<Gitignore>) -> impl IntoResponse {
    let igs = fetch_ignores(params).await;

    (StatusCode::OK, igs)
}

async fn closure_replacement(lang: String) -> String {
    match get_ignore(&lang).await {
        Ok(ig) => format!(
            "# Start of .gitignore for {}\n{}\n# End of .gitignore for {}\n",
            lang, ig, lang
        ),
        Err(e) => {
            println!("err = {:?}", e);
            format!("#####\n# Failure finding .gitignore for {}\n####\n", lang)
        }
    }
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
    let igs = join_all(
        params
            .lang
            .into_iter()
            .map(|lang| closure_replacement(lang))
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
}

pub fn available_ignores_from_file() -> Vec<Tree> {
// Use fs_err to read a file from disk and deserialise with serde
    let j: crate::github::Root = serde_json::from_str(IGNORE_LIST).unwrap();
    j.tree
}

pub fn get_matching_ignores(all_ignores: Vec<Tree>, matching: &Vec<String>) -> Vec<String> {
    let lower_map = all_ignores.into_iter().map(|x| (x.path.to_ascii_lowercase().replace(".gitignore", ""), x.path)).collect::<HashMap<String, String>>();
    println!("lower_map = {:?}", lower_map);
    println!("matching = {:?}", matching);
    matching.into_iter().filter_map(|x| {
        match lower_map.get(&*x.to_ascii_lowercase()) {
            Some(m) => Some(m.clone()),
            None => None,
        }
    }).collect::<Vec<_>>()
}
