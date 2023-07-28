use actix_web::web::{Bytes, Data};
use serde::Deserialize;
use tokio::sync::broadcast::{Sender, channel};

pub struct Event {
    id: Option<String>,
    event: Option<String>,
    data: Option<String>,
}

impl Event {

    pub fn new() -> Self {
        Self {
            id: None,
            event: None,
            data: None,
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn message(self, ms: String) -> Self {
        self.event("message").data(&ms)
    }

    pub fn ping(self) -> Self {
        self.event("ping").data("ok")
    }

    pub fn event(mut self, event: &str) -> Self {
        self.event = Some(event.to_string());
        self
    }

    pub fn data(mut self, data: &str) -> Self {
        self.data = Some(data.to_string());
        self
    }

    pub fn to_message(&self) -> Result<Bytes, String> {
        let mut out = String::from("");
        if let Some(id) = &self.id {
            out = format!("{}id: {}\n", out, id);
        }
        if let Some(event) = &self.event {
            out = format!("{}event: {}\n", out, event);
        }
        if let Some(data) = &self.data {
            out = format!("{}data: {}\n\n", out, data);
        }
        Ok(Bytes::from(out))
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        let mut out = String::from("");
        if let Some(id) = &self.id {
            out = format!("{}id: {}\n", out, id);
        }
        if let Some(event) = &self.event {
            out = format!("{}event: {}\n", out, event);
        }
        if let Some(data) = &self.data {
            out = format!("{}data: {}\n\n", out, data);
        }
        out
    }
}

#[macro_export]
macro_rules! err_format {
    ($name: expr) => {
        format!("{}:{},error:{}", file!(), line!(), $name)
    };
}

#[derive(Clone, Deserialize)]
pub struct SseQuery {
    pub user_id: i64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Message {
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub message: String,
}
pub struct SseData(pub Sender<Message>);

impl SseData {

    pub fn create() -> Data<Self>{
        let (tx, _) = channel::<Message>(1024);
        let data = Data::new(SseData(tx.clone()));
        data
    }
    pub fn send(&self, ms: Message) -> String {
        log::info!("send a new message:{:?}", ms);
        let tx = &self.0;
        match tx.send(ms) {
            Ok(size) => {
                format!("Ok.send count:{}.channel count:{}", size, tx.receiver_count())
            },
            Err(e) => err_format!(e)
        }
    }
}
