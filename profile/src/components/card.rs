use crate::icons;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KnownColleges {
    LPNU,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Education {
    pub logo: KnownColleges,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub details: &'static str,
    pub href: &'static str,
}

#[derive(Props, Clone, PartialEq)]
pub struct CardSectionProps {
    pub darkmode: bool,
    pub items: Vec<Education>,
}

#[component]
pub fn CardSection(props: CardSectionProps) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-2 gap-3",
            for item in props.items.iter() {
                Link {
                    to: item.href,
                    div { class: format_args!("p-4 border rounded-lg {}", if props.darkmode { "border-neutral-700" } else { "border-gray-200" }),
                        {match item.logo {
                            KnownColleges::LPNU => rsx! { icons::LPNU { darkmode: props.darkmode, class: "shrink-0 size-10 mb-3" } },
                        }}
                        h3 { class: format_args!("mb-1 text-xs {}", if props.darkmode { "text-neutral-400" } else { "text-gray-600" }),
                            {item.details}
                        }
                        p { class: format_args!("font-semibold text-sm {}", if props.darkmode { "text-neutral-200" } else { "text-gray-800" }),
                            {item.subtitle}
                        }
                        p { class: format_args!("mt-1 text-sm {}", if props.darkmode { "text-neutral-400" } else { "text-gray-600" }),
                            {item.title}
                        }
                    }
                }
            }
        }
    }
}
