use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

mod components;
mod models;
mod our_projects;
mod routes;

use components::*;
use our_projects::*;
use routes::*;

/// The organization name *on GitHub*.
pub const ORGANIZATION_NAME: &str = "dioxus-community";
pub const PROJECT_MARKER_FILE_NAME: &str = "Cargo.toml";

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/our-projects")]
        OurProjects {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}
