mod components;

mod pages;

use dioxus::prelude::*;

use pages::edit::Edit;
use pages::errors::NotFoundPage;
use pages::login::Login;
use pages::month::Month;
use pages::signup::Signup;
use pages::u::U;
use components::navbar::Navbar;
pub mod backend;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
   
    #[route("/")]
    Home {},
    #[route("/signup")]
    Signup {},
    #[route("/login")]
    Login {},
   
    #[route("/:..segments")]
    NotFoundPage { segments: Vec<String> },


    
    #[nest("/u/:id")]
        #[layout(Navbar)]


            #[route("/")]
            U { id: u32 },
            #[route("/month")]
            Month { id: u32 },
            #[route("/edit")]
            Edit { id: u32 },

            // #[route("/u/:id")]
            // U { id: u32 },
            // #[route("/u/:id/month")]
            // Month { id: u32 },
            // #[route("/u/:id/edit")]
            // Edit { id: u32 },
}

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const EBOOK_CSS: Asset = asset!("/assets/ebook.css");

fn main() {
    //  #[cfg(not(feature = "server"))]
    // server_fn::client::set_server_url("https://habits.toastxc.xyz");
    
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: EBOOK_CSS }
        meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        Router::<Route> {}
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
