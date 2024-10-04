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

#[component]
pub fn LogDrawer(
    indexes: RwSignal<Vec<LogIndexItem>>,
    #[prop(into)] on_select: Callback<LogContent>,
) -> impl IntoView {
    let load_action = create_action(|indexes: &RwSignal<Vec<LogIndexItem>>| load_index(*indexes));
    load_action.dispatch(indexes);

    let get_log = create_action(move |id: &String| {
        let id = id.clone();
        async move {
            let content = get_log(id).await;
            if let Ok(content) = content {
                on_select.call(content);
            }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogIndexItem {
    pub id: Uuid,
    pub method: String,
    pub uri: String,
    #[serde(with = "time::serde::timestamp")]
    pub done_date: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogContent {
    pub request: Request,
    pub response: Response,
}

async fn load_index(indexes: RwSignal<Vec<LogIndexItem>>) {
    let index_items = get_local(INDEXES)
        .await
        .inspect_err(|e| error!("{e:?}"))
        .unwrap_or_default();
    indexes.set(index_items);
}

async fn get_log(id: String) -> Result<LogContent, JsValue> {
    get_local(&id).await
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
    };
    let content = LogContent { request, response };
    indexes.update(|i| i.insert(0, index));

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
