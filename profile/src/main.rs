use dioxus::prelude::*;
use docs::router_articles::BookRoute;

use views::{BlogPost, Home, Scaffold};

mod components;
mod docs;
mod icons;
mod views;
mod dioxus_spring;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Scaffold)]
    #[nest("/article")]
        #[layout(BlogPost)]
            #[child("")]
            BlogPost { child: BookRoute },
        #[end_layout]
    #[end_nest]

    #[route("/")]
    //  if the current location doesn't match any of the other routes, redirect to "/home"
    #[redirect("/:.._route", |_route: Vec<String>| Route::Home { })]
    Home {}, 
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DarkMode(bool);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));

    rsx! {
        Router::<Route> {}
    }
}
