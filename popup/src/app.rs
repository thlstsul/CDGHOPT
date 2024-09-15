use std::str::FromStr;

use crate::body::BodyArea;
use crate::header::HeaderTable;
use crate::method::MethodSelect;
use crate::response::Response;
use crate::send::SendButton;
use crate::uri::UrlInput;
use backon::{ConstantBuilder, Retryable};
use leptos::ev::MouseEvent;
use leptos::*;
use module::{self, Message, Request};

use snafu::Snafu;
use web_extensions_sys::chrome;

fn serde_error(e: impl std::error::Error) -> Error {
    Error::Serialize { src: e.to_string() }
}

async fn send_message(msg: &Message) -> Result<Message, Error> {
    let msg = serde_wasm_bindgen::to_value(msg).map_err(serde_error)?;
    chrome()
        .runtime()
        .send_message(None, &msg, None)
        .await
        .map_err(|e| Error::Send {
            src: format!("{e:?}"),
        })
        .and_then(|v| serde_wasm_bindgen::from_value::<Message>(v).map_err(serde_error))
}

async fn http_send(req: Request) -> Result<Response, Error> {
    let _ = http::Uri::from_str(&req.uri).map_err(|e| Error::Uri { src: e.to_string() })?;
    let param = serde_json::to_string(&req).map_err(serde_error)?;
    let msg = Message::new("http_send".to_string(), param);

    let send = || async { send_message(&msg).await };

    send.retry(ConstantBuilder::default().with_max_times(1))
        .await
        .and_then(|msg| {
            if "error" == msg.func {
                Err(Error::Send { src: msg.param })
            } else {
                serde_json::from_str::<module::Response>(&msg.param).map_err(serde_error)
            }
        })
        .map(|resp| resp.into())
}

#[component]
pub fn App() -> impl IntoView {
    let method_element: NodeRef<html::Select> = create_node_ref();
    let uri_element: NodeRef<html::Input> = create_node_ref();
    let body_element: NodeRef<html::Div> = create_node_ref();
    let headers = create_rw_signal(vec![("".to_string(), "".to_string())]);
    let http_send = create_action(|req: &Request| {
        let req = req.clone();
        http_send(req)
    });
    let pending = http_send.pending();
    let resp = http_send.value();

    let on_submit = move |_ev: MouseEvent| {
        let uri = uri_element
            .get()
            .expect("<UriInput> should be mounted")
            .value();
        let method = method_element
            .get()
            .expect("<MethodSelect> should be mounted")
            .value();
        let body = body_element
            .get()
            .expect("<BodyArea> should be mounted")
            .text_content()
            .unwrap_or_default();

        let request = Request::new(method, uri, headers.get(), body.into_bytes());
        http_send.dispatch(request);
    };

    view! {
        <div class="grid grid-cols-2 gap-4">
            <div class="p-4 min-h-screen">
                <div class="join join-vertical rounded-none h-full w-full">
                    <div class="join rounded-none w-full join-item">
                        <MethodSelect
                            node_ref=method_element
                            class="select rounded-none join-item"
                        />
                        <UrlInput
                            node_ref=uri_element
                            class="input rounded-none w-full join-item"
                        />
                        <SendButton on:click=on_submit class="btn btn-active join-item" />
                    </div>
                    <div class="divider"></div>
                    <HeaderTable rows=headers class="w-full join-item" />
                    <div class="divider"></div>
                    <BodyArea
                        node_ref=body_element
                        class="textarea rounded-none h-full w-full join-item"
                    />
                </div>
            </div>
            <Show
                when=move || { !pending.get() }
                fallback=|| {
                    view! {
                        <div class="flex min-h-screen">
                            <div class="m-auto">
                                <span class="loading loading-infinity loading-lg"></span>
                            </div>
                        </div>
                    }
                }
            >
                <div class="p-4">
                    <ErrorBoundary fallback=|errors| {
                        view! {
                            <div role="alert" class="alert alert-error">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-6 w-6 shrink-0 stroke-current"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    />
                                </svg>
                                <span>
                                    {move || {
                                        errors
                                            .get()
                                            .into_iter()
                                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                            .collect_view()
                                    }}
                                </span>
                            </div>
                        }
                    }>{resp}</ErrorBoundary>
                </div>
            </Show>
        </div>
    }
}

#[derive(Debug, Clone, Snafu)]
enum Error {
    #[snafu(display("Failed to send: {src}",), context(suffix(false)))]
    Send { src: String },
    #[snafu(display("{src}"), context(suffix(false)))]
    Uri { src: String },
    #[snafu(display("{src}"), context(suffix(false)))]
    Serialize { src: String },
}
