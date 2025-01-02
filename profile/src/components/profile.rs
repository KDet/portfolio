use dioxus::prelude::*;
use crate::icons;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Biography {
    pub photo: &'static str,
    pub fullname: &'static str,
    pub title: &'static str,
    pub about: Option<&'static str>,
    pub contacts: Option<Vec<Contact>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ContactType {
    Email,
    Telegram,
    // TwitterX,
    // LinkedIn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Contact {
    pub platform: ContactType,
    pub href: &'static str,
    pub title: &'static str,
}

#[component]
pub fn UserProfile(
    darkmode: bool,
    onmounted: Option<EventHandler<Event<MountedData>>>,
    id: Option<&'static str>,
    bio: Biography,
) -> Element {
    rsx! {
        section {
            id: id.unwrap_or_default(),
            onmounted: move |data| if let Some(mounted) = &onmounted { mounted.call(data) },
            div { class: "flex items-center gap-x-3",
                div { class: "shrink-0",
                    img { class: "shrink-0 size-16 rounded-full",
                        alt: "Avatar",
                        src: bio.photo,
                    }
                }
                div { class: "grow",
                    h1 { class: format_args!("text-lg font-medium {}", if darkmode { "text-neutral-200" } else { "text-gray-800" }),
                        {bio.fullname}
                    }
                    p { class: format_args!("text-sm {}", if darkmode { "text-neutral-400" } else { "text-gray-600" }),
                        {bio.title}
                    }
                }
            }
            if bio.about.is_some() || bio.contacts.is_some() {
                div { class: "mt-8",
                    if let Some(about) = &bio.about {
                        for (i, parag) in about.split('\n').enumerate() {
                            p { class: format_args!("text-sm {} {}", if darkmode { "text-neutral-400" } else { "text-gray-600" }, if i != 0 { "mt-3" } else { "" }),
                                {parag}
                            }
                        }
                    }
                    if let Some(contacts) = &bio.contacts {
                        ul { class: "mt-5 flex flex-col gap-y-3",
                            for contact in contacts {
                                li { class: "flex items-center gap-x-2.5",
                                    match contact.platform {
                                        ContactType::Email => rsx! { icons::ENVELOPE {darkmode: darkmode, class: "shrink-0 size-3.5" } },
                                        // ContactType::LinkedIn => rsx! {LinkedIn {darkmode: dark_mode, class: "shrink-0 size-3.5" }},
                                        ContactType::Telegram => rsx! { icons::TELEGRAM {darkmode: darkmode, class: "shrink-0 size-3.5" } },
                                        // ContactType::TwitterX => rsx! {TwitterX {darkmode: dark_mode, class: "shrink-0 size-3.5" }},
                                    }
                                    Link { class: format_args!("text-[13px] underline hover:decoration-2 focus:outline-none focus:decoration-2 {}", if darkmode { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" }),
                                        to: contact.href,
                                        new_tab: true,
                                        {contact.title}
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
