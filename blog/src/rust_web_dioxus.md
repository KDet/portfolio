# 🦀 Rust Experiments: WebAssembly development

> November 11, 2024

Rust has rapidly become one of the [most popular programming languages](https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/)  among developers. According to the [Stack Overflow Developer Survey](https://stackoverflow.blog/2023/06/13/developer-survey-results-are-in/), Rust has consistently ranked as one of the "most loved" programming languages for several years. This popularity can be attributed to its combination of high performance and memory safety, which sets it apart from many other languages. Additionally, Rust's use in critical, security-sensitive applications over the past few years has solidified its reputation as a reliable and powerful language.

Notable Examples of Rust Adoption:

- **Google's Shift to Rust Programming** 

Google has adopted Rust for Android development, leading to a [68% reduction in memory vulnerabilities](https://thehackernews.com/2024/09/googles-shift-to-rust-programming-cuts.html).

- **Linux Kernel Integration**

Rust has been integrated into the Linux kernel since [Linus Torvalds approved it for the Linux 6.1 release](https://www.zdnet.com/article/linus-torvalds-rust-will-go-into-linux-6-1/), and it has continued [evolving in this role](https://www.zdnet.com/article/rust-in-linux-where-we-are-and-where-were-going-next/). Additionally, [Pop!_OS's COSMIC desktop environment](https://system76.com/cosmic/) has been implemented using Rust, showcasing its effectiveness in building reliable system components.

- **Microsoft's Windows Rewriting**

[Microsoft is rewriting core Windows components in memory-safe Rust](https://www.theregister.com/2023/04/27/microsoft_windows_rust/), aiming to enhance the security and stability of its operating system.

- **Project Caliptra**

Developed by NVIDIA, AMD, Google, and Microsoft, [Project Caliptra](https://github.com/chipsalliance/Caliptra) is a critical [Root of Trust](https://medium.com/@chanibonner/a-beginners-guide-to-root-of-trust-and-secure-boot-1ccf72b3e9b2) component. It provides the foundation for secure operations in computing systems by managing cryptographic keys and enabling secure boot processes. This initiative is expected to significantly impact [data center solutions](https://www.opencompute.org/blog/cloud-security-integrating-trust-into-every-chip) and [AI infrastructure advancements](https://azure.microsoft.com/en-us/blog/fostering-ai-infrastructure-advancements-through-standardization/). Google and Microsoft are planning to implement Caliptra in their [silicon in 2024](https://semianalysis.com/2022/10/25/caliptra-first-open-source-silicon/), while [AMD targets 2026+](https://community.amd.com/t5/corporate/addressing-security-integrating-project-caliptra-into-amd-s/ba-p/716837).

- **Mozilla's Gecko Engine**

Rust has been integrated into [Mozilla's Gecko](https://firefox-source-docs.mozilla.org/testing-rust-code/index.html) browser engine, enhancing memory safety and performance in web browsing.

- **Automotive Industry Adoption**

Rust has been [adopted by Volvo for its production lines](https://tweedegolf.nl/en/blog/137/rust-is-rolling-off-the-volvo-assembly-line), and it is seen as the [future of automotive software development](https://www.linkedin.com/pulse/power-rust-future-automotive-software-development-acsiatech/?lipi=urn%3Ali%3Apage%3Ad_flagship3_pulse_read%3BQIwogWXUT%2F6vQxKinbW7Hg%3D%3D), providing reliability for safety-critical systems.

- **AI Infrastructure**

xAI, a company led by Elon Musk behind the [Grok AI system](https://x.ai/blog/grok), has built its infrastructure using Rust. [Hugging Face](https://huggingface.co/), one of the largest AI communities, also [uses Rust components](https://github.com/huggingface), demonstrating the language's suitability for high-performance AI applications.

- **Blockchain Development**

Rust is widely used in blockchain development. Notable projects include the [Foundry toolkit for Ethereum applications](https://github.com/foundry-rs), [Lightning Dev Kit for Bitcoin](https://github.com/lightningdevkit), [Parity Polkadot SDK](https://github.com/paritytech/polkadot-sdk), and the [Solana blockchain](https://solana.com/).

- **Tor and Privacy Tools**

Rust is being used to implement the [Tor](https://www.torproject.org/) anonymity protocols in the [Arti Project](https://tpo.pages.torproject.net/core/arti/), as well as in other privacy tools such as [1Password](https://1password.com/), [Firecracker](https://firecracker-microvm.github.io/), and [Cloudflare](https://www.cloudflare.com/en-gb/).

- **Aerospace Community**

Rust also has an active aerospace community focusing on using Rust in aerospace applications ([AeroRust](https://aerorust.org/)).

- **Microsoft Azure Endorsement**

Microsoft Azure CTO Mark Russinovich has endorsed using Rust to avoid memory safety vulnerabilities, recognizing its benefits over traditional languages like C and C++. 

![Microsoft Azure CTO Mark Russinovich](https://media.licdn.com/dms/image/v2/D4E12AQEn-3KSyCPZKw/article-inline_image-shrink_1500_2232/article-inline_image-shrink_1500_2232/0/1731347766205?e=1737590400&v=beta&t=XoS67fQxgn9K0L7uWWIFK7v__nCwzG0SvypWmKw8rM4)

- **DARPA Initiative** 

The U.S. government, through [DARPA, plans to translate legacy C code to Rust](https://www.darpa.mil/program/translating-all-c-to-rust) to enhance system security.

## Web Development with Rust and WebAssembly

Rust has rapidly become a go-to language for native web development, especially through [WebAssembly (Wasm)](https://developer.mozilla.org/en-US/docs/WebAssembly). WebAssembly, co-developed by Mozilla, allows developers to compile Rust code that runs at near-native performance in a web environment while preserving Rust's memory safety features. This makes Rust an excellent choice for [building fast, secure web applications](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm).

The Rust ecosystem includes several frameworks that make web development efficient and enjoyable. Below are some of the most popular frameworks for Rust web development: [Dioxus](https://dioxuslabs.com/), [Leptos](https://leptos.dev/), [Yew](https://yew.rs/), [Seed](https://github.com/seed-rs/seed), etc. For a detailed comparison of these frameworks, refer to this [comparison table](https://krausest.github.io/js-framework-benchmark/2024/table_chrome_129.0.6668.58.html), which outlines metrics like rendering speed, build size, and transferred size. 

Among the listed frameworks, [Dioxus aims to provide a unified development experience across web, desktop, and mobile platforms](https://dioxuslabs.com/learn/0.6/#introduction). It has features like live hot reloading, server functions, and easy deployment. Its React-inspired design also makes it easy for developers familiar with [React](https://www.linkedin.com/redir/invalid-link-page?url=https%3A%2F%2Freact%2edev%2F&lipi=urn%3Ali%3Apage%3Ad_flagship3_pulse_read%3BQIwogWXUT%2F6vQxKinbW7Hg%3D%3D) to adopt. Also, you can learn more about their plans for native support on [the official roadmap page](https://github.com/dioxuslabs/dioxus/milestones).

## Component in Dioxus

We won't cover the initial setup and first component creation, as you can find a comprehensive guide on the [Dioxus documentation site](https://dioxuslabs.com/learn/0.6/guide/your_first_component) along with [the full code](https://dioxuslabs.com/learn/0.6/router/example/full-code).

Since Dioxus supports styling applications using [Tailwind CSS](https://dioxuslabs.com/learn/0.6/cookbook/tailwind) framework or vanilla CSS, we’ll use [Tailwind](https://tailwindcss.com/), specifically, [Preline UI's Mini Floating Header](https://www.preline.co/examples/navigations-navbars.html#mini-floating-header) as our example.

- **Converting HTML to RSX with dx translate**

Dioxus uses RSX to represent HTML in Rust code. RSX is concise and idiomatic, but converting HTML manually can be tedious. To streamline this process, Dioxus provides the dx translate command-line tool: 

```
$ dx translate --file index.html
```

This command makes it easy to take HTML and convert it into RSX for further customization in Dioxus.

- **Optimizing and Adding Interactivity to Our Component**

Once we have the RSX, we can optimize our code by using [Manual Props](https://dioxuslabs.com/learn/0.6/migration/props#manual-props) and create a NavBar component. Props allow to make components more flexible and reusable. For **NavBar**, we’ll add parameters for the navigation items, dark mode, and brand link. We'll also set up [event handlers](https://dioxuslabs.com/learn/0.6/reference/event_handlers) for user interactions, such as clicking on navigation links or toggling the mobile menu.

To simplify the implementation, we use the following helper crates:

* `dioxus-spring = "0.2.1"`: An animation library for Dioxus. More details are available [here](https://github.com/dioxus-community/dioxus-spring).
* `dioxus-use-mounted = "0.2.1"`: A utility to encapsulate component `MountedData`, which you can find [here](https://github.com/dioxus-community/dioxus-use-mounted/blob/main/src/lib.rs).

Here’s the complete implementation of the NavBar component:

```rust
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
```

It refers a **scroll_into_view** function for smooth scrolling to a selected section and a **get_scroll_height** function to calculate the height for the mobile menu. Here's the implementation:

```rust
fn scroll_into_view(mounted: UseMounted) {
    if let Some(element) = &*mounted.signal.read() {
        let _ = element.scroll_to(ScrollBehavior::Smooth);
    }
}

fn get_scroll_height(mounted: UseMounted) -> Option<i32> {
    if let Some(element) = &*mounted.signal.read() {
        if let Some(raw_elem) = element.downcast::<web_sys::Element>() {
            let att = raw_elem.get_attribute("style").unwrap_or_default();
            raw_elem.set_attribute("style", format!("{};display: block;", att).as_str()).unwrap_or_default();
            let height = raw_elem.scroll_height();
            raw_elem.set_attribute("style", att.as_str()).unwrap_or_default();
            return Some(height);
        }
    }
    None
}
```

When development is done, it's time to add some optimizations. You can find more information [here](https://dioxuslabs.com/learn/0.6/cookbook/optimizing). I use these

```
[profile.release]
opt-level = "z"
debug = false
lto = true
codegen-units = 1
panic = "abort"
strip = true
incremental = false
```

# Deployment

Dioxus-specific deployment support is coming soon, and you can follow the updates [here](https://dioxuslabs.com/deploy). In the meantime, we can deploy a Dioxus application using Azure Static Web Apps, similar to the steps described [here](https://www.linkedin.com/pulse/hosting-flutter-web-app-azure-static-apps-dmytro-kravchyna-pwp9e/?trackingId=emI92qzFTVigexUSzOl68Q%3D%3D&lipi=urn%3Ali%3Apage%3Ad_flagship3_pulse_read%3BQIwogWXUT%2F6vQxKinbW7Hg%3D%3D).

We can also explore community deployment samples from [Dioxus Awesome](https://dioxuslabs.com/awesome). For instance, try deploying using the [Vercel](https://vercel.com/) template, as demonstrated in the [dioxus-vercel-demo repository](https://github.com/lucifer1004/dioxus-vercel-demo). Note that I had to modify the deployment instructions slightly to get it working:

Replace:

```yaml
 - name: Install trunk
    if: steps.cache.outputs.cache-hit != 'true'
    uses: actions-rs/cargo@v1
    with:
      use-cross: true
      command: install
      args: trunk

  - name: Build web pages
    run: trunk build --release
```

With:

```yaml
- name: Install Dioxus
    if: steps.cache.outputs.cache-hit != 'true'
    uses: actions-rs/cargo@v1
    with:
      use-cross: true
      command: install
      args: dioxus-cli

  - name: Build web pages
    run: dx build --release
```

The component described here is part of a larger application, which you can view [here](https://dmytro-kravchyna.vercel.app/).

# Conclusion

Rust has gained widespread popularity for its performance, memory safety, and suitability for critical applications. This article outlines Rust’s adoption across various sectors, from Google’s Android development to Microsoft’s Windows core components, Linux kernel, AI infrastructure, and even aerospace and automotive industries. Rust's integration into Mozilla's Gecko engine, blockchain platforms, and privacy tools underscores its versatility and reliability.

For web development, Rust's compatibility with WebAssembly has enabled fast, secure web applications. Frameworks like Dioxus, Yew, and Leptos provide Rust developers with efficient tools for building web applications. Notably, Dioxus extends Rust’s reach to desktop and mobile, creating a unified development experience.