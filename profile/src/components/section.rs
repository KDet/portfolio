use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionProps {
    pub darkmode: bool,
    pub title: Option<&'static str>,
    pub secondary: Option<bool>,
    pub id: Option<&'static str>,
    pub onmounted: Option<EventHandler<Event<MountedData>>>,
    pub children: Element,
}

#[component]
pub fn Section(props: SectionProps) -> Element {
    rsx! {
        section {
            id: props.id.unwrap_or_default(),
            onmounted: move |data| if let Some(mounted) = &props.onmounted { mounted.call(data) },
            div { class: "my-10 sm:my-14",
                if let Some(title) = props.title {
                    h2 { class: format_args!("font-medium {} {}", if props.secondary.unwrap_or(false) { "mb-3" } else { "mb-5" }, if props.darkmode { "text-neutral-200" } else { "text-gray-800" }),
                        {title}
                    }
                }
                {props.children}
            }
        }
    }
}