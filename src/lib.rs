mod stuff;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum_extra::extract::Query;
use futures::future::{join_all, JoinAll};
use serde::Deserialize;

pub async fn hello_world() -> &'static str {
    "Hello from a non-standard package layout"
}

pub async fn updates(Path(id): Path<Vec<i32>>) -> impl IntoResponse {
    let string = format!("Hello, world {:?}!", id);
    (StatusCode::OK, string)
}
#[derive(Deserialize, Debug)]
pub struct Gitignore {
    pub langs: Vec<String>,
}

pub async fn get_ignore(lang: &str) -> Result<String, ()> {
    let url = format!(
        "https://raw.githubusercontent.com/github/gitignore/master/{}.gitignore",
        lang
    );
    let resp = reqwest::get(&url).await.unwrap();
    match resp.status() {
        reqwest::StatusCode::OK => {
            println!("ok");
            let body = resp.text().await.unwrap();
            println!("body = {:?}", body);
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
            .langs
            .into_iter()
            .map(|lang| closure_replacement(lang))
            .collect::<Vec<_>>(),
    )
    .await.join("\n");
    igs
}

pub fn make_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/ignores", get(get_ignores))
}
