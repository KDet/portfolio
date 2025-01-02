use dioxus::prelude::*;
use crate::icons;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Media {
    pub title: &'static str,
    pub image: &'static str,
    pub href: &'static str,
}

#[derive(Props, Clone, PartialEq)]
pub struct GalleryProps {
    pub darkmode: bool,
    pub items: Vec<Media>,
}

#[component]
pub fn Gallery(props: GalleryProps) -> Element {
    rsx! {
        div { class: "grid grid-cols-2 sm:grid-cols-3 gap-2",
            for item in props.items.iter() {
                Link { class: "group block relative overflow-hidden rounded-lg",
                    to: item.href,
                    img { class: format_args!("w-full size-40 object-cover rounded-lg {}", if props.darkmode { "bg-neutral-800" } else { "bg-gray-100" }),
                        alt: item.title,
                        src: item.image,
                    }
                    div { class: "absolute bottom-1 end-1 opacity-0 group-hover:opacity-100 transition",
                        div { class: format_args!("flex items-center gap-x-1 py-1 px-2 rounded-lg {}", if props.darkmode { "bg-neutral-900 border-neutral-700 text-neutral-200" } else { "bg-white border border-gray-200 text-gray-800" }),
                            icons::MAGNITUDE { class: "shrink-0 size-3",  darkmode: props.darkmode }
                            span { class: "text-xs", {item.title} }
                        }
                    }
                }
            }
        }
    }
}

// <div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
//   <div class="grid grid-cols-2 gap-2 sm:grid-cols-4">
//     <div class="space-y-2">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1540575861501-7cf05a4b125a?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1668906093328-99601a1aa584?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1567016526105-22da7c13161a?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//     </div>
//     <div class="space-y-2">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1668584054131-d5721c515211?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1664574654529-b60630f33fdb?ixlib=rb-4.0.3&ixid=MnwxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//     </div>
//     <div class="space-y-2">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1486406146926-c627a92ad1ab?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1586232702178-f044c5f4d4b7?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1542125387-c71274d94f0a?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//     </div>
//     <div class="space-y-2">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1668869713519-9bcbb0da7171?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//       <img class="object-cover w-full h-auto" src="https://images.unsplash.com/photo-1668584054035-f5ba7d426401?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=560&q=80" alt="Gallery Masonry Image">
//     </div>
//   </div>
// </div>