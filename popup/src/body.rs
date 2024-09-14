use leptos::*;

#[component]
pub fn BodyArea(node_ref: NodeRef<html::Div>, class: &'static str) -> impl IntoView {
    view! { <div node_ref=node_ref class=class contenteditable="true"></div> }
}
