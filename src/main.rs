use askama::Template;
use axum::response::Html;
use axum::{routing::get, Router};
use regex::Regex;
use std::fs::read_to_string;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/condor", get(condor))
        .route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn condor() -> Html<String> {
    let course_catalog_url = "https://bannerweb.oci.emich.edu/pls/banner/bwckschd.p_disp_detail_sched?term_in=202520&crn_in=22222";

    let html = reqwest::get(course_catalog_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let regex = Regex::new(r#"Seats</SPAN></th>\n<td CLASS=\"dddefault\">\d{1,2}</td>\n<td CLASS=\"dddefault\">(?<actual>\d{1,2})</td>\n<td CLASS=\"dddefault\">(?<remaining>-?\d{1,2})</td>\n</tr>\n<tr>\n<th CLASS=\"ddlabel\" scope=\"row\" ><SPAN class=\"fieldlabeltext\">Waitlist Seats</SPAN></th>\n(<td CLASS=\"dddefault\">\d{1,2}</td>\n){2}<td CLASS=\"dddefault\">(?<waitlist_remaining>-?\d{1,2})</td>"#).unwrap();

    let Some(captures) = regex.captures(&html) else {
        unimplemented!();
    };

    Html(html)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn hello() -> Html<String> {
    Html((HelloTemplate { name: "world" }).render().unwrap())
}
