use leptos::*;

#[component]
pub fn MethodSelect(node_ref: NodeRef<html::Select>, class: &'static str) -> impl IntoView {
    view! {
        <select node_ref=node_ref class=class>
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
