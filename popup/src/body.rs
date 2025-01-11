use leptos::{html::Div, prelude::*};
use thaw_utils::class_list;

#[component]
pub fn BodyArea(
    node_ref: NodeRef<Div>,
    class: &'static str,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] contenteditable: Signal<bool>,
) -> impl IntoView {
    view! {
        <div
            class=class_list!["textarea", "rounded-none", class]
            node_ref=node_ref
            contenteditable=move || contenteditable.get().to_string()
        >
            {value}
        </div>
    }
}
