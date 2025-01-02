use crate::{dioxus_spring::use_spring_signal, icons};
use dioxus::prelude::*;
use dioxus_use_mounted::{use_mounted, UseMounted};
use std::time::Duration;

#[derive(Copy, Clone, PartialEq)]
pub struct NavItem {
    pub id: &'static str,
    pub title: &'static str,
    pub href: &'static str,
    pub section: UseMounted,
}

#[derive(Props, Clone, PartialEq)]
pub struct NavBarProps {
    pub darkmode: bool,
    pub menu: Vec<NavItem>,
    pub selected: Option<usize>,
    #[props(into)]
    pub brandlink: NavigationTarget,
    pub icon: Option<Element>,
}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
    let mut selected_index = use_signal(|| props.selected.unwrap_or(0));

    let mut height = use_signal(|| 0);
    let (spring, spring_ref) = use_spring_signal(height());
    use_memo(move || {
        spring_ref.animate(height(), Duration::from_millis(300));
    });

    let container_ref = use_mounted();

    rsx! {
        nav { class: format_args!("mt-4 relative max-w-2xl w-full mx-2 py-2.5 md:flex md:items-center md:justify-between md:py-0 md:px-4 md:mx-auto border rounded-[2rem] {}", if props.darkmode { "bg-neutral-900 border-neutral-700" } else { "bg-white border-gray-200" }),
            div { class: "px-4 md:px-0 flex justify-between items-center",
                div {  class: "shrink-0",
                    Link { class: "flex-none rounded-md inline-block text-xl font-semibold focus:outline-none focus:opacity-80",
                        to: props.brandlink,
                        if let Some(icon) = props.icon { {icon} } else { icons::AT { darkmode: props.darkmode, class: "mx-2 justify-center" } }
                    }
                }
                div { class: "md:hidden",
                    button { class: format_args!("flex justify-center items-center size-6 border rounded-full focus:outline-none {}", if props.darkmode { "border-neutral-700 text-neutral-400 hover:bg-neutral-700 focus:bg-neutral-700" } else { "border-gray-200 text-gray-500 hover:bg-gray-200 focus:bg-gray-200" }),
                        r#type: "button",
                        onclick: move |_| height.set(if height() > 0 { 0 } else { get_scroll_height(container_ref).unwrap_or(0) }),
                        svg {
                            width: "24",
                            stroke: "currentColor",
                            fill: "none",
                            height: "24",
                            view_box: "0 0 24 24",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            class: "shrink-0 size-3.5",
                            if height() > 0 {
                                path { d: "M18 6 6 18" }
                                path { d: "m6 6 12 12" }
                            } else {
                                line { y2: "6", x2: "21", x1: "3", y1: "6" }
                                line { y1: "12", x1: "3", y2: "12", x2: "21" }
                                line { y1: "18", x2: "21", y2: "18", x1: "3" }
                            }
                        }
                    }
                }
            }
            div { class: format_args!("basis-full grow overflow-hidden md:block {}", if spring() == 0 { "hidden" } else { "" }),
                style: format_args!("{}", if height() != spring() { format!("height: {}px;", spring()) } else { "".into() }),
                onmounted: move |event| container_ref.onmounted(event),
                div { class: "flex flex-col md:flex-row md:items-center md:justify-end gap-2 md:gap-3 mt-3 md:mt-0 py-2 md:py-0 md:ps-7",
                    for (i, &nav) in props.menu.iter().enumerate() {
                        Link { class: format_args!("py-0.5 md:py-3 px-4 md:px-1 border-s-2 md:border-s-0 md:border-b-2 focus:outline-none {} {}", if props.darkmode { "border-neutral-200 text-neutral-200" } else { "border-gray-800 text-gray-800" }, if selected_index() == i { "font-medium" } else { "border-transparent" }),
                            to: nav.href,
                            onclick_only: true,
                            onclick: move |_| {
                                selected_index.set(i);
                                scroll_into_view(nav.section);
                            },
                            {nav.title}
                        }
                    }
                }
            }
        }
    }
}

fn scroll_into_view(mounted: UseMounted) {
    if let Some(element) = &*mounted.signal.read() {
        let _ = element.scroll_to(ScrollBehavior::Smooth);
        // if let Some(raw_elem) = element.downcast::<web_sys::Element>() {
        //     let options = web_sys::ScrollIntoViewOptions::new();
        //     options.set_behavior(web_sys::ScrollBehavior::Smooth);
        //     options.set_block(web_sys::ScrollLogicalPosition::Center);
        //     raw_elem.scroll_into_view_with_scroll_into_view_options(&options);
        // }
    }
}

fn get_scroll_height(mounted: UseMounted) -> Option<i32> {
    if let Some(element) = &*mounted.signal.read() {
        if let Some(raw_elem) = element.downcast::<web_sys::Element>() {
            let att = raw_elem.get_attribute("style").unwrap_or_default();
            raw_elem
                .set_attribute("style", format!("{};display: block;", att).as_str())
                .unwrap_or_default();
            let height = raw_elem.scroll_height();

            raw_elem
                .set_attribute("style", att.as_str())
                .unwrap_or_default();
            return Some(height);
        }
    }
    None
}
