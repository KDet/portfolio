use crate::components::{BookSection, NavBar, NavItem, Biography, UserProfile};
use crate::docs::router_articles::BookRoute as BlogRoute;
use crate::{icons, DarkMode, Route};

use dioxus_use_mounted::use_mounted;
use dioxus::prelude::*;

//const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn BlogPost() -> Element {
    let home_nav: NavItem = NavItem {
        id: "home",
        href: "#home",
        title: "Home",
        section: use_mounted(),
    };
    let articles_nav: NavItem = NavItem {
        id: "articles",
        href: "#articles",
        title: "Articles",
        section: use_mounted(),
    };

    let dark_mode = use_context::<Signal<DarkMode>>();
    let route: Route = use_route();

    rsx! {
        header { class: "top-0 inset-x-0 flex flex-wrap md:justify-start md:flex-nowrap w-full z-50 text-sm sticky",
            NavBar {
                darkmode: dark_mode().0,
                menu: vec![ home_nav, articles_nav],
                brandlink: Route::Home { },
                icon: rsx! {
                    icons::WORK { class: "shrink-0 size-4", darkmode: dark_mode().0 }
                }
            }
        }
        main {
            div { class: "w-full max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 pt-10 md:pt-16",
                div { class: "space-y-5 lg:space-y-8 pb-12 lg:pe-8",
                    onmounted: move |cx| home_nav.section.onmounted(cx),
                    Outlet::<Route> { }   
                }
                div { class: format_args!("mt-8 pb-8 border-b lg:pe-8 {}" , if dark_mode().0 { "border-neutral-700" } else { "border-gray-200" }),
                    UserProfile {
                        darkmode: dark_mode().0,
                        bio: Biography {
                            photo: "https://media.licdn.com/dms/image/v2/C4E03AQHVjVsfWdcStA/profile-displayphoto-shrink_800_800/profile-displayphoto-shrink_800_800/0/1516850185499?e=1740614400&v=beta&t=klwiDgG7VDdiChFBrW7IzDeW-5FztkhEPHqZPAmA4LA",
                            fullname: "Dmytro Kravchyna",
                            title: "Software Developer",
                            about: None,
                            contacts: None
                        }
                    }
                }
                div { class: "mt-5 pb-12",
                    // id: ARTICLES_NAV.id,
                    onmounted: move |cx| articles_nav.section.onmounted(cx),
                    BookSection { 
                        darkmode: dark_mode().0, 
                        routes: BlogRoute::static_routes().into_iter()
                        .filter(|value| match route {
                            Route::BlogPost { child } if child == *value => false,
                            _ => true,
                        }).collect() 
                    },
                }
            }
        }
    }
}
