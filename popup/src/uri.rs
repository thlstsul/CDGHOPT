use leptos::*;

#[component]
pub fn UrlInput(node_ref: NodeRef<html::Input>, class: &'static str) -> impl IntoView {
    view! { <input type="text" node_ref=node_ref class=class /> }
}
