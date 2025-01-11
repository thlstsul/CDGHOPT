use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn UriInput(value: RwSignal<String>, class: &'static str) -> impl IntoView {
    view! {
        <input
            type="text"
            prop:value=value
            class=class_list!["input", "rounded-none", class]
            on:focusout=move |ev| {
                let input = event_target_value(&ev);
                if !input.is_empty() && !input.starts_with("http") {
                    value.set(format!("http://{}", input));
                } else {
                    value.set(input);
                }
            }
        />
    }
}
