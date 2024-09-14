use std::{collections::HashMap, str::FromStr};

use http::{HeaderName, HeaderValue, Method};
use module::{Request, Response};
use reqwest::{Body, Client, Url};
use snafu::Snafu;
use time::{macros::format_description, OffsetDateTime, UtcOffset};
use tracing::{error, info, Level};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_web::MakeWebConsoleWriter;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Function;
use web_extensions::tabs::{self, CreateProperties};
use web_extensions_sys::chrome;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
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

    let on_message =
        { move |request, sender, send_response| on_message(request, sender, send_response) };
    let closure: Closure<dyn Fn(JsValue, JsValue, Function) -> bool> = Closure::new(on_message);
    chrome()
        .runtime()
        .on_message()
        .add_listener(closure.as_ref().unchecked_ref());
    closure.forget();

    let on_clicked = {
        move || {
            wasm_bindgen_futures::spawn_local(async {
                let tab = CreateProperties {
                    url: "index.html",
                    active: true,
                };
                let _ = tabs::create(tab).await;
            })
        }
    };
    let closure: Closure<dyn Fn()> = Closure::new(on_clicked);
    chrome()
        .action()
        .on_clicked()
        .add_listener(closure.as_ref().unchecked_ref());
    closure.forget();
}

fn on_message(request: JsValue, _sender: JsValue, send_response: Function) -> bool {
    info!("input: {request:?}");
    wasm_bindgen_futures::spawn_local(async move {
        let this = JsValue::null();
        match route(request).await {
            Ok(response) => {
                if let Err(e) = send_response.call1(&this, &response) {
                    error!("{e:?}");
                }
            }
            Err(e) => {
                let message = module::Message::new("error".to_string(), e.to_string());
                error!("{message:?}");
                let resp = serde_wasm_bindgen::to_value(&message).unwrap();
                if let Err(e) = send_response.call1(&this, &resp) {
                    error!("{e:?}");
                }
            }
        }
    });
    true
}

async fn route(message: JsValue) -> Result<JsValue, Error> {
    let module::Message { func, param } = serde_wasm_bindgen::from_value(message)?;
    match func.as_str() {
        "http_send" => {
            let req = serde_json::from_str(&param)?;
            let resp = send(req).await?;
            let message = module::Message::new(func, serde_json::to_string(&resp)?);
            info!("output: {message:?}");
            Ok(serde_wasm_bindgen::to_value(&message)?)
        }
        _ => unimplemented!(),
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
    #[snafu(display("Failed to ser/de message: {source}"), context(false))]
    Message { source: serde_wasm_bindgen::Error },
    #[snafu(display("Failed to ser/de request: {source}"), context(false))]
    Request { source: serde_json::Error },
    #[snafu(display("{source}"), context(false))]
    Method { source: http::method::InvalidMethod },
    #[snafu(display("Invalid url: {source}"), context(false))]
    Url { source: url::ParseError },
    #[snafu(display("{source}"), context(false))]
    HeaderName {
        source: http::header::InvalidHeaderName,
    },
    #[snafu(display("{source}"), context(false))]
    HeaderValue {
        source: http::header::InvalidHeaderValue,
    },
    #[snafu(display("Failed to send: {source}",), context(false))]
    Send { source: reqwest::Error },
    #[snafu(
        display("Header value to string error: {source}"),
        context(suffix(false))
    )]
    HeaderValueStr { source: http::header::ToStrError },
    #[snafu(display("{source}"), context(false))]
    LocalDateTime {
        source: time::error::IndeterminateOffset,
    },
}
