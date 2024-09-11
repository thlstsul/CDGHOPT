use std::{collections::HashMap, str::FromStr};

use http::{HeaderName, HeaderValue, Method};
use module::{Message, Request, Response};
use reqwest::{Body, Client, Url};
use snafu::Snafu;
use time::{macros::format_description, OffsetDateTime, UtcOffset};
use tracing::{info, Level};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_web::MakeWebConsoleWriter;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    let offset = UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = OffsetTime::new(
        offset,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );

    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_ansi(true)
        .with_max_level(Level::INFO)
        .with_writer(MakeWebConsoleWriter::new())
        .init();
}

#[wasm_bindgen]
pub async fn listen(message: JsValue) -> Result<JsValue, Error> {
    info!("input: {message:?}");
    route(message).await
}

async fn route(message: JsValue) -> Result<JsValue, Error> {
    let Message { func, param } = serde_wasm_bindgen::from_value(message)?;
    match func.as_str() {
        "http_send" => {
            let req = serde_json::from_str(&param)?;
            let resp = send(req).await?;
            let message = Message::new(func, serde_json::to_string(&resp)?);
            info!("output: {message:?}");
            Ok(serde_wasm_bindgen::to_value(&message)?)
        }
        _ => todo!(),
    }
}

async fn send(
    Request {
        method,
        uri,
        header,
        body,
    }: Request,
) -> Result<Response, Error> {
    let method = Method::from_str(&method)?;
    let url = Url::from_str(&uri)?;
    let mut request = reqwest::Request::new(method, url);
    for (name, value) in header {
        request
            .headers_mut()
            .append(HeaderName::from_str(&name)?, HeaderValue::from_str(&value)?);
    }
    let _ = request.body_mut().insert(Body::from(body));

    let done_date = OffsetDateTime::now_local()?;
    let resp = Client::new().execute(request).await?;
    let elapsed_time = OffsetDateTime::now_local()? - done_date;
    let mut header = HashMap::new();
    for (name, value) in resp.headers() {
        header.insert(name.to_string(), value.as_bytes().to_vec());
    }

    Ok(Response {
        done_date,
        status: resp.status().as_u16(),
        header,
        body: resp.bytes().await?.into(),
        elapsed_time: elapsed_time.whole_milliseconds() as i32,
    })
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(context(false))]
    Message { source: serde_wasm_bindgen::Error },
    #[snafu(context(false))]
    Request { source: serde_json::Error },
    #[snafu(context(false))]
    Method { source: http::method::InvalidMethod },
    #[snafu(context(false))]
    Url { source: url::ParseError },
    #[snafu(context(false))]
    HeaderName {
        source: http::header::InvalidHeaderName,
    },
    #[snafu(context(false))]
    HeaderValue {
        source: http::header::InvalidHeaderValue,
    },
    #[snafu(context(false))]
    Send { source: reqwest::Error },
    #[snafu(context(false))]
    HeaderValueStr { source: http::header::ToStrError },
    #[snafu(context(false))]
    LocalDateTime {
        source: time::error::IndeterminateOffset,
    },
}
