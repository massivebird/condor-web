use askama::Template;
use axum::{extract::Query, http::status::StatusCode, response::Html, routing::get, Router};
use condor::CourseStatus;
use std::{collections::HashMap, fs::read_to_string};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/",
            get(|| async { Html(read_to_string("index.html").unwrap()) }),
        )
        .route("/condor", get(get_course))
        .route("/api/get_course", get(get_course))
        .route("/api/sneeze", get(|| async { "achoo" }))
        .route("/form", get(show_form).post(|| async { "duh" }));

    let app = app.fallback(|| async { (StatusCode::NOT_FOUND, "404") });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn get_course(Query(params): Query<HashMap<String, String>>) -> Result<String, ()> {
    let crn = params.get("crn").ok_or(())?;
    let year = params.get("year").ok_or(())?;
    let season = params.get("season").ok_or(())?;

    let semester_code = match season.as_str() {
        "winter" => format!("{year}20"),
        "fall" => format!("{}10", year.parse::<u32>().unwrap() + 1),
        _ => panic!("Unknown season: {season}"),
    };

    let course_status: CourseStatus = condor::get_course_status(crn, &semester_code)
        .await
        .unwrap();

    Ok(course_status.as_json().unwrap())
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
