use leptos::*;

#[component]
pub fn HeaderTable(rows: RwSignal<Vec<(String, String)>>, class: &'static str) -> impl IntoView {
    view! {
        <table class=class>
            <tbody>
                <For
                    each=move || rows.get().into_iter().enumerate()
                    key=|(index, state)| format!("{}-{}", index, state.0)
                    children=move |(index, _)| {
                        let row: Memo<(String, String)> = create_memo(move |_| {
                            rows.get().get(index).unwrap().clone()
                        });
                        let is_last = index == rows.get().len() - 1;
                        view! {
                            // TODO valid header
                            <tr>
                                <td>
                                    <input
                                        type="text"
                                        placeholder="Header name..."
                                        prop:value=row.get().0
                                        on:focusout=move |ev| {
                                            rows.update(|rows| {
                                                let row = rows.get_mut(index).unwrap();
                                                let value = event_target_value(&ev);
                                                let no_empty = !value.is_empty();
                                                row.0 = value;
                                                if is_last && no_empty {
                                                    rows.push((String::new(), String::new()));
                                                }
                                            });
                                        }
                                        class="input input-sm rounded-none w-full"
                                    />
                                </td>
                                <td>
                                    <input
                                        type="text"
                                        placeholder="value..."
                                        prop:value=row.get().1
                                        on:focusout=move |ev| {
                                            rows.update(|rows| {
                                                let row = rows.get_mut(index).unwrap();
                                                row.1 = event_target_value(&ev);
                                            });
                                        }
                                        class="input input-sm rounded-none w-full"
                                    />
                                </td>
                                <th class="h-6 w-6">
                                    <Show when=move || !is_last fallback=|| view! {}>
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 24 24"
                                            fill="currentColor"
                                            class="size-6"
                                            on:click=move |_| {
                                                rows.update(|rows| {
                                                    rows.remove(index);
                                                });
                                            }
                                        >
                                            <path
                                                fill-rule="evenodd"
                                                d="M16.5 4.478v.227a48.816 48.816 0 0 1 3.878.512.75.75 0 1 1-.256 1.478l-.209-.035-1.005 13.07a3 3 0 0 1-2.991 2.77H8.084a3 3 0 0 1-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 0 1-.256-1.478A48.567 48.567 0 0 1 7.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 0 1 3.369 0c1.603.051 2.815 1.387 2.815 2.951Zm-6.136-1.452a51.196 51.196 0 0 1 3.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 0 0-6 0v-.113c0-.794.609-1.428 1.364-1.452Zm-.355 5.945a.75.75 0 1 0-1.5.058l.347 9a.75.75 0 1 0 1.499-.058l-.346-9Zm5.48.058a.75.75 0 1 0-1.498-.058l-.347 9a.75.75 0 0 0 1.5.058l.345-9Z"
                                                clip-rule="evenodd"
                                            />
                                        </svg>
                                    </Show>
                                </th>
                            </tr>
                        }
                    }
                />
            </tbody>
        </table>
    }
}
