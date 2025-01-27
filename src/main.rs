use askama::Template;
use axum::response::Html;
use axum::{routing::get, Router};
use condor::CourseStatus;
use regex::Regex;
use std::fs::read_to_string;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Html(read_to_string("index.html").unwrap()) }))
        .route("/condor", get(condor))
        .route("/cont", get(cont))
        .route("/hello", get(hello))
        .route("/api/sneeze", get(|| async { "achoo" }))
        .route("/form", get(show_form).post(|| async { "duh" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn condor() -> Html<String> {
    let course_status: CourseStatus = condor::get_course_status("22222", "202520").await.unwrap();
    Html(format!("{course_status:?}"))
}

async fn cont() -> Html<String> {
    Html("brew".to_string())
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn hello() -> Html<String> {
    Html((HelloTemplate { name: "world" }).render().unwrap())
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/form" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}
