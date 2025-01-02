// use dioxus::prelude::*;

// use crate::components::icons::Message;

// #[derive(Props, Clone, PartialEq)]
// pub struct SubscribeProps {
//     pub darkmode: bool,
// }

// #[component]
// pub fn Subscribe(props: SubscribeProps) -> Element {
//     rsx! {
//         form {
//             div {
//                 class: format_args!("p-1.5 flex flex-col sm:flex-row items-center gap-2 border rounded-lg {}", if props.darkmode { "border-neutral-700" } else { "border-gray-200" }),
//                 div { class: "relative w-full",
//                     label { class: "sr-only",
//                         r#for: "hero-input",
//                         "Subscribe"
//                     }
//                     div { class: "absolute inset-y-0 start-0 flex items-center pointer-events-none z-20 ps-3",
//                         { rsx! { Message {class: "shrink-0 size-4",  darkmode: props.darkmode} }}
//                     }
//                     input { class: format_args!("py-2 ps-9 pe-3 block w-full border-transparent rounded-lg text-sm focus:border-transparent focus:ring-transparent disabled:opacity-50 disabled:pointer-events-none {}",  if props.darkmode { "bg-neutral-900 text-neutral-400 placeholder-neutral-500" } else { "" }),
//                         placeholder: "Enter your email",
//                         r#type: "text",
//                         name: "hero-input",
//                         id: "hero-input"
//                     }
//                 }
//                 a { class: format_args!("w-full sm:w-auto whitespace-nowrap py-2 px-2.5 inline-flex justify-center items-center gap-x-2 text-sm font-semibold rounded-md border border-transparent {}", if props.darkmode { "bg-white text-neutral-800 hover:bg-neutral-200" } else { "bg-gray-800 text-white hover:bg-gray-900" }),
//                     href: "#",
//                     "Join"
//                     svg {
//                         stroke_linecap: "round",
//                         stroke_width: "2",
//                         fill: "none",
//                         stroke_linejoin: "round",
//                         view_box: "0 0 24 24",
//                         stroke: "currentColor",
//                         width: "24",
//                         height: "24",
//                         class: "shrink-0 size-3.5",
//                         path { d: "M5 12h14" }
//                         path { d: "m12 5 7 7-7 7" }
//                     }
//                 }
//             }
//             p { class: format_args!("mt-2 text-xs {}", if props.darkmode { "text-neutral-500" } else { "text-gray-500" }),
//                 "No spam, unsubscribe at any time."
//             }
//         }
//     }
// }
