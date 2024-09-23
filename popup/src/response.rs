use std::collections::HashMap;

use http::{HeaderValue, StatusCode};
use leptos::*;
use time::{macros::format_description, OffsetDateTime};

#[derive(Debug, Clone)]
pub struct Response {
    pub done_date: OffsetDateTime,
    pub status: StatusCode,
    pub header: HashMap<String, String>,
    pub body: Vec<u8>,
    pub elapsed_time: i32,
}

impl From<module::http::Response> for Response {
    fn from(val: module::http::Response) -> Self {
        let module::http::Response {
            done_date,
            status,
            header,
            body,
            elapsed_time,
        } = val;

        let mut head_map = HashMap::new();
        for (name, value) in header.into_iter() {
            let value_str = HeaderValue::from_bytes(&value)
                .unwrap()
                .to_str()
                .unwrap_or("no visible")
                .to_string();
            // TODO base 64
            head_map.insert(name, value_str);
        }

        Response {
            done_date,
            status: StatusCode::from_u16(status).unwrap_or_default(),
            header: head_map,
            body,
            elapsed_time,
        }
    }
}

impl IntoView for Response {
    fn into_view(self) -> View {
        // TODO parse
        let body = String::from_utf8(self.body);
        view! {
            <Stat status=self.status elapsed_time=self.elapsed_time done_date=self.done_date />
            <div class="divider h-0"></div>
            <Header header=self.header />
            <div class="divider h-0"></div>
            <pre class="p-4 rounded-md w-full overflow-x-auto">
                <code>{body}</code>
            </pre>
        }
        .into_view()
    }
}

#[component]
fn Header(header: HashMap<String, String>) -> impl IntoView {
    view! {
        <div class="collapse collapse-arrow">
            <input type="checkbox" />
            <div class="collapse-title text-xl font-medium">Headers</div>
            <div class="collapse-content">
                <table class="table table-xs w-full">
                    <tbody>
                        {header
                            .into_iter()
                            .map(|h| {
                                view! {
                                    <tr>
                                        <td>{h.0}</td>
                                        <td>{h.1}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn Stat(status: StatusCode, elapsed_time: i32, done_date: OffsetDateTime) -> impl IntoView {
    let (color_class, status_icon) = if status.is_success() {
        ("text-success",
            "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z")
    } else if status.is_client_error() || status.is_server_error() {
        ("text-error",
            "M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25Zm-1.72 6.97a.75.75 0 1 0-1.06 1.06L10.94 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06L12 13.06l1.72 1.72a.75.75 0 1 0 1.06-1.06L13.06 12l1.72-1.72a.75.75 0 1 0-1.06-1.06L12 10.94l-1.72-1.72Z")
    } else {
        ("text-warning",
            "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm8.706-1.442c1.146-.573 2.437.463 2.126 1.706l-.709 2.836.042-.02a.75.75 0 0 1 .67 1.34l-.04.022c-1.147.573-2.438-.463-2.127-1.706l.71-2.836-.042.02a.75.75 0 1 1-.671-1.34l.041-.022ZM12 9a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z")
    };
    let status_code = status.as_u16();
    let reason = status.canonical_reason().unwrap_or_default().to_string();

    let time = done_date.format(format_description!("[hour]:[minute]:[second]"));
    let date = done_date.format(format_description!("[year]-[month]-[day]"));
    view! {
        <div class="stats shadow w-full">
            <div class="stat py-0">
                <div class=move || format!("stat-figure {color_class}")>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 24 24"
                        class="h-5 w-5"
                    >
                        <path fill-rule="evenodd" clip-rule="evenodd" d=status_icon></path>
                    </svg>
                </div>
                <div class="stat-title">Status</div>
                <div class=move || {
                    format!("stat-value text-2xl {color_class}")
                }>{status_code}</div>
                <div class="stat-desc">{reason}</div>
            </div>

            <div class="stat py-0">
                <div class="stat-figure">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 24 24"
                        class="h-5 w-5"
                    >
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25ZM12.75 6a.75.75 0 0 0-1.5 0v6c0 .414.336.75.75.75h4.5a.75.75 0 0 0 0-1.5h-3.75V6Z"
                        ></path>
                    </svg>
                </div>
                <div class="stat-title">Elapsed</div>
                <div class="stat-value text-2xl">{elapsed_time}</div>
                <div class="stat-desc">ms</div>
            </div>

            <div class="stat py-0">
                <div class="stat-figure">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 24 24"
                        class="h-5 w-5"
                    >
                        <path d="M12.75 12.75a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM7.5 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM8.25 17.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM9.75 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM10.5 17.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM12 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM12.75 17.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM14.25 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM15 17.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM16.5 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM15 12.75a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM16.5 13.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"></path>
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M6.75 2.25A.75.75 0 0 1 7.5 3v1.5h9V3A.75.75 0 0 1 18 3v1.5h.75a3 3 0 0 1 3 3v11.25a3 3 0 0 1-3 3H5.25a3 3 0 0 1-3-3V7.5a3 3 0 0 1 3-3H6V3a.75.75 0 0 1 .75-.75Zm13.5 9a1.5 1.5 0 0 0-1.5-1.5H5.25a1.5 1.5 0 0 0-1.5 1.5v7.5a1.5 1.5 0 0 0 1.5 1.5h13.5a1.5 1.5 0 0 0 1.5-1.5v-7.5Z"
                        ></path>
                    </svg>
                </div>
                <div class="stat-title">At</div>
                <div class="stat-value text-2xl">{time}</div>
                <div class="stat-desc">{date}</div>
            </div>
        </div>
    }
}
