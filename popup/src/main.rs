use app::App;
use leptos::*;
use time::macros::format_description;
use time::UtcOffset;
use tracing::Level;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_web::MakeWebConsoleWriter;

mod app;
mod body;
mod header;
mod method;
mod response;
mod send;
mod uri;

fn main() {
    console_error_panic_hook::set_once();
    let offset = UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = OffsetTime::new(
        offset,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );

    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_ansi(true)
        .with_max_level(Level::INFO)
        .with_writer(MakeWebConsoleWriter::new())
        .init();

    mount_to_body(App)
}
