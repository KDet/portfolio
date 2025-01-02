use crate::icons;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KnownLogo {
    Bitimpulse,
    Exoft,
    GlobalLogic,
    Netminds,
    // Other,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Feed {
    pub logo: KnownLogo,
    pub title: &'static str,
    pub description: Option<&'static str>,
    pub date: &'static str,
    pub notes: Option<Vec<&'static str>>,
    pub blog: Option<CardBlog>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CardBlog {
    pub title: &'static str,
    pub description: &'static str,
    pub image: &'static str,
    pub url: &'static str,
}

#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    pub darkmode: bool,
    pub items: Vec<Feed>,
}

#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    rsx! {
        div {
            for work in props.items.iter() {
                div { class: "group relative flex gap-x-5",
                    div { class: format_args!("relative group-last:after:hidden after:absolute after:top-8 after:bottom-2 after:start-3 after:w-px after:-translate-x-[0.5px] {}", if props.darkmode { "after:bg-neutral-700" } else { "after:bg-gray-200" }),
                        div { class: "relative z-10 size-6 flex justify-center items-center",
                            {match work.logo {
                                KnownLogo::GlobalLogic => rsx!{ icons::GLOBAL_LOGIC {darkmode: props.darkmode, class: "shrink-0 size-10" } },
                                KnownLogo::Exoft => rsx!{ icons::EXOFT {darkmode: props.darkmode, class: "shrink-0 size-10" } },
                                KnownLogo::Bitimpulse => rsx!{ icons::BITIMPULSE {darkmode: props.darkmode, class: "shrink-0 size-10" } },
                                KnownLogo::Netminds => rsx!{ icons::INTER_LOGIC {darkmode: props.darkmode, class: "shrink-0 size-10" } },
                                // KnownLogo::Other => rsx! { Work {darkmode: props.darkmode, class: "shrink-0 size-6" } }
                            }}
                        }
                    }
                    div { class: "grow pb-8 group-last:pb-0",
                        h3 { class: format_args!("mb-1 text-xs {}", if props.darkmode { "text-neutral-400" } else { "text-gray-600" }),
                            {work.date}
                        }
                        p { class: format_args!("font-semibold text-sm {}", if props.darkmode { "text-neutral-200" } else { "text-gray-800" }),
                            {work.title}
                        }
                        if let Some(description) = &work.description {
                            p { class: format_args!("mt-1 text-sm {}", if props.darkmode { "text-neutral-400" } else { "text-gray-600" }),
                                {description}
                            }
                        }
                        if let Some(roles) = &work.notes {
                            ul { class: "list-disc ms-6 mt-3 space-y-1.5",
                                for role in roles.iter() {
                                    li { class: format_args!("ps-1 text-sm {}", if props.darkmode { "text-neutral-400" } else { "text-gray-600" }),
                                        {role}
                                    }
                                }
                            }
                        }
                        if let Some(blog) = &work.blog {
                            div { class: "mt-3",
                                a { class: format_args!("block border rounded-lg hover:shadow-sm focus:outline-none {}", if props.darkmode { "border-neutral-700" } else { "border-gray-200" }),
                                    href: blog.url,
                                    div { class: "relative flex items-center overflow-hidden",
                                        img { class: "w-32 sm:w-48 h-full absolute inset-0 object-cover rounded-s-lg",
                                            src: blog.image,
                                            alt: "Blog Image",
                                        }
                                        div { class: "grow p-4 ms-32 sm:ms-48",
                                            div { class: "min-h-24 flex flex-col justify-center",
                                                h3 { class: format_args!("font-semibold text-sm {}", if props.darkmode  { "text-neutral-300" } else { "text-gray-800" }),
                                                   {blog.title}
                                                }
                                                p { class: format_args!("mt-1 text-sm {}", if props.darkmode  { "text-neutral-500" } else { "text-gray-500" }),
                                                    {blog.description}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
