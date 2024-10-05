use leptos::ev::Event;
use leptos::*;
use module::http::{Request, Response};
use serde::ser::Serializer as _;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use time::{macros::format_description, OffsetDateTime};
use tracing::error;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys::Object;
use web_sys::js_sys::Map;

use crate::browser::{browser, get_local};

const INDEXES: &str = "indexes";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogIndexItem {
    pub id: Uuid,
    pub method: String,
    pub uri: String,
    #[serde(with = "time::serde::timestamp")]
    pub done_date: OffsetDateTime,
    #[serde(default)]
    pub star: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogContent {
    pub request: Request,
    pub response: Response,
}

#[component]
pub fn LogDrawer(
    indexes: RwSignal<Vec<LogIndexItem>>,
    #[prop(into)] on_select: Callback<LogContent>,
) -> impl IntoView {
    let load_index = create_action(|indexes: &RwSignal<Vec<LogIndexItem>>| load_index(*indexes));
    load_index.dispatch(indexes);

    let get_log = create_action(move |id: &String| {
        let id = id.clone();
        async move {
            let content = get_log(&id).await;
            if let Ok(content) = content {
                on_select.call(content);
            }
        }
    });

    let star = create_action(|index_param: &(RwSignal<Vec<LogIndexItem>>, String)| {
        let indexes = index_param.0;
        let id = index_param.1.clone();
        async move {
            let _ = star(indexes, &id).await.inspect(|e| error!("{e:?}"));
        }
    });

    view! {
        <div class="drawer drawer-end">
            <input id="log-drawer" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content absolute right-0">
                <label for="log-drawer" class="drawer-button btn btn-circle btn-ghost">
                    <span class="loading loading-ring loading-lg"></span>
                </label>
            </div>
            <div class="drawer-side z-50">
                <label for="log-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
                <ul class="menu bg-base-200 text-base-content min-h-full w-1/2">
                    <For
                        each=move || indexes.get().into_iter()
                        key=|index| index.id
                        children=move |index| {
                            view! {
                                <li>
                                    <div class="card bg-base-100 shadow-xl w-full">
                                        <div class="card-body w-full py-0">
                                            <h2 class="card-title">
                                                {index.method}
                                                <div class="badge badge-xs">
                                                    {index
                                                        .done_date
                                                        .format(
                                                            format_description!(
                                                                "[year]-[month]-[day] [hour]:[minute]:[second]"
                                                            ),
                                                        )}
                                                </div>
                                            </h2>
                                            <p class="break-all">{index.uri}</p>
                                        </div>
                                        <div class="card-actions justify-end">
                                            <StarButton
                                                checked=index.star
                                                on_change=move |_| {
                                                    star.dispatch((indexes, index.id.to_string()))
                                                }
                                            />
                                            <div
                                                class="badge badge-primary"
                                                on:click=move |_| get_log.dispatch(index.id.to_string())
                                            >
                                                Open
                                            </div>
                                        </div>
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
        </div>
    }
    .into_view()
}

#[component]
pub fn StarButton(checked: bool, on_change: impl FnMut(Event) + 'static) -> impl IntoView {
    view! {
        <label class="swap swap-rotate">
            <input type="checkbox" checked=checked on:change=on_change />

            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="swap-off h-5 w-5"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M11.48 3.499a.562.562 0 0 1 1.04 0l2.125 5.111a.563.563 0 0 0 .475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 0 0-.182.557l1.285 5.385a.562.562 0 0 1-.84.61l-4.725-2.885a.562.562 0 0 0-.586 0L6.982 20.54a.562.562 0 0 1-.84-.61l1.285-5.386a.562.562 0 0 0-.182-.557l-4.204-3.602a.562.562 0 0 1 .321-.988l5.518-.442a.563.563 0 0 0 .475-.345L11.48 3.5Z"
                />
            </svg>

            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="swap-on h-5 w-5 fill-current"
            >
                <path
                    fill-rule="evenodd"
                    d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.006 5.404.434c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.434 2.082-5.005Z"
                    clip-rule="evenodd"
                />
            </svg>
        </label>
    }
}

async fn load_index(indexes: RwSignal<Vec<LogIndexItem>>) {
    let index_items = get_local(INDEXES)
        .await
        .inspect_err(|e| error!("{e:?}"))
        .unwrap_or_default();
    indexes.set(index_items);
}

async fn get_log(id: &str) -> Result<LogContent, JsValue> {
    get_local(id).await
}

pub async fn save_log(
    indexes: RwSignal<Vec<LogIndexItem>>,
    request: Request,
    response: Response,
) -> Result<(), JsValue> {
    let id = Uuid::now_v7();
    let index = LogIndexItem {
        id,
        method: request.method.clone(),
        uri: request.uri.clone(),
        done_date: response.done_date,
        star: false,
    };
    let content = LogContent { request, response };
    indexes.update(|indexes| {
        let mut i = indexes.len();
        for (ii, indexed) in indexes.iter().enumerate() {
            if !indexed.star {
                i = ii;
                break;
            }
        }
        indexes.insert(i, index);
    });

    let serializer = Serializer::json_compatible();
    let index_key = JsValue::from_str(INDEXES);
    let index_value = serializer.serialize_some(&indexes.get())?;
    let content_key = JsValue::from_str(&id.to_string());
    let content_value = serializer.serialize_some(&content)?;
    let items = Map::new()
        .set(&index_key, &index_value)
        .set(&content_key, &content_value);
    let items = Object::from_entries(&items);
    if let Ok(ref items) = items {
        browser().storage().local().set(items).await?;
    }

    Ok(())
}

pub async fn star(indexes: RwSignal<Vec<LogIndexItem>>, id: &str) -> Result<(), JsValue> {
    indexes.update(|indexes| {
        let i = indexes
            .iter()
            .enumerate()
            .filter_map(|(i, index)| {
                if id == index.id.to_string() {
                    Some(i)
                } else {
                    None
                }
            })
            .next();

        if let Some(i) = i {
            let mut index = indexes.remove(i);
            if index.star {
                index.star = false;
                indexes.push(index);
            } else {
                index.star = true;
                indexes.insert(0, index);
            }
        }
    });

    let serializer = Serializer::json_compatible();
    let index_key = JsValue::from_str(INDEXES);
    let index_value = serializer.serialize_some(&indexes.get())?;
    let items = Map::new().set(&index_key, &index_value);
    let items = Object::from_entries(&items);
    if let Ok(ref items) = items {
        browser().storage().local().set(items).await?;
    }

    Ok(())
}
