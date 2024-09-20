use leptos::*;

#[component]
pub fn BodyArea(
    node_ref: NodeRef<html::Div>,
    class: &'static str,
    #[prop(into)] contenteditable: Signal<bool>,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=class
            contenteditable=move || format!("{}", contenteditable.get())
        ></div>
    }
}
