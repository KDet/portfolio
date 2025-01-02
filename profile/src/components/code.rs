use crate::icons;
use dioxus::prelude::*;

#[component]
pub fn CodeBlock(contents: String, darkmode: bool, name: Option<String>) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        div { class: format_args!("border overflow-hidden rounded-md mb-2 {}", if darkmode { "border-neutral-700" } else { "border-gray-200" }),
            div { class: format_args!("w-full flex flex-row justify-between border-b py-1 px-2 text-xs items-center {}", if darkmode { "border-neutral-700 bg-ideblack" } else { "border-gray-200 bg-gray-100" }),
                div { class: "font-mono text-sm",
                    if let Some(path) = name {
                        "src/{path}"
                    }
                }
                button {
                    class: format_args!("flex flex-row items-center gap-1 text-xs {}", if darkmode { "text-neutral-400" } else { "text-gray-600" }),
                    "onclick": "navigator.clipboard.writeText(this.parentNode.parentNode.lastChild.innerText);",
                    onclick: move |_| copied.set(true),
                    // if let Some(window) = window() {
                    //     if let Some(document) = window.document() {
                    //         let parent_node = document.query_selector(".container").unwrap();
                    //         if let Some(parent) = parent_node {
                    //             if let Some(last_child) = parent.last_child() {
                    //                 let text = last_child.text_content().unwrap_or_default();
                    //                 let clipboard = window.navigator().clipboard();
                    //                 if let Some(clipboard) = clipboard {
                    //                     clipboard.write_text(&text).unwrap();
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                    if copied() {
                        "Copied!"
                    }
                    span { icons::COPY { darkmode: darkmode, class: "shrink-0 size-4 me-1" } }
                }
            } //p-4
            div { class: "text-xs overflow-y-auto", dangerous_inner_html: contents }
        }
    }
}
