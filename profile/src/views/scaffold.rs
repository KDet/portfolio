use crate::{components::{Footer, FooterItem}, DarkMode, Route};
use dioxus::prelude::*;
use document::{Meta, Script, Stylesheet, Title};

#[component]
pub fn Scaffold() -> Element {
    let dark_mode = use_context::<Signal<DarkMode>>();

    rsx! {
        Title { "Portfolio | Kravchyna Dmytro" }
        Meta {
            name: "description",
            content: "Portfolio | Kravchyna Dmytro.",
        }
        Stylesheet { href: asset!("./assets/tailwind.css") }
        Script {
            r#async: true,
            src: "https://www.googletagmanager.com/ns.html?id=GTM-N5JFCLXB",
        }
        Script {
            r#async: true,
            src: asset!("./assets/gtag.js"),
            r#type: "text/javascript",
        }

        body { class: if dark_mode().0 { "bg-neutral-900" } else { "bg-white" },
            Outlet::<Route> {}
            Footer {
                dark_mode: dark_mode().0,
                copyright: "Special thanks to Dioxus and Preline.",
                has_theme_toggle: true,
                has_border: true,
                items: vec![
                    FooterItem { name: "LinkedIn", href: "https://www.linkedin.com/in/dmytro-kravchyna-111a1181" },
                    FooterItem { name: "Github", href: "https://github.com/KDet/portfolio" },
                ]
            }
        }
    }
}
