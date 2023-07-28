use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, middleware::Logger};
use env_logger::Env;
use sse::types::SseData;
extern crate env_logger;
mod sse;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_HTML_UTF_8))
        .body(include_str!("../static/html/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // spawn
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // debug!("");
    let sse_data = SseData::create();
    HttpServer::new(move || {
        App::new()
             .wrap(Logger::default())
            .service(index)
            .service(sse::stage())
            .app_data(sse_data.clone())
    })
    .workers(10)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
