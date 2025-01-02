use dioxus::prelude::*;

use crate::DarkMode;

#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    pub dark_mode: bool,
    pub copyright: Option<&'static str>,
    pub items: Vec<FooterItem>,
    pub has_theme_toggle: Option<bool>,
    pub has_border: Option<bool>,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FooterItem {
    pub name: &'static str,
    pub href: &'static str,
}


#[component]
pub fn Footer(props: FooterProps) -> Element {
    rsx! {
        footer { class: "w-full max-w-2xl mx-auto px-4 sm:px-6 lg:px-8",
            div { class: format_args!("py-6 {} {}", 
                if props.dark_mode { "border-neutral-700" } else { "border-gray-200" }, 
                if let Some(has_border) = props.has_border { if has_border { "border-t" } else { "" } } else { "" }),
                div { class: "flex flex-wrap justify-between items-center gap-2",
                    div { if let Some(text) = props.copyright {
                            p { class: format_args!("text-xs {}", if props.dark_mode {  "text-neutral-400" } else { "text-gray-600" }),
                                {text}
                            }
                        }
                    }
                    ul { class: "flex flex-wrap items-center",
                        for (i, FooterItem { name, href }) in props.items.iter().enumerate()  {
                            li { class: format_args!("inline-block pe-4 text-xs {} {}", if i != props.items.len() - 1 { "relative last:pe-0 last-of-type:before:hidden before:absolute before:top-1/2 before:end-1.5 before:-translate-y-1/2 before:size-[3px] before:rounded-full" } else { "" }, if props.dark_mode { "text-neutral-500 before:bg-neutral-600" } else { "before:bg-gray-400 text-gray-500" }),
                                a { class: format_args!("text-xs underline hover:decoration-2 focus:outline-none focus:decoration-2 {}", if props.dark_mode { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" }),
                                    href: *href,
                                    {*name}
                                }
                            }
                        }
                        if props.has_theme_toggle.unwrap_or(true) {
                            li { class: "inline-block",
                                ThemeToogle { }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ThemeToogle() -> Element {
    let mut dark_mode = use_context::<Signal<DarkMode>>();

    rsx! {
        button { class: format_args!("relative flex justify-center items-center size-7 border rounded-full {}", if dark_mode().0 { "border-neutral-700 text-neutral-400 hover:bg-neutral-700 focus:bg-neutral-700" } else { "border-gray-200 text-gray-500 hover:bg-gray-200 focus:bg-gray-200" }),
            r#type: "button",
            onclick: move |_| dark_mode.set(DarkMode(!dark_mode().0)),
            span { class: "sr-only", if dark_mode().0 { "Light" } else { "Dark" }  }
            svg {
                view_box: "0 0 24 24",
                width: "24",
                height: "24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "shrink-0 size-3.5",
                if dark_mode().0 {
                    circle { cx: "12", cy: "12", r: "4" }
                    path { d: "M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" }
                } else {
                    path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                }
            }
        }
    }
}