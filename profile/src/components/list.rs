use crate::icons;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Skill {
    Figma,
    Sketch,
    Dioxus,
    SwiftUI,
    WPF,
    Xamarin,
    Flutter,
    AspNetCore,
    Git,
    YAML,
    Docker,
    Kubernetes,
    Swift,
    Cpp,
    Dart,
    CSharp,
    Rust,
    Name(&'static str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Category {
    pub name: &'static str,
    pub values: Vec<Skill>,
}

#[derive(Props, Clone, PartialEq)]
pub struct DescriptionListProps {
    pub darkmode: bool,
    pub items: Vec<Category>,
}

#[component]
pub fn DescriptionList(props: DescriptionListProps) -> Element {
    rsx! {
        div { class: "space-y-3",
        for skill in props.items.iter() {
            dl { class: "flex flex-col sm:flex-row gap-1",
                dt { class: "min-w-40",
                    span { class: format_args!("block text-sm {}", if props.darkmode { "text-neutral-500" } else { "text-gray-500" }),
                        {format!("{}:", skill.name)}
                    }
                }
                dd {
                    ul {
                        for tool in skill.values.iter() {
                            li { class: format_args!("me-1 after:content-[','] inline-flex items-center text-sm {}", if props.darkmode { "text-neutral-200" } else { "text-gray-800" }),
                                {match tool {
                                    Skill::Figma => rsx! { icons::FIGMA { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Figma" },
                                    Skill::Sketch => rsx! { icons::SKETCH { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Sketch" },
                                    Skill::Dioxus => rsx! { icons::DIOXUS { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Dioxus" },
                                    Skill::SwiftUI => rsx! { icons::SWIFT_UI { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Swift UI" },
                                    Skill::WPF => rsx! { icons::MICROSOFT { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "WPF" },
                                    Skill::Xamarin => rsx! { icons::XAMARIN { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Xamarin" },
                                    Skill::Flutter => rsx! { icons::FLUTTER { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Flutter" },
                                    Skill::AspNetCore => rsx! { icons::NET_CORE { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "ASP.NET Core" },
                                    Skill::Git => rsx! { icons::GIT { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Git" },
                                    Skill::YAML => rsx! { icons::AZURE { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "YAML CI/CD Pipelines" },
                                    Skill::Docker => rsx! { icons::DOCKER { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Docker" },
                                    Skill::Kubernetes => rsx! { icons::K8S { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Kubernetes" },
                                    Skill::Swift => rsx! { icons::SWIFT { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Swift" },
                                    Skill::Cpp => rsx! { icons::CPP { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "C++" },
                                    Skill::Dart => rsx! { icons::DART { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Dart" },
                                    Skill::CSharp => rsx! { icons::CSHARP { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "C#" },
                                    Skill::Rust => rsx! { icons::RUST { darkmode: props.darkmode, class: "shrink-0 size-4 me-1" }, "Rust" },
                                    Skill::Name(value) => rsx! { {value} },
                                }}
                            }
                        }
                    }
                }
            }
        }
    }
    }
}
