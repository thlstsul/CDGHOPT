use leptos::*;

#[component]
pub fn UriInput(value: RwSignal<String>, class: &'static str) -> impl IntoView {
    view! {
        <input
            type="text"
            prop:value=value
            class=class
            on:focusout=move |ev| {
                let input = event_target_value(&ev);
                if !input.is_empty() && !input.starts_with("http") {
                    value.set(format!("http://{}", input));
                }
            }
        />
    }
}
