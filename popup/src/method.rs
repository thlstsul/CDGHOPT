use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn MethodSelect(value: RwSignal<String>, class: &'static str) -> impl IntoView {
    value.set("GET".to_string());
    view! {
        <select
            prop:value=value
            class=class_list!["select", "rounded-none", class]
            on:change=move |ev| {
                let new_value = event_target_value(&ev);
                value.set(new_value);
            }
        >
            <option value="CONNECT">CONNECT</option>
            <option value="DELETE">DELETE</option>
            <option value="GET" selected>
                GET
            </option>
            <option value="HEAD">HEAD</option>
            <option value="OPTIONS">OPTIONS</option>
            <option value="PATCH">PATCH</option>
            <option value="POST">POST</option>
            <option value="PUT">PUT</option>
            <option value="TRACE">TRACE</option>
        </select>
    }
}
