use crate::{docs::router_articles::BookRoute, Route};

use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    pub title: String,
    pub description: String,
    pub subtitle: String,
    pub route: BookRoute,
}

#[derive(Props, Clone, PartialEq)]
pub struct BlogSectionProps {
    pub darkmode: bool,
    pub items: Vec<Note>,
}

#[component]
pub fn BlogSection(props: BlogSectionProps) -> Element {
    rsx! {
        ul { class: "space-y-10",
            for article in props.items.iter() {
                li {
                    p { class: format_args!("mb-2 text-sm {}", if props.darkmode { "text-neutral-500" } else { "text-gray-500" }),
                        {article.subtitle.as_str()}
                    }
                    h5 { class: format_args!("font-medium text-sm {}", if props.darkmode { "text-neutral-200" } else { "text-gray-800" }),
                        {article.title.as_str()}
                    }
                    p { class: format_args!("mt-1 text-sm {}", if props.darkmode { "text-neutral-500" } else { "text-gray-500" }),
                        {article.description.as_str()}
                    }
                    p { class: "mt-1",
                        BlogLink { darkmode: props.darkmode, to: Route::BlogPost { child: article.route }, "Continue reading" }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BookSectionProps {
    pub darkmode: bool,
    pub routes: Vec<BookRoute>,
}

#[component]
pub fn BookSection(props: BookSectionProps) -> Element {
    rsx! {
        BlogSection { darkmode: props.darkmode, items: props.routes.into_iter().rev()
            .filter_map(|route| {
                let title = &route.page().title;
                if title.contains("[draft]") { return None; }
                let items: Vec<&str> = title.splitn(4, " $ ").collect();
                let [title, _category, date, description] = items.as_slice() else { return None; };

                Some(Note {
                    route: route,
                    title: title.to_string(),
                    subtitle: date.to_string(),
                    description: description.to_string(),
                })
            }).collect(),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BlogLinkProps {
    #[props(into)]
    pub to: NavigationTarget,
    pub darkmode: bool,
    pub class: Option<&'static str>,
    pub children: Element,
}

#[component]
pub fn BlogLink(props: BlogLinkProps) -> Element {
    rsx! {
        Link { class: format_args!("text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2 {} {}", 
            if props.darkmode { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" }, 
            if let Some(class) = props.class { class } else {""} ),
            to: props.to,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ParagraphProps {
    pub darkmode: bool,
    pub class: Option<&'static str>,
    pub children: Element,
}
