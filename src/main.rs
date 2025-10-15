mod components;

mod pages;

use dioxus::prelude::*;

use pages::edit::Edit;
use pages::login::Login;
// use pages::month::Month;
use pages::signup::Signup;
use pages::u::U;

pub mod backend;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/signup")]
    Signup {},
    #[route("/login")]
    Login {},
    #[route("/u/:id")]
    U { id: u32 },
    // #[route("/u/:id/month")]
    // Month { id: u32 },
    #[route("/u/:id/edit")]
    Edit { id: u32 },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const EBOOK_CSS: Asset = asset!("/assets/ebook.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: EBOOK_CSS }
        meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }
        Router::<Route> {}
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        h1 { "404" }
        Link { to: Route::Home {}, class: "",
            h1 { class: "is-bordered", "Home" }
        }
    }
}

#[component]
pub fn Intro() -> Element {
    rsx! {

        h1 { "Habits" }
        h4 {
            Link { to: Route::Signup {}, "Signup" }
        }
        h4 {
            Link { class: "is-bordered", to: Route::Login {}, "Login" }
        }
    }
}

pub fn disable(i: bool) -> &'static str {
    if !i {
        "is-disabled"
    } else {
        ""
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Intro {}
    }
}

// const DB: Arc<RwLock<Data>> =
