use crate::header::HeaderTable;
use crate::log::{save_log, LogContent, LogDrawer, LogIndexItem};
use crate::method::MethodSelect;
use crate::send::SendButton;
use crate::uri::UriInput;
use crate::{body::BodyArea, browser::browser};
use backon::{ConstantBuilder, Retryable};
use leptos::*;
use module::{
    http::{self, Request, Response},
    Message,
};

use snafu::Snafu;
use tracing::error;

#[component]
pub fn App() -> impl IntoView {
    let log_indexes: RwSignal<Vec<LogIndexItem>> = create_rw_signal(Vec::new());
    let method_value = create_rw_signal("".to_string());
    let uri_value = create_rw_signal("".to_string());
    let body_element: NodeRef<html::Div> = create_node_ref();
    let body_value = create_rw_signal("".to_string());
    let header_value = create_rw_signal(vec![("".to_string(), "".to_string())]);
    let body_editable = move || {
        let method = method_value.get();
        "PATCH" == method || "POST" == method || "PUT" == method
    };

    let http_send = create_action(|req_param: &(Request, RwSignal<Vec<LogIndexItem>>)| {
        let req = req_param.0.clone();
        let log_indexes = req_param.1;
        async move {
            let resp = http_send(req.clone()).await?;

            let _ = save_log(log_indexes, req, resp.clone())
                .await
                .inspect_err(|e| error!("{e:?}"));
            Ok::<crate::response::Response, Error>(resp.into())
        }
    });
    let pending = http_send.pending();
    let resp = http_send.value();

    let on_submit = move |_| {
        let uri = uri_value.get();
        let method = method_value.get();
        let body = body_element
            .get()
            .expect("<BodyArea> should be mounted")
            .text_content()
            .unwrap_or_default();

        let request = Request::new(method, uri, header_value.get(), body.into_bytes());
        http_send.dispatch((request, log_indexes));
    };

    view! {
        <LogDrawer
            indexes=log_indexes
            on_select=move |content: LogContent| {
                let LogContent { request: Request { method, uri, header, body }, response } = content;
                method_value.set(method);
                uri_value.set(uri);
                let mut header: Vec<(String, String)> = header.into_iter().collect();
                header.push(("".to_string(), "".to_string()));
                header_value.set(header);
                body_value.set(String::from_utf8(body).unwrap_or_default());
                resp.set(Some(Ok(response.into())));
            }
        />
        <div class="grid grid-cols-2 gap-4">
            <div class="p-4 min-h-screen">
                <div class="join join-vertical rounded-none h-full w-full">
                    <div class="join rounded-none w-full join-item">
                        <MethodSelect value=method_value class="select rounded-none join-item" />
                        <UriInput value=uri_value class="input rounded-none w-full join-item" />
                        <SendButton on:click=on_submit class="btn btn-active join-item" />
                    </div>
                    <div class="divider"></div>
                    <HeaderTable rows=header_value class="w-full join-item" />
                    <div class="divider"></div>
                    <BodyArea
                        node_ref=body_element
                        value=body_value
                        contenteditable=body_editable
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

async fn http_send(req: Request) -> Result<Response, Error> {
    let msg = req.try_into().map_err(serde_error)?;

    send_message(&msg)
        .await
        .and_then(|msg| serde_json::from_str::<http::Response>(&msg.value).map_err(serde_error))
}

async fn send_message(msg: &Message) -> Result<Message, Error> {
    let msg = serde_wasm_bindgen::to_value(msg).map_err(serde_error)?;

    let send = || async {
        browser()
            .runtime()
            .send_message(None, &msg, None)
            .await
            .map_err(|e| Error::Send {
                src: format!("{e:?}"),
            })
            .and_then(|v| serde_wasm_bindgen::from_value::<Message>(v).map_err(serde_error))
    };

    send.retry(ConstantBuilder::default().with_max_times(1))
        .await
        .and_then(|msg| {
            if "error" == msg.code {
                Err(Error::Send { src: msg.value })
            } else {
                Ok(msg)
            }
        })
}

fn serde_error(e: impl std::error::Error) -> Error {
    Error::Serialize { src: e.to_string() }
}

#[derive(Debug, Clone, Snafu)]
enum Error {
    #[snafu(display("Failed to send: {src}"), context(suffix(false)))]
    Send { src: String },
    #[snafu(display("{src}"), context(suffix(false)))]
    Serialize { src: String },
}
