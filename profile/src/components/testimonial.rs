use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Testimonial {
    pub quote: &'static str,
    pub photo: &'static str,
    pub fullname: &'static str,
}

#[derive(Props, Clone, PartialEq)]
pub struct TestimonialsProps {
    pub darkmode: bool,
    pub items: Vec<Testimonial>,
}

#[component]
pub fn Testimonials(props: TestimonialsProps) -> Element {
    rsx! {
        div { class: format_args!("grid grid-cols-1 sm:grid-cols-2 gap-x-3 border-y divide-y sm:divide-y-0 sm:divide-x {}", if props.darkmode { "border-neutral-700 divide-neutral-700" } else { "border-gray-200 divide-gray-200" }),
            for (i, feedback) in props.items.iter().enumerate() {
                div { class: format_args!("py-6 sm:px-4 {}", if i == 0 { "sm:-ms-4" } else { "" }),
                    blockquote {
                        span { class: format_args!("text-sm {}", if props.darkmode { "text-neutral-200" } else { "text-gray-800" }),
                            {feedback.quote}
                        }
                        footer { class: "mt-3",
                            div { class: "flex items-center gap-x-2",
                                img { class: "shrink-0 size-5 rounded-full",
                                    src: feedback.photo,
                                    alt: "Avatar",
                                }
                                div { class: "grow",
                                    div { class: format_args!("text-xs {}", if props.darkmode { "text-neutral-500" } else { "text-gray-500" }),
                                        {feedback.fullname}
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