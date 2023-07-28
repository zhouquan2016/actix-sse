use std::time::Duration;

use actix_web::{
    get,
    http::header::ContentType,
    post,
    web::{scope, Data, Json, Query},
    HttpResponse, Scope,
};
use async_stream::stream;
use tokio::{select, time::interval};

use crate::sse::types::{Event, SseData, SseQuery};

use self::types::Message;

pub mod types;

const MAX_TIME: u64 = 360;

#[get("")]
async fn sse(query: Query<SseQuery>, sse_data: Data<SseData>) -> HttpResponse {
    let mut rx = sse_data.0.subscribe();
    let user_id = query.user_id;
    let mut timer = interval(Duration::from_secs(MAX_TIME));
    let rx = stream! {
        timer.tick().await;
        yield Event::new().data("welcome").event("welcome").to_message();
        loop {
            select! {
                m = rx.recv() => {
                    match m {
                        Ok(m) => {
                            if m.to_user_id == user_id {
                                yield Event::new().message(m.message).to_message();
                            }
                        },
                        Err(e) => {
                            println!("user:{}, receive error:{}", user_id.clone(), e);
                            break;
                        }
                    }
                },
                _ = timer.tick() => {
                    log::info!("time out");
                    break;
                }

            }
        }
    };

    HttpResponse::Ok()
        .append_header(ContentType(mime::TEXT_EVENT_STREAM))
        .streaming(rx)
}

#[post("/push")]
async fn push(sse_data: Data<SseData>, ms: Json<Message>) -> String {
    sse_data.send(ms.0.clone())
}
pub fn stage() -> Scope {
    scope("/sse").service(sse).service(push)
}
