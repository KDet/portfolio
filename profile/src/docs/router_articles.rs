use super::*;
use crate::DarkMode;
use dioxus::prelude::*;
#[derive(
    Clone,
    Copy,
    dioxus_router::prelude::Routable,
    PartialEq,
    Eq,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BookRoute {
    #[route("/vim_math3d_cpp")]
    VimMath3DCpp {},
    #[route("/flutter_web_deployment_azure")]
    FlutterWebDeploymentAzure {},
    #[route("/rust_web_dioxus")]
    RustWebDioxus {},
}
impl BookRoute {
    pub fn sections(&self) -> &'static [use_mdbook::mdbook_shared::Section] {
        &self.page().sections
    }
    pub fn page(&self) -> &'static use_mdbook::mdbook_shared::Page<Self> {
        LAZY_BOOK.get_page(self)
    }
    pub fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        match self {
            BookRoute::VimMath3DCpp {} => use_mdbook::mdbook_shared::PageId(0usize),
            BookRoute::FlutterWebDeploymentAzure {} => use_mdbook::mdbook_shared::PageId(1usize),
            BookRoute::RustWebDioxus {} => use_mdbook::mdbook_shared::PageId(2usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::VimMath3DCpp {}
    }
}
pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> =
    use_mdbook::Lazy::new(|| {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        pages
            .push((
                0usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "🌍 Open-Source C++ Version of Vim.Math3D! $ #VIM #3D #Math #MIT #C++ $ October 13, 2024 $ Does your project need an efficient library for 3D mathematics? VIM Math 3D is a lightweight and easy-to-use library. It is now available for C++."
                            .to_string(),
                        url: BookRoute::VimMath3DCpp {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🌍 Open-Source C++ Version of Vim.Math3D!"
                                    .to_string(),
                                id: "🌍-open-source-c++-version-of-vim.math3d!"
                                    .to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(0usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::VimMath3DCpp {},
            ::use_mdbook::mdbook_shared::PageId(0usize),
        );
        pages
            .push((
                1usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "🚀 Deploying a Flutter Web App on Azure Static Web Apps $ #Azure #Flutter #YAML $ November 10, 2024 $ If you want to experiment with the new Azure Static Web App free service plan and Flutter Web, this guide may speed up the getting started process."
                            .to_string(),
                        url: BookRoute::FlutterWebDeploymentAzure {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Deploying a Flutter Web App on Azure Static Web Apps"
                                    .to_string(),
                                id: "🚀-deploying-a-flutter-web-app-on-azure-static-web-apps"
                                    .to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Step 1: Prepare build-time variables".to_string(),
                                id: "step-1:-prepare-build-time-variables".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Step 2: Make Azure DevOps Build Pipeline"
                                    .to_string(),
                                id: "step-2:-make-azure-devops-build-pipeline".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Step 3: Create Azure Static Web App".to_string(),
                                id: "step-3:-create-azure-static-web-app".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Step 4: Handling CORS Issues".to_string(),
                                id: "step-4:-handling-cors-issues".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Summary".to_string(),
                                id: "summary".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(1usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::FlutterWebDeploymentAzure {},
            ::use_mdbook::mdbook_shared::PageId(1usize),
        );
        pages
            .push((
                2usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "🦀 Rust Experiments: WebAssembly development $ #RustLang #WebAssembly #Dioxus #TailwindCSS $ November 11, 2024 $ Rust is worth experimenting with, and WebAssembly is the first and fun stop for diving into its potential!"
                            .to_string(),
                        url: BookRoute::RustWebDioxus {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🦀 Rust Experiments: WebAssembly development"
                                    .to_string(),
                                id: "🦀-rust-experiments:-webassembly-development"
                                    .to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Web Development with Rust and WebAssembly"
                                    .to_string(),
                                id: "web-development-with-rust-and-webassembly".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Component in Dioxus".to_string(),
                                id: "component-in-dioxus".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Deployment".to_string(),
                                id: "deployment".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Conclusion".to_string(),
                                id: "conclusion".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(2usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::RustWebDioxus {},
            ::use_mdbook::mdbook_shared::PageId(2usize),
        );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "🌍 Open-Source C++ Version of Vim.Math3D! $ #VIM #3D #Math #MIT #C++ $ October 13, 2024 $ Does your project need an efficient library for 3D mathematics? VIM Math 3D is a lightweight and easy-to-use library. It is now available for C++."
                            .to_string(),
                        location: Some(BookRoute::VimMath3DCpp {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "🚀 Deploying a Flutter Web App on Azure Static Web Apps $ #Azure #Flutter #YAML $ November 10, 2024 $ If you want to experiment with the new Azure Static Web App free service plan and Flutter Web, this guide may speed up the getting started process."
                            .to_string(),
                        location: Some(BookRoute::FlutterWebDeploymentAzure {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "🦀 Rust Experiments: WebAssembly development $ #RustLang #WebAssembly #Dioxus #TailwindCSS $ November 11, 2024 $ Rust is worth experimenting with, and WebAssembly is the first and fun stop for diving into its potential!"
                            .to_string(),
                        location: Some(BookRoute::RustWebDioxus {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32]),
                        ),
                        nested_items: vec![],
                    }),
                ],
                suffix_chapters: vec![],
            },
            pages: pages.into_iter().collect(),
            page_id_mapping,
        }
    });
#[component(no_case_check)]
pub fn VimMath3DCpp() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    let dark_mode = use_context::<Signal<DarkMode>>();
    rsx! {
        h1 {
            id: "-open-source-c-version-of-vimmath3d",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#-open-source-c-version-of-vimmath3d",
                "🌍 Open-Source C++ Version of Vim.Math3D!"
            }
        }
        blockquote {
            p {
                class: "text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                "October 13, 2024"
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            a {
                href: "https://github.com/vimaec/Math3D",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Vim.Math3D"
            }
            " is an efficient 3D mathematics library originally developed in C# for .NET Standard 2.0 by the "
            a {
                href: "https://www.vimaec.com/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "VIM team"
            }
            ". The library was designed as a more robust alternative to System.Numerics, eliminating the need for any additional dependencies. The original version is licensed under the "
            strong { "MIT license" }
            "."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            a {
                href: "https://github.com/KDet/vim_math3d/tree/develop/src/cpp",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "This repository"
            }
            " contains a translation of Vim.Math3D from C# to C++. The C++ version is a "
            strong { "C++17 header-only" }
            " library that preserves all the core advantages of the original, including being lightweight, cross-platform, and free of external dependencies. ✨ The project leverages CMake for build configuration, ensuring straightforward integration across platforms. Additionally, comprehensive unit tests ("
            code { "using <cassert>" }
            ") are included to verify correctness."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            img {
                src: "https://media.licdn.com/dms/image/v2/D4E12AQEX8vGThJI4_w/article-inline_image-shrink_1000_1488/article-inline_image-shrink_1000_1488/0/1728841027929?e=1737590400&v=beta&t=UnQkzXwL3YX5fsluGQRW7GgvCwScAJXtij-ugMm65F8",
                alt: "VIM Math C++ Unit Tests",
                class: "rounded-lg",
                class: if dark_mode().0 { "bg-neutral-800" } else { "bg-gray-100" },
                title: "",
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "If you need an open-source 3D math library that integrates easily into C++ projects, Vim.Math3D may be useful. It’s suitable for a wide range of applications, from game development to physics simulations or any project requiring efficient 3D mathematical computations."
        }
    }
}
#[component(no_case_check)]
pub fn FlutterWebDeploymentAzure() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    let dark_mode = use_context::<Signal<DarkMode>>();
    rsx! {
        h1 {
            id: "-deploying-a-flutter-web-app-on-azure-static-web-apps",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#-deploying-a-flutter-web-app-on-azure-static-web-apps",
                "🚀 Deploying a Flutter Web App on Azure Static Web Apps"
            }
        }
        blockquote {
            p {
                class: "text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                "November 10, 2024"
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Flutter provides a versatile way to build web applications, and Azure Static Web Apps offers a good platform for hosting them. This guide will walk you through building a Flutter web app using Azure DevOps and deploying it to Azure Static Web Apps, ensuring a clear understanding of each step."
        }
        h2 {
            id: "step-1-prepare-build-time-variables",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#step-1-prepare-build-time-variables", "Step 1: Prepare build-time variables" }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "To build a Flutter web app, use the following command pattern in your Azure DevOps pipeline:"
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"color:#f8f8f2;\">flutter build web \n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">name</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">VERSION</span><span style=\"color:#f8f8f2;\">) \n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">number</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">BUILD_NUMBER</span><span style=\"color:#f8f8f2;\">) \n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">dart</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">define</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#f8f8f2;\">) \n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">web</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">renderer html </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">release</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"color:#000000;\">flutter build web \n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">name</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">VERSION</span><span style=\"color:#000000;\">) \n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">number</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">BUILD_NUMBER</span><span style=\"color:#000000;\">) \n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">dart</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">define</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#000000;\">) \n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">web</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">renderer html </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">release</span></pre>\n" },
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            strong { "Explanation:" }
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                code { "--web-renderer html" }
                ": Specifies the HTML renderer for Flutter. Flutter provides two web renderers: html and canvaskit. The HTML renderer is lightweight and ideal for text-heavy apps, while canvaskit is better for complex graphics. Using the HTML renderer results in smaller app sizes, faster load times, and better browser compatibility."
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Flutter also supports compiling web apps to WebAssembly (Wasm), which enables near-native performance in the browser and provides significant performance boosts for computation-heavy tasks."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                code { "--dart-define" }
                ": This flag allows you to pass environment-specific variables at build time. In this example, BASE_API defines different API URLs for development, testing, and production, allowing you to control the app's behavior without modifying the source code."
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "For instance, in your Flutter app, you can use BASE_API like this:"
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"color:#f8f8f2;\">Dio networkClient() {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">const</span><span style=\"color:#f8f8f2;\"> baseUrl </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">.fromEnvironment(</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">, defaultValue: </span><span style=\"color:#f92672;\">&#39;&#39;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"background-color:#f92672;color:#f8f8f0;\">final</span><span style=\"color:#f8f8f2;\"> dio </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> Dio(BaseOptions(baseUrl: baseUrl));\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">return</span><span style=\"color:#f8f8f2;\"> dio</span><span style=\"color:#f92672;\">..</span><span style=\"color:#f8f8f2;\">transformer </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> BackgroundTransformer();\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"color:#000000;\">Dio networkClient() {{\n</span><span style=\"color:#000000;\">  </span><span style=\"font-style:italic;color:#28c6e4;\">const</span><span style=\"color:#000000;\"> baseUrl </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#28c6e4;\">String</span><span style=\"color:#000000;\">.fromEnvironment(</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#000000;\">, defaultValue: </span><span style=\"color:#f92672;\">&#39;&#39;</span><span style=\"color:#000000;\">);\n</span><span style=\"color:#000000;\">  </span><span style=\"background-color:#f92672;color:#f8f8f0;\">final</span><span style=\"color:#000000;\"> dio </span><span style=\"color:#f92672;\">=</span><span style=\"color:#000000;\"> Dio(BaseOptions(baseUrl: baseUrl));\n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">return</span><span style=\"color:#000000;\"> dio</span><span style=\"color:#f92672;\">..</span><span style=\"color:#000000;\">transformer </span><span style=\"color:#f92672;\">=</span><span style=\"color:#000000;\"> BackgroundTransformer();\n</span><span style=\"color:#000000;\">}}</span></pre>\n" },
        }
        h2 {
            id: "step-2-make-azure-devops-build-pipeline",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#step-2-make-azure-devops-build-pipeline",
                "Step 2: Make Azure DevOps Build Pipeline"
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Below is an example of a basic YAML file for Azure DevOps pipeline:"
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"color:#f8f8f2;\">trigger:\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> main\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">pool:\n</span><span style=\"color:#f8f8f2;\">  vmImage: </span><span style=\"color:#f92672;\">&#39;ubuntu-</span><span style=\"color:#f8f8f2;\">latest</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">variables:\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> group: some</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">other</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">variables</span><span style=\"color:#f92672;\">-for-</span><span style=\"color:#f8f8f2;\">pipeline\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"color:#ae81ff;\">BUILD_NUMBER\n</span><span style=\"color:#f8f8f2;\">  value: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#f8f8f2;\">(Build.BuildId)</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"color:#ae81ff;\">VERSION\n</span><span style=\"color:#f8f8f2;\">  value: </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#ae81ff;\">1.0</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#ae81ff;\">0</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"color:#ae81ff;\">PROJECT_DIRECTORY\n</span><span style=\"color:#f8f8f2;\">  value: </span><span style=\"color:#f92672;\">&#39;src/</span><span style=\"color:#f8f8f2;\">test_app</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"color:#ae81ff;\">BUILD_ARTEFACTS_PATH\n</span><span style=\"color:#f8f8f2;\">  value: </span><span style=\"color:#f92672;\">&#39;build/</span><span style=\"color:#f8f8f2;\">web</span><span style=\"color:#f92672;\">/&#39;\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"color:#ae81ff;\">BASE_API\n</span><span style=\"color:#f8f8f2;\">  value: &#39;https:</span><span style=\"color:#75715e;\">//{{server-name}}/api/&#39;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">jobs:\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> job: BuildTestWebApp\n</span><span style=\"color:#f8f8f2;\">  displayName: </span><span style=\"color:#f92672;\">&#39;Build</span><span style=\"color:#f8f8f2;\"> Test Web App</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">  steps:\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> task: FlutterInstall</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ae81ff;\">0\n</span><span style=\"color:#f8f8f2;\">    inputs:\n</span><span style=\"color:#f8f8f2;\">      mode: </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">auto</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">      channel: </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">stable</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">      version: </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">latest</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> task: FlutterCommand</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ae81ff;\">0\n</span><span style=\"color:#f8f8f2;\">    inputs:\n</span><span style=\"color:#f8f8f2;\">      projectDirectory: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">PROJECT_DIRECTORY</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">      arguments: </span><span style=\"color:#f92672;\">&#39;build</span><span style=\"color:#f8f8f2;\"> web </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">name</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">VERSION</span><span style=\"color:#f8f8f2;\">) </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">number</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">BUILD_NUMBER</span><span style=\"color:#f8f8f2;\">) </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">dart</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">define</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#f8f8f2;\">) </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">web</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">renderer html </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">release</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> task: AzureStaticWebApp</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ae81ff;\">0\n</span><span style=\"color:#f8f8f2;\">    displayName: </span><span style=\"color:#f92672;\">&#39;Deploy</span><span style=\"color:#f8f8f2;\"> Flutter Web </span><span style=\"font-style:italic;color:#66d9ef;\">Drop</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">    inputs:\n</span><span style=\"color:#f8f8f2;\">      app_location: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">PROJECT_DIRECTORY</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">/$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">BUILD_ARTEFACTS_PATH</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">      output_location: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">PROJECT_DIRECTORY</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">/$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">BUILD_ARTEFACTS_PATH</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">      skip_app_build: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#f8f8f2;\">      skip_api_build: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#f8f8f2;\">      verbose: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#f8f8f2;\">      azure_static_web_apps_api_token: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">DEPLOYMENT_TOKEN</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">&#39;</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"color:#000000;\">trigger:\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> main\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">pool:\n</span><span style=\"color:#000000;\">  vmImage: </span><span style=\"color:#f92672;\">&#39;ubuntu-</span><span style=\"color:#000000;\">latest</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">variables:\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> group: some</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">other</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">variables</span><span style=\"color:#f92672;\">-for-</span><span style=\"color:#000000;\">pipeline\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> name: </span><span style=\"color:#ae81ff;\">BUILD_NUMBER\n</span><span style=\"color:#000000;\">  value: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#000000;\">(Build.BuildId)</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> name: </span><span style=\"color:#ae81ff;\">VERSION\n</span><span style=\"color:#000000;\">  value: </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#ae81ff;\">1.0</span><span style=\"color:#000000;\">.</span><span style=\"color:#ae81ff;\">0</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> name: </span><span style=\"color:#ae81ff;\">PROJECT_DIRECTORY\n</span><span style=\"color:#000000;\">  value: </span><span style=\"color:#f92672;\">&#39;src/</span><span style=\"color:#000000;\">test_app</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> name: </span><span style=\"color:#ae81ff;\">BUILD_ARTEFACTS_PATH\n</span><span style=\"color:#000000;\">  value: </span><span style=\"color:#f92672;\">&#39;build/</span><span style=\"color:#000000;\">web</span><span style=\"color:#f92672;\">/&#39;\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> name: </span><span style=\"color:#ae81ff;\">BASE_API\n</span><span style=\"color:#000000;\">  value: &#39;https:</span><span style=\"color:#9f9f8f;\">//{{server-name}}/api/&#39;\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">jobs:\n</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> job: BuildTestWebApp\n</span><span style=\"color:#000000;\">  displayName: </span><span style=\"color:#f92672;\">&#39;Build</span><span style=\"color:#000000;\"> Test Web App</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">  steps:\n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> task: FlutterInstall</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ae81ff;\">0\n</span><span style=\"color:#000000;\">    inputs:\n</span><span style=\"color:#000000;\">      mode: </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#000000;\">auto</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">      channel: </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#000000;\">stable</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">      version: </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#000000;\">latest</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> task: FlutterCommand</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ae81ff;\">0\n</span><span style=\"color:#000000;\">    inputs:\n</span><span style=\"color:#000000;\">      projectDirectory: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">PROJECT_DIRECTORY</span><span style=\"color:#000000;\">)</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">      arguments: </span><span style=\"color:#f92672;\">&#39;build</span><span style=\"color:#000000;\"> web </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">name</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">VERSION</span><span style=\"color:#000000;\">) </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">number</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">BUILD_NUMBER</span><span style=\"color:#000000;\">) </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">dart</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">define</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#f92672;\">=$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">BASE_API</span><span style=\"color:#000000;\">) </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">web</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">renderer html </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">release</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> task: AzureStaticWebApp</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ae81ff;\">0\n</span><span style=\"color:#000000;\">    displayName: </span><span style=\"color:#f92672;\">&#39;Deploy</span><span style=\"color:#000000;\"> Flutter Web </span><span style=\"font-style:italic;color:#28c6e4;\">Drop</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">    inputs:\n</span><span style=\"color:#000000;\">      app_location: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">PROJECT_DIRECTORY</span><span style=\"color:#000000;\">)</span><span style=\"color:#f92672;\">/$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">BUILD_ARTEFACTS_PATH</span><span style=\"color:#000000;\">)</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">      output_location: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">PROJECT_DIRECTORY</span><span style=\"color:#000000;\">)</span><span style=\"color:#f92672;\">/$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">BUILD_ARTEFACTS_PATH</span><span style=\"color:#000000;\">)</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">      skip_app_build: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#000000;\">      skip_api_build: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#000000;\">      verbose: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#000000;\">      azure_static_web_apps_api_token: </span><span style=\"color:#f92672;\">&#39;$</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">DEPLOYMENT_TOKEN</span><span style=\"color:#000000;\">)</span><span style=\"color:#f92672;\">&#39;</span></pre>\n" },
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            strong { "Explanation:" }
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                code { "FlutterInstall@0" }
                ": Installs Flutter on the build agent."
            }
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                code { "FlutterCommand@0" }
                ": Builds the web app with the specified parameters, generating optimized files for deployment."
            }
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                code { "AzureStaticWebApp@0" }
                ": Deploys the built Flutter web app to Azure Static Web Apps using a deployment token you find bellow."
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            code { "FlutterInstall@0" }
            ",  "
            code { "FlutterCommand@0" }
            " steps ensure that Flutter web app is built in a consistent variables, with all dependencie managed by the pipeline."
        }
        h2 {
            id: "step-3-create-azure-static-web-app",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#step-3-create-azure-static-web-app", "Step 3: Create Azure Static Web App" }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "After building Flutter web app, the next step is to deploy it to Azure Static Web Apps:"
        }
        ol { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                "Go to the "
                a {
                    href: "https://portal.azure.com/",
                    class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                    class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                    "Azure Portal"
                }
                "."
            }
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                "Search for "
                strong { "Azure Static Web Apps" }
                " and click "
                strong { "Create" }
                "."
            }
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                "Enter the subscription, resource group, and app name details. Under "
                strong { "Deployment details" }
                ", select "
                strong { "Other" }
                " to integrate with Azure DevOps instead of GitHub."
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            img {
                src: "https://media.licdn.com/dms/image/v2/D4E12AQHMbx7zMSQB2A/article-inline_image-shrink_1000_1488/article-inline_image-shrink_1000_1488/0/1731267908639?e=1737590400&v=beta&t=nLgJsthlnG4beusbCvYHU-qaBDrgAfznOQ-BOItx9E0",
                alt: "Hosting Plan",
                class: "rounded-lg",
                class: if dark_mode().0 { "bg-neutral-800" } else { "bg-gray-100" },
                title: "",
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "After creating the resource, you'll need to press Manage deployment token and copy the deployment token for use in the deployment script. This token allows Azure DevOps to authenticate and push the build artifacts. See,  "
            code { "AzureStaticWebApp@0" }
            " line  "
            code { "azure_static_web_apps_api_token: '$(DEPLOYMENT_TOKEN)'" }
            "."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            img {
                src: "https://media.licdn.com/dms/image/v2/D4E12AQHLUGgh31H-dA/article-inline_image-shrink_1000_1488/article-inline_image-shrink_1000_1488/0/1731265410558?e=1737590400&v=beta&t=_rmJGk171mAkm1eq94uBkaPq8NvpS-4MDcMBdyfnF6w",
                alt: "Manage Deployment Token",
                class: "rounded-lg",
                class: if dark_mode().0 { "bg-neutral-800" } else { "bg-gray-100" },
                title: "",
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "You can now run the pipeline and should see results similar to this."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            img {
                src: "https://media.licdn.com/dms/image/v2/D4E12AQEj7hOZWEOlaw/article-inline_image-shrink_1000_1488/article-inline_image-shrink_1000_1488/0/1731313761489?e=1737590400&v=beta&t=2I3U0q_dT6a7VoLdfCsPZfBr-CUQpEdE0RtPX1QZb8U",
                alt: "Pipeline Results ",
                class: "rounded-lg",
                class: if dark_mode().0 { "bg-neutral-800" } else { "bg-gray-100" },
                title: "",
            }
        }
        h2 {
            id: "step-4-handling-cors-issues",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#step-4-handling-cors-issues", "Step 4: Handling CORS Issues" }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "A common challenge when hosting web apps is "
            strong { "Cross-Origin Resource Sharing (CORS)" }
            ". Browsers enforce CORS to restrict how resources are accessed across different domains, which can lead to errors when frontend makes requests to an API hosted on a different domain."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Since you cannot disable CORS in a Flutter app directly, consider these options:"
        }
        ol { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                "Use a Proxy Server"
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Configure a proxy server to forward API requests, effectively bypassing CORS restrictions. In Azure Static Web Apps, this can be done using a "
            a {
                href: "https://learn.microsoft.com/en-us/azure/static-web-apps/configuration",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "staticwebapp.config.json"
            }
            " file in "
            code { "$(PROJECT_DIRECTORY)/web" }
            " directory like this:"
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#e6db74;\">&quot;routes&quot;</span><span style=\"color:#f8f8f2;\">: [\n</span><span style=\"color:#f8f8f2;\">    {{\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#e6db74;\">&quot;route&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#e6db74;\">&quot;/api/*&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#e6db74;\">&quot;allowedRoles&quot;</span><span style=\"color:#f8f8f2;\">: [</span><span style=\"color:#e6db74;\">&quot;anonymous&quot;</span><span style=\"color:#f8f8f2;\">],\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#e6db74;\">&quot;rewrite&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#e6db74;\">&quot;https://{{server-name}}/api/*&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">  ]\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"color:#000000;\">{{\n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f25a00;\">&quot;routes&quot;</span><span style=\"color:#000000;\">: [\n</span><span style=\"color:#000000;\">    {{\n</span><span style=\"color:#000000;\">      </span><span style=\"color:#f25a00;\">&quot;route&quot;</span><span style=\"color:#000000;\">: </span><span style=\"color:#f25a00;\">&quot;/api/*&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">      </span><span style=\"color:#f25a00;\">&quot;allowedRoles&quot;</span><span style=\"color:#000000;\">: [</span><span style=\"color:#f25a00;\">&quot;anonymous&quot;</span><span style=\"color:#000000;\">],\n</span><span style=\"color:#000000;\">      </span><span style=\"color:#f25a00;\">&quot;rewrite&quot;</span><span style=\"color:#000000;\">: </span><span style=\"color:#f25a00;\">&quot;https://{{server-name}}/api/*&quot;\n</span><span style=\"color:#000000;\">    }}\n</span><span style=\"color:#000000;\">  ]\n</span><span style=\"color:#000000;\">}}</span></pre>\n" },
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "This allows requests to "
            em { "/api/" }
            " ** to be proxied to the backend server, avoiding CORS issues."
        }
        ol { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                "CORS Proxy Service"
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "For development purposes, you can use a third-party CORS proxy like cors-anywhere, though it's not recommended for production due to security concerns. You can also pass  "
            code { "BASE_API=https://cors-anywhere.herokuapp.com/https://{{server-name}}/api/" }
            " to the Azure pipeline to bypass CORS restrictions."
        }
        ol { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                "Local Reverse Proxy for Development"
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Use tools like "
            strong { "http-proxy-middleware" }
            " in a Node.js server or configure Nginx to create a reverse proxy that forwards requests to the backend. This allows you to develop locally without worrying about CORS issues."
        }
        h2 {
            id: "summary",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#summary", "Summary" }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "In this guide, we covered how to build and deploy a Flutter web app using Azure DevOps and Azure Static Web Apps. We discussed handling CORS issues, setting up build pipelines, and deploying efficiently."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Azure Static Web Apps is a solid option for scalable hosting with CI/CD integration, offering features like auto-scaling, easy integration with Azure Functions, and support for the free plan. By using it, you can deploy your Flutter app smoothly and make it accessible across different environments."
        }
    }
}
#[component(no_case_check)]
pub fn RustWebDioxus() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    let dark_mode = use_context::<Signal<DarkMode>>();
    rsx! {
        h1 {
            id: "-rust-experiments-webassembly-development",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#-rust-experiments-webassembly-development",
                "🦀 Rust Experiments: WebAssembly development"
            }
        }
        blockquote {
            p {
                class: "text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                "November 11, 2024"
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Rust has rapidly become one of the "
            a {
                href: "https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "most popular programming languages"
            }
            "  among developers. According to the "
            a {
                href: "https://stackoverflow.blog/2023/06/13/developer-survey-results-are-in/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Stack Overflow Developer Survey"
            }
            ", Rust has consistently ranked as one of the \"most loved\" programming languages for several years. This popularity can be attributed to its combination of high performance and memory safety, which sets it apart from many other languages. Additionally, Rust's use in critical, security-sensitive applications over the past few years has solidified its reputation as a reliable and powerful language."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Notable Examples of Rust Adoption:"
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Google's Shift to Rust Programming" }
                " "
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Google has adopted Rust for Android development, leading to a "
            a {
                href: "https://thehackernews.com/2024/09/googles-shift-to-rust-programming-cuts.html",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "68% reduction in memory vulnerabilities"
            }
            "."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Linux Kernel Integration" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Rust has been integrated into the Linux kernel since "
            a {
                href: "https://www.zdnet.com/article/linus-torvalds-rust-will-go-into-linux-6-1/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Linus Torvalds approved it for the Linux 6.1 release"
            }
            ", and it has continued "
            a {
                href: "https://www.zdnet.com/article/rust-in-linux-where-we-are-and-where-were-going-next/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "evolving in this role"
            }
            ". Additionally, "
            a {
                href: "https://system76.com/cosmic/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Pop!"
            }
            " has been implemented using Rust, showcasing its effectiveness in building reliable system components."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Microsoft's Windows Rewriting" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            a {
                href: "https://www.theregister.com/2023/04/27/microsoft_windows_rust/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Microsoft is rewriting core Windows components in memory-safe Rust"
            }
            ", aiming to enhance the security and stability of its operating system."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Project Caliptra" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Developed by NVIDIA, AMD, Google, and Microsoft, "
            a {
                href: "https://github.com/chipsalliance/Caliptra",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Project Caliptra"
            }
            " is a critical "
            a {
                href: "https://medium.com/@chanibonner/a-beginners-guide-to-root-of-trust-and-secure-boot-1ccf72b3e9b2",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Root of Trust"
            }
            " component. It provides the foundation for secure operations in computing systems by managing cryptographic keys and enabling secure boot processes. This initiative is expected to significantly impact "
            a {
                href: "https://www.opencompute.org/blog/cloud-security-integrating-trust-into-every-chip",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "data center solutions"
            }
            " and "
            a {
                href: "https://azure.microsoft.com/en-us/blog/fostering-ai-infrastructure-advancements-through-standardization/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "AI infrastructure advancements"
            }
            ". Google and Microsoft are planning to implement Caliptra in their "
            a {
                href: "https://semianalysis.com/2022/10/25/caliptra-first-open-source-silicon/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "silicon in 2024"
            }
            ", while "
            a {
                href: "https://community.amd.com/t5/corporate/addressing-security-integrating-project-caliptra-into-amd-s/ba-p/716837",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "AMD targets 2026+"
            }
            "."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Mozilla's Gecko Engine" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Rust has been integrated into "
            a {
                href: "https://firefox-source-docs.mozilla.org/testing-rust-code/index.html",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Mozilla's Gecko"
            }
            " browser engine, enhancing memory safety and performance in web browsing."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Automotive Industry Adoption" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Rust has been "
            a {
                href: "https://tweedegolf.nl/en/blog/137/rust-is-rolling-off-the-volvo-assembly-line",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "adopted by Volvo for its production lines"
            }
            ", and it is seen as the "
            a {
                href: "https://www.linkedin.com/pulse/power-rust-future-automotive-software-development-acsiatech/?lipi=urn%3Ali%3Apage%3Ad_flagship3_pulse_read%3BQIwogWXUT%2F6vQxKinbW7Hg%3D%3D",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "future of automotive software development"
            }
            ", providing reliability for safety-critical systems."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "AI Infrastructure" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "xAI, a company led by Elon Musk behind the "
            a {
                href: "https://x.ai/blog/grok",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Grok AI system"
            }
            ", has built its infrastructure using Rust. "
            a {
                href: "https://huggingface.co/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Hugging Face"
            }
            ", one of the largest AI communities, also "
            a {
                href: "https://github.com/huggingface",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "uses Rust components"
            }
            ", demonstrating the language's suitability for high-performance AI applications."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Blockchain Development" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Rust is widely used in blockchain development. Notable projects include the "
            a {
                href: "https://github.com/foundry-rs",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Foundry toolkit for Ethereum applications"
            }
            ", "
            a {
                href: "https://github.com/lightningdevkit",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Lightning Dev Kit for Bitcoin"
            }
            ", "
            a {
                href: "https://github.com/paritytech/polkadot-sdk",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Parity Polkadot SDK"
            }
            ", and the "
            a {
                href: "https://solana.com/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Solana blockchain"
            }
            "."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Tor and Privacy Tools" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Rust is being used to implement the "
            a {
                href: "https://www.torproject.org/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Tor"
            }
            " anonymity protocols in the "
            a {
                href: "https://tpo.pages.torproject.net/core/arti/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Arti Project"
            }
            ", as well as in other privacy tools such as "
            a {
                href: "https://1password.com/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "1Password"
            }
            ", "
            a {
                href: "https://firecracker-microvm.github.io/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Firecracker"
            }
            ", and "
            a {
                href: "https://www.cloudflare.com/en-gb/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Cloudflare"
            }
            "."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Aerospace Community" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Rust also has an active aerospace community focusing on using Rust in aerospace applications ("
            a {
                href: "https://aerorust.org/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "AeroRust"
            }
            ")."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Microsoft Azure Endorsement" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Microsoft Azure CTO Mark Russinovich has endorsed using Rust to avoid memory safety vulnerabilities, recognizing its benefits over traditional languages like C and C++. "
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            img {
                src: "https://media.licdn.com/dms/image/v2/D4E12AQEn-3KSyCPZKw/article-inline_image-shrink_1500_2232/article-inline_image-shrink_1500_2232/0/1731347766205?e=1737590400&v=beta&t=XoS67fQxgn9K0L7uWWIFK7v__nCwzG0SvypWmKw8rM4",
                alt: "Microsoft Azure CTO Mark Russinovich",
                class: "rounded-lg",
                class: if dark_mode().0 { "bg-neutral-800" } else { "bg-gray-100" },
                title: "",
            }
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "DARPA Initiative" }
                " "
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "The U.S. government, through "
            a {
                href: "https://www.darpa.mil/program/translating-all-c-to-rust",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "DARPA, plans to translate legacy C code to Rust"
            }
            " to enhance system security."
        }
        h2 {
            id: "web-development-with-rust-and-webassembly",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#web-development-with-rust-and-webassembly",
                "Web Development with Rust and WebAssembly"
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Rust has rapidly become a go-to language for native web development, especially through "
            a {
                href: "https://developer.mozilla.org/en-US/docs/WebAssembly",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "WebAssembly (Wasm)"
            }
            ". WebAssembly, co-developed by Mozilla, allows developers to compile Rust code that runs at near-native performance in a web environment while preserving Rust's memory safety features. This makes Rust an excellent choice for "
            a {
                href: "https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "building fast, secure web applications"
            }
            "."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "The Rust ecosystem includes several frameworks that make web development efficient and enjoyable. Below are some of the most popular frameworks for Rust web development: "
            a {
                href: "https://dioxuslabs.com/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Dioxus"
            }
            ", "
            a {
                href: "https://leptos.dev/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Leptos"
            }
            ", "
            a {
                href: "https://yew.rs/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Yew"
            }
            ", "
            a {
                href: "https://github.com/seed-rs/seed",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Seed"
            }
            ", etc. For a detailed comparison of these frameworks, refer to this "
            a {
                href: "https://krausest.github.io/js-framework-benchmark/2024/table_chrome_129.0.6668.58.html",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "comparison table"
            }
            ", which outlines metrics like rendering speed, build size, and transferred size. "
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Among the listed frameworks, "
            a {
                href: "https://dioxuslabs.com/learn/0.6/#introduction",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Dioxus aims to provide a unified development experience across web, desktop, and mobile platforms"
            }
            ". It has features like live hot reloading, server functions, and easy deployment. Its React-inspired design also makes it easy for developers familiar with "
            a {
                href: "https://www.linkedin.com/redir/invalid-link-page?url=https%3A%2F%2Freact%2edev%2F&lipi=urn%3Ali%3Apage%3Ad_flagship3_pulse_read%3BQIwogWXUT%2F6vQxKinbW7Hg%3D%3D",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "React"
            }
            " to adopt. Also, you can learn more about their plans for native support on "
            a {
                href: "https://github.com/dioxuslabs/dioxus/milestones",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "the official roadmap page"
            }
            "."
        }
        h2 {
            id: "component-in-dioxus",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#component-in-dioxus", "Component in Dioxus" }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "We won't cover the initial setup and first component creation, as you can find a comprehensive guide on the "
            a {
                href: "https://dioxuslabs.com/learn/0.6/guide/your_first_component",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Dioxus documentation site"
            }
            " along with "
            a {
                href: "https://dioxuslabs.com/learn/0.6/router/example/full-code",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "the full code"
            }
            "."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Since Dioxus supports styling applications using "
            a {
                href: "https://dioxuslabs.com/learn/0.6/cookbook/tailwind",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Tailwind CSS"
            }
            " framework or vanilla CSS, we’ll use "
            a {
                href: "https://tailwindcss.com/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Tailwind"
            }
            ", specifically, "
            a {
                href: "https://www.preline.co/examples/navigations-navbars.html#mini-floating-header",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Preline UI's Mini Floating Header"
            }
            " as our example."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Converting HTML to RSX with dx translate" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Dioxus uses RSX to represent HTML in Rust code. RSX is concise and idiomatic, but converting HTML manually can be tedious. To streamline this process, Dioxus provides the dx translate command-line tool: "
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"color:#f92672;\">$</span><span style=\"color:#f8f8f2;\"> dx translate </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">file index.html</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"color:#f92672;\">$</span><span style=\"color:#000000;\"> dx translate </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">file index.html</span></pre>\n" },
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "This command makes it easy to take HTML and convert it into RSX for further customization in Dioxus."
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                strong { "Optimizing and Adding Interactivity to Our Component" }
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Once we have the RSX, we can optimize our code by using "
            a {
                href: "https://dioxuslabs.com/learn/0.6/migration/props#manual-props",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Manual Props"
            }
            " and create a NavBar component. Props allow to make components more flexible and reusable. For "
            strong { "NavBar" }
            ", we’ll add parameters for the navigation items, dark mode, and brand link. We'll also set up "
            a {
                href: "https://dioxuslabs.com/learn/0.6/reference/event_handlers",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "event handlers"
            }
            " for user interactions, such as clicking on navigation links or toggling the mobile menu."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "To simplify the implementation, we use the following helper crates:"
        }
        ul { class: "list-disc ms-6 mt-3 space-y-1.5",
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                code { "dioxus-spring = \"0.2.1\"" }
                ": An animation library for Dioxus. More details are available "
                a {
                    href: "https://github.com/dioxus-community/dioxus-spring",
                    class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                    class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                    "here"
                }
                "."
            }
            li {
                class: "ps-1 text-sm",
                class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
                code { "dioxus-use-mounted = \"0.2.1\"" }
                ": A utility to encapsulate component "
                code { "MountedData" }
                ", which you can find "
                a {
                    href: "https://github.com/dioxus-community/dioxus-use-mounted/blob/main/src/lib.rs",
                    class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                    class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                    "here"
                }
                "."
            }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Here’s the complete implementation of the NavBar component:"
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"color:#f8f8f2;\">#[derive(Copy, Clone, PartialEq)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">NavItem {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">id: </span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">title: </span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">href: </span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">section: UseMounted,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone, PartialEq)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">NavBarProps {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">darkmode: </span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">menu: Vec&lt;NavItem&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">selected: Option&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[props(into)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">brandlink: NavigationTarget,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">icon: Option&lt;Element&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NavBar</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">props</span><span style=\"color:#f8f8f2;\">: NavBarProps) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> selected_index </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| props.selected.</span><span style=\"color:#66d9ef;\">unwrap_or</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">0</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> height </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ae81ff;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f8f8f2;\">(spring, spring_ref) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_spring_signal</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">height</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        spring_ref.</span><span style=\"color:#66d9ef;\">animate</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">height</span><span style=\"color:#f8f8f2;\">(), Duration::from_millis(</span><span style=\"color:#ae81ff;\">300</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> container_ref </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_mounted</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        nav {{ class: format_args!(</span><span style=\"color:#e6db74;\">&quot;mt-4 relative max-w-2xl w-full mx-2 py-2.5 md:flex md:items-center md:justify-between md:py-0 md:px-4 md:mx-auto border rounded-[2rem] {{}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> props.darkmode {{ </span><span style=\"color:#e6db74;\">&quot;bg-neutral-900 border-neutral-700&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#e6db74;\">&quot;bg-white border-gray-200&quot; </span><span style=\"color:#f8f8f2;\">}}),\n</span><span style=\"color:#f8f8f2;\">            div {{ class: </span><span style=\"color:#e6db74;\">&quot;px-4 md:px-0 flex justify-between items-center&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                div {{  class: </span><span style=\"color:#e6db74;\">&quot;shrink-0&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    Link {{ class: </span><span style=\"color:#e6db74;\">&quot;flex-none rounded-md inline-block text-xl font-semibold focus:outline-none focus:opacity-80&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                        to: props.brandlink,\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(icon) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> props.icon {{ {{icon}} }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ icons::</span><span style=\"color:#ae81ff;\">AT </span><span style=\"color:#f8f8f2;\">{{ darkmode: props.darkmode, class: </span><span style=\"color:#e6db74;\">&quot;mx-2 justify-center&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">                div {{ class: </span><span style=\"color:#e6db74;\">&quot;md:hidden&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    button {{ class: format_args!(</span><span style=\"color:#e6db74;\">&quot;flex justify-center items-center size-6 border rounded-full focus:outline-none {{}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> props.darkmode {{ </span><span style=\"color:#e6db74;\">&quot;border-neutral-700 text-neutral-400 hover:bg-neutral-700 focus:bg-neutral-700&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#e6db74;\">&quot;border-gray-200 text-gray-500 hover:bg-gray-200 focus:bg-gray-200&quot; </span><span style=\"color:#f8f8f2;\">}}),\n</span><span style=\"color:#f8f8f2;\">                        r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#e6db74;\">&quot;button&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                        onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> height.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">height</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ae81ff;\">0 </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ae81ff;\">0 </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#66d9ef;\">get_scroll_height</span><span style=\"color:#f8f8f2;\">(container_ref).</span><span style=\"color:#66d9ef;\">unwrap_or</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ae81ff;\">0</span><span style=\"color:#f8f8f2;\">) }}),\n</span><span style=\"color:#f8f8f2;\">                        svg {{\n</span><span style=\"color:#f8f8f2;\">                            width: </span><span style=\"color:#e6db74;\">&quot;24&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            stroke: </span><span style=\"color:#e6db74;\">&quot;currentColor&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            fill: </span><span style=\"color:#e6db74;\">&quot;none&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            height: </span><span style=\"color:#e6db74;\">&quot;24&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            view_box: </span><span style=\"color:#e6db74;\">&quot;0 0 24 24&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            stroke_width: </span><span style=\"color:#e6db74;\">&quot;2&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            stroke_linecap: </span><span style=\"color:#e6db74;\">&quot;round&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            stroke_linejoin: </span><span style=\"color:#e6db74;\">&quot;round&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            class: </span><span style=\"color:#e6db74;\">&quot;shrink-0 size-3.5&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">height</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ae81ff;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                                path {{ d: </span><span style=\"color:#e6db74;\">&quot;M18 6 6 18&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                                path {{ d: </span><span style=\"color:#e6db74;\">&quot;m6 6 12 12&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                            }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                                line {{ y2: </span><span style=\"color:#e6db74;\">&quot;6&quot;</span><span style=\"color:#f8f8f2;\">, x2: </span><span style=\"color:#e6db74;\">&quot;21&quot;</span><span style=\"color:#f8f8f2;\">, x1: </span><span style=\"color:#e6db74;\">&quot;3&quot;</span><span style=\"color:#f8f8f2;\">, y1: </span><span style=\"color:#e6db74;\">&quot;6&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                                line {{ y1: </span><span style=\"color:#e6db74;\">&quot;12&quot;</span><span style=\"color:#f8f8f2;\">, x1: </span><span style=\"color:#e6db74;\">&quot;3&quot;</span><span style=\"color:#f8f8f2;\">, y2: </span><span style=\"color:#e6db74;\">&quot;12&quot;</span><span style=\"color:#f8f8f2;\">, x2: </span><span style=\"color:#e6db74;\">&quot;21&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                                line {{ y1: </span><span style=\"color:#e6db74;\">&quot;18&quot;</span><span style=\"color:#f8f8f2;\">, x2: </span><span style=\"color:#e6db74;\">&quot;21&quot;</span><span style=\"color:#f8f8f2;\">, y2: </span><span style=\"color:#e6db74;\">&quot;18&quot;</span><span style=\"color:#f8f8f2;\">, x1: </span><span style=\"color:#e6db74;\">&quot;3&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                            }}\n</span><span style=\"color:#f8f8f2;\">                        }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            div {{ class: format_args!(</span><span style=\"color:#e6db74;\">&quot;basis-full grow overflow-hidden md:block {{}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">spring</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ae81ff;\">0 </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#e6db74;\">&quot;hidden&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#e6db74;\">&quot;&quot; </span><span style=\"color:#f8f8f2;\">}}),\n</span><span style=\"color:#f8f8f2;\">                style: format_args!(</span><span style=\"color:#e6db74;\">&quot;{{}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">height</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">!= </span><span style=\"color:#66d9ef;\">spring</span><span style=\"color:#f8f8f2;\">() {{ format!(</span><span style=\"color:#e6db74;\">&quot;height: </span><span style=\"color:#ae81ff;\">{{}}</span><span style=\"color:#e6db74;\">px;&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#66d9ef;\">spring</span><span style=\"color:#f8f8f2;\">()) }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#e6db74;\">&quot;&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">() }}),\n</span><span style=\"color:#f8f8f2;\">                onmounted: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> container_ref.</span><span style=\"color:#66d9ef;\">onmounted</span><span style=\"color:#f8f8f2;\">(event),\n</span><span style=\"color:#f8f8f2;\">                div {{ class: </span><span style=\"color:#e6db74;\">&quot;flex flex-col md:flex-row md:items-center md:justify-end gap-2 md:gap-3 mt-3 md:mt-0 py-2 md:py-0 md:ps-7&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">(i, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">nav) </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> props.menu.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">enumerate</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">                        Link {{ class: format_args!(</span><span style=\"color:#e6db74;\">&quot;py-0.5 md:py-3 px-4 md:px-1 border-s-2 md:border-s-0 md:border-b-2 focus:outline-none {{}} {{}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> props.darkmode {{ </span><span style=\"color:#e6db74;\">&quot;border-neutral-200 text-neutral-200&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#e6db74;\">&quot;border-gray-800 text-gray-800&quot; </span><span style=\"color:#f8f8f2;\">}}, </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">selected_index</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">==</span><span style=\"color:#f8f8f2;\"> i {{ </span><span style=\"color:#e6db74;\">&quot;font-medium&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#e6db74;\">&quot;border-transparent&quot; </span><span style=\"color:#f8f8f2;\">}}),\n</span><span style=\"color:#f8f8f2;\">                            to: nav.href,\n</span><span style=\"color:#f8f8f2;\">                            onclick_only: </span><span style=\"color:#ae81ff;\">true</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                                selected_index.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(i);\n</span><span style=\"color:#f8f8f2;\">                                </span><span style=\"color:#66d9ef;\">scroll_into_view</span><span style=\"color:#f8f8f2;\">(nav.section);\n</span><span style=\"color:#f8f8f2;\">                            }},\n</span><span style=\"color:#f8f8f2;\">                            {{nav.title}}\n</span><span style=\"color:#f8f8f2;\">                        }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"color:#000000;\">#[derive(Copy, Clone, PartialEq)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#28c6e4;\">struct </span><span style=\"color:#000000;\">NavItem {{\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#000000;\">id: </span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#28c6e4;\">str</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#000000;\">title: </span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#28c6e4;\">str</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#000000;\">href: </span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#28c6e4;\">str</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#000000;\">section: UseMounted,\n</span><span style=\"color:#000000;\">}}\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">#[derive(Props, Clone, PartialEq)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#28c6e4;\">struct </span><span style=\"color:#000000;\">NavBarProps {{\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#000000;\">darkmode: </span><span style=\"font-style:italic;color:#28c6e4;\">bool</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#000000;\">menu: Vec&lt;NavItem&gt;,\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#000000;\">selected: Option&lt;</span><span style=\"font-style:italic;color:#28c6e4;\">usize</span><span style=\"color:#000000;\">&gt;,\n</span><span style=\"color:#000000;\">    #[props(into)]\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#000000;\">brandlink: NavigationTarget,\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#000000;\">icon: Option&lt;Element&gt;,\n</span><span style=\"color:#000000;\">}}\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">#[component]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#28c6e4;\">fn </span><span style=\"color:#6aaf19;\">NavBar</span><span style=\"color:#000000;\">(</span><span style=\"font-style:italic;color:#fd971f;\">props</span><span style=\"color:#000000;\">: NavBarProps) -&gt; Element {{\n</span><span style=\"color:#000000;\">    </span><span style=\"font-style:italic;color:#28c6e4;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#000000;\"> selected_index </span><span style=\"color:#f92672;\">= </span><span style=\"color:#28c6e4;\">use_signal</span><span style=\"color:#000000;\">(|| props.selected.</span><span style=\"color:#28c6e4;\">unwrap_or</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">0</span><span style=\"color:#000000;\">));\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">    </span><span style=\"font-style:italic;color:#28c6e4;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#000000;\"> height </span><span style=\"color:#f92672;\">= </span><span style=\"color:#28c6e4;\">use_signal</span><span style=\"color:#000000;\">(|| </span><span style=\"color:#ae81ff;\">0</span><span style=\"color:#000000;\">);\n</span><span style=\"color:#000000;\">    </span><span style=\"font-style:italic;color:#28c6e4;\">let </span><span style=\"color:#000000;\">(spring, spring_ref) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#28c6e4;\">use_spring_signal</span><span style=\"color:#000000;\">(</span><span style=\"color:#28c6e4;\">height</span><span style=\"color:#000000;\">());\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#28c6e4;\">use_memo</span><span style=\"color:#000000;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#000000;\">{{\n</span><span style=\"color:#000000;\">        spring_ref.</span><span style=\"color:#28c6e4;\">animate</span><span style=\"color:#000000;\">(</span><span style=\"color:#28c6e4;\">height</span><span style=\"color:#000000;\">(), Duration::from_millis(</span><span style=\"color:#ae81ff;\">300</span><span style=\"color:#000000;\">));\n</span><span style=\"color:#000000;\">    }});\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">    </span><span style=\"font-style:italic;color:#28c6e4;\">let</span><span style=\"color:#000000;\"> container_ref </span><span style=\"color:#f92672;\">= </span><span style=\"color:#28c6e4;\">use_mounted</span><span style=\"color:#000000;\">();\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">    rsx! {{\n</span><span style=\"color:#000000;\">        nav {{ class: format_args!(</span><span style=\"color:#f25a00;\">&quot;mt-4 relative max-w-2xl w-full mx-2 py-2.5 md:flex md:items-center md:justify-between md:py-0 md:px-4 md:mx-auto border rounded-[2rem] {{}}&quot;</span><span style=\"color:#000000;\">, </span><span style=\"color:#f92672;\">if</span><span style=\"color:#000000;\"> props.darkmode {{ </span><span style=\"color:#f25a00;\">&quot;bg-neutral-900 border-neutral-700&quot; </span><span style=\"color:#000000;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#000000;\">{{ </span><span style=\"color:#f25a00;\">&quot;bg-white border-gray-200&quot; </span><span style=\"color:#000000;\">}}),\n</span><span style=\"color:#000000;\">            div {{ class: </span><span style=\"color:#f25a00;\">&quot;px-4 md:px-0 flex justify-between items-center&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                div {{  class: </span><span style=\"color:#f25a00;\">&quot;shrink-0&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                    Link {{ class: </span><span style=\"color:#f25a00;\">&quot;flex-none rounded-md inline-block text-xl font-semibold focus:outline-none focus:opacity-80&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                        to: props.brandlink,\n</span><span style=\"color:#000000;\">                        </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#28c6e4;\">let Some</span><span style=\"color:#000000;\">(icon) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#000000;\"> props.icon {{ {{icon}} }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#000000;\">{{ icons::</span><span style=\"color:#ae81ff;\">AT </span><span style=\"color:#000000;\">{{ darkmode: props.darkmode, class: </span><span style=\"color:#f25a00;\">&quot;mx-2 justify-center&quot; </span><span style=\"color:#000000;\">}} }}\n</span><span style=\"color:#000000;\">                    }}\n</span><span style=\"color:#000000;\">                }}\n</span><span style=\"color:#000000;\">                div {{ class: </span><span style=\"color:#f25a00;\">&quot;md:hidden&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                    button {{ class: format_args!(</span><span style=\"color:#f25a00;\">&quot;flex justify-center items-center size-6 border rounded-full focus:outline-none {{}}&quot;</span><span style=\"color:#000000;\">, </span><span style=\"color:#f92672;\">if</span><span style=\"color:#000000;\"> props.darkmode {{ </span><span style=\"color:#f25a00;\">&quot;border-neutral-700 text-neutral-400 hover:bg-neutral-700 focus:bg-neutral-700&quot; </span><span style=\"color:#000000;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#000000;\">{{ </span><span style=\"color:#f25a00;\">&quot;border-gray-200 text-gray-500 hover:bg-gray-200 focus:bg-gray-200&quot; </span><span style=\"color:#000000;\">}}),\n</span><span style=\"color:#000000;\">                        r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#28c6e4;\">type</span><span style=\"color:#000000;\">: </span><span style=\"color:#f25a00;\">&quot;button&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                        onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#000000;\"> height.</span><span style=\"color:#28c6e4;\">set</span><span style=\"color:#000000;\">(</span><span style=\"color:#f92672;\">if </span><span style=\"color:#28c6e4;\">height</span><span style=\"color:#000000;\">() </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ae81ff;\">0 </span><span style=\"color:#000000;\">{{ </span><span style=\"color:#ae81ff;\">0 </span><span style=\"color:#000000;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#000000;\">{{ </span><span style=\"color:#28c6e4;\">get_scroll_height</span><span style=\"color:#000000;\">(container_ref).</span><span style=\"color:#28c6e4;\">unwrap_or</span><span style=\"color:#000000;\">(</span><span style=\"color:#ae81ff;\">0</span><span style=\"color:#000000;\">) }}),\n</span><span style=\"color:#000000;\">                        svg {{\n</span><span style=\"color:#000000;\">                            width: </span><span style=\"color:#f25a00;\">&quot;24&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            stroke: </span><span style=\"color:#f25a00;\">&quot;currentColor&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            fill: </span><span style=\"color:#f25a00;\">&quot;none&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            height: </span><span style=\"color:#f25a00;\">&quot;24&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            view_box: </span><span style=\"color:#f25a00;\">&quot;0 0 24 24&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            stroke_width: </span><span style=\"color:#f25a00;\">&quot;2&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            stroke_linecap: </span><span style=\"color:#f25a00;\">&quot;round&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            stroke_linejoin: </span><span style=\"color:#f25a00;\">&quot;round&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            class: </span><span style=\"color:#f25a00;\">&quot;shrink-0 size-3.5&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            </span><span style=\"color:#f92672;\">if </span><span style=\"color:#28c6e4;\">height</span><span style=\"color:#000000;\">() </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ae81ff;\">0 </span><span style=\"color:#000000;\">{{\n</span><span style=\"color:#000000;\">                                path {{ d: </span><span style=\"color:#f25a00;\">&quot;M18 6 6 18&quot; </span><span style=\"color:#000000;\">}}\n</span><span style=\"color:#000000;\">                                path {{ d: </span><span style=\"color:#f25a00;\">&quot;m6 6 12 12&quot; </span><span style=\"color:#000000;\">}}\n</span><span style=\"color:#000000;\">                            }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#000000;\">{{\n</span><span style=\"color:#000000;\">                                line {{ y2: </span><span style=\"color:#f25a00;\">&quot;6&quot;</span><span style=\"color:#000000;\">, x2: </span><span style=\"color:#f25a00;\">&quot;21&quot;</span><span style=\"color:#000000;\">, x1: </span><span style=\"color:#f25a00;\">&quot;3&quot;</span><span style=\"color:#000000;\">, y1: </span><span style=\"color:#f25a00;\">&quot;6&quot; </span><span style=\"color:#000000;\">}}\n</span><span style=\"color:#000000;\">                                line {{ y1: </span><span style=\"color:#f25a00;\">&quot;12&quot;</span><span style=\"color:#000000;\">, x1: </span><span style=\"color:#f25a00;\">&quot;3&quot;</span><span style=\"color:#000000;\">, y2: </span><span style=\"color:#f25a00;\">&quot;12&quot;</span><span style=\"color:#000000;\">, x2: </span><span style=\"color:#f25a00;\">&quot;21&quot; </span><span style=\"color:#000000;\">}}\n</span><span style=\"color:#000000;\">                                line {{ y1: </span><span style=\"color:#f25a00;\">&quot;18&quot;</span><span style=\"color:#000000;\">, x2: </span><span style=\"color:#f25a00;\">&quot;21&quot;</span><span style=\"color:#000000;\">, y2: </span><span style=\"color:#f25a00;\">&quot;18&quot;</span><span style=\"color:#000000;\">, x1: </span><span style=\"color:#f25a00;\">&quot;3&quot; </span><span style=\"color:#000000;\">}}\n</span><span style=\"color:#000000;\">                            }}\n</span><span style=\"color:#000000;\">                        }}\n</span><span style=\"color:#000000;\">                    }}\n</span><span style=\"color:#000000;\">                }}\n</span><span style=\"color:#000000;\">            }}\n</span><span style=\"color:#000000;\">            div {{ class: format_args!(</span><span style=\"color:#f25a00;\">&quot;basis-full grow overflow-hidden md:block {{}}&quot;</span><span style=\"color:#000000;\">, </span><span style=\"color:#f92672;\">if </span><span style=\"color:#28c6e4;\">spring</span><span style=\"color:#000000;\">() </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ae81ff;\">0 </span><span style=\"color:#000000;\">{{ </span><span style=\"color:#f25a00;\">&quot;hidden&quot; </span><span style=\"color:#000000;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#000000;\">{{ </span><span style=\"color:#f25a00;\">&quot;&quot; </span><span style=\"color:#000000;\">}}),\n</span><span style=\"color:#000000;\">                style: format_args!(</span><span style=\"color:#f25a00;\">&quot;{{}}&quot;</span><span style=\"color:#000000;\">, </span><span style=\"color:#f92672;\">if </span><span style=\"color:#28c6e4;\">height</span><span style=\"color:#000000;\">() </span><span style=\"color:#f92672;\">!= </span><span style=\"color:#28c6e4;\">spring</span><span style=\"color:#000000;\">() {{ format!(</span><span style=\"color:#f25a00;\">&quot;height: </span><span style=\"color:#ae81ff;\">{{}}</span><span style=\"color:#f25a00;\">px;&quot;</span><span style=\"color:#000000;\">, </span><span style=\"color:#28c6e4;\">spring</span><span style=\"color:#000000;\">()) }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#000000;\">{{ </span><span style=\"color:#f25a00;\">&quot;&quot;</span><span style=\"color:#000000;\">.</span><span style=\"color:#28c6e4;\">into</span><span style=\"color:#000000;\">() }}),\n</span><span style=\"color:#000000;\">                onmounted: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#000000;\">event</span><span style=\"color:#f92672;\">|</span><span style=\"color:#000000;\"> container_ref.</span><span style=\"color:#28c6e4;\">onmounted</span><span style=\"color:#000000;\">(event),\n</span><span style=\"color:#000000;\">                div {{ class: </span><span style=\"color:#f25a00;\">&quot;flex flex-col md:flex-row md:items-center md:justify-end gap-2 md:gap-3 mt-3 md:mt-0 py-2 md:py-0 md:ps-7&quot;</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                    </span><span style=\"color:#f92672;\">for </span><span style=\"color:#000000;\">(i, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#000000;\">nav) </span><span style=\"color:#f92672;\">in</span><span style=\"color:#000000;\"> props.menu.</span><span style=\"color:#28c6e4;\">iter</span><span style=\"color:#000000;\">().</span><span style=\"color:#28c6e4;\">enumerate</span><span style=\"color:#000000;\">() {{\n</span><span style=\"color:#000000;\">                        Link {{ class: format_args!(</span><span style=\"color:#f25a00;\">&quot;py-0.5 md:py-3 px-4 md:px-1 border-s-2 md:border-s-0 md:border-b-2 focus:outline-none {{}} {{}}&quot;</span><span style=\"color:#000000;\">, </span><span style=\"color:#f92672;\">if</span><span style=\"color:#000000;\"> props.darkmode {{ </span><span style=\"color:#f25a00;\">&quot;border-neutral-200 text-neutral-200&quot; </span><span style=\"color:#000000;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#000000;\">{{ </span><span style=\"color:#f25a00;\">&quot;border-gray-800 text-gray-800&quot; </span><span style=\"color:#000000;\">}}, </span><span style=\"color:#f92672;\">if </span><span style=\"color:#28c6e4;\">selected_index</span><span style=\"color:#000000;\">() </span><span style=\"color:#f92672;\">==</span><span style=\"color:#000000;\"> i {{ </span><span style=\"color:#f25a00;\">&quot;font-medium&quot; </span><span style=\"color:#000000;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#000000;\">{{ </span><span style=\"color:#f25a00;\">&quot;border-transparent&quot; </span><span style=\"color:#000000;\">}}),\n</span><span style=\"color:#000000;\">                            to: nav.href,\n</span><span style=\"color:#000000;\">                            onclick_only: </span><span style=\"color:#ae81ff;\">true</span><span style=\"color:#000000;\">,\n</span><span style=\"color:#000000;\">                            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#000000;\">{{\n</span><span style=\"color:#000000;\">                                selected_index.</span><span style=\"color:#28c6e4;\">set</span><span style=\"color:#000000;\">(i);\n</span><span style=\"color:#000000;\">                                </span><span style=\"color:#28c6e4;\">scroll_into_view</span><span style=\"color:#000000;\">(nav.section);\n</span><span style=\"color:#000000;\">                            }},\n</span><span style=\"color:#000000;\">                            {{nav.title}}\n</span><span style=\"color:#000000;\">                        }}\n</span><span style=\"color:#000000;\">                    }}\n</span><span style=\"color:#000000;\">                }}\n</span><span style=\"color:#000000;\">            }}\n</span><span style=\"color:#000000;\">        }}\n</span><span style=\"color:#000000;\">    }}\n</span><span style=\"color:#000000;\">}}</span></pre>\n" },
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "It refers a "
            strong { "scroll_into_view" }
            " function for smooth scrolling to a selected section and a "
            strong { "get_scroll_height" }
            " function to calculate the height for the mobile menu. Here's the implementation:"
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">scroll_into_view</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">mounted</span><span style=\"color:#f8f8f2;\">: UseMounted) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(element) </span><span style=\"color:#f92672;\">= &amp;*</span><span style=\"color:#f8f8f2;\">mounted.signal.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">_ =</span><span style=\"color:#f8f8f2;\"> element.</span><span style=\"color:#66d9ef;\">scroll_to</span><span style=\"color:#f8f8f2;\">(ScrollBehavior::Smooth);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_scroll_height</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">mounted</span><span style=\"color:#f8f8f2;\">: UseMounted) -&gt; Option&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(element) </span><span style=\"color:#f92672;\">= &amp;*</span><span style=\"color:#f8f8f2;\">mounted.signal.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(raw_elem) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> element.downcast::&lt;web_sys::Element&gt;() {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> att </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> raw_elem.</span><span style=\"color:#66d9ef;\">get_attribute</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#e6db74;\">&quot;style&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap_or_default</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">            raw_elem.</span><span style=\"color:#66d9ef;\">set_attribute</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#e6db74;\">&quot;style&quot;</span><span style=\"color:#f8f8f2;\">, format!(</span><span style=\"color:#e6db74;\">&quot;</span><span style=\"color:#ae81ff;\">{{}}</span><span style=\"color:#e6db74;\">;display: block;&quot;</span><span style=\"color:#f8f8f2;\">, att).</span><span style=\"color:#66d9ef;\">as_str</span><span style=\"color:#f8f8f2;\">()).</span><span style=\"color:#66d9ef;\">unwrap_or_default</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> height </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> raw_elem.</span><span style=\"color:#66d9ef;\">scroll_height</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">            raw_elem.</span><span style=\"color:#66d9ef;\">set_attribute</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#e6db74;\">&quot;style&quot;</span><span style=\"color:#f8f8f2;\">, att.</span><span style=\"color:#66d9ef;\">as_str</span><span style=\"color:#f8f8f2;\">()).</span><span style=\"color:#66d9ef;\">unwrap_or_default</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">return </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(height);\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">None\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"font-style:italic;color:#28c6e4;\">fn </span><span style=\"color:#6aaf19;\">scroll_into_view</span><span style=\"color:#000000;\">(</span><span style=\"font-style:italic;color:#fd971f;\">mounted</span><span style=\"color:#000000;\">: UseMounted) {{\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#28c6e4;\">let Some</span><span style=\"color:#000000;\">(element) </span><span style=\"color:#f92672;\">= &amp;*</span><span style=\"color:#000000;\">mounted.signal.</span><span style=\"color:#28c6e4;\">read</span><span style=\"color:#000000;\">() {{\n</span><span style=\"color:#000000;\">        </span><span style=\"font-style:italic;color:#28c6e4;\">let </span><span style=\"color:#f92672;\">_ =</span><span style=\"color:#000000;\"> element.</span><span style=\"color:#28c6e4;\">scroll_to</span><span style=\"color:#000000;\">(ScrollBehavior::Smooth);\n</span><span style=\"color:#000000;\">    }}\n</span><span style=\"color:#000000;\">}}\n</span><span style=\"color:#000000;\">\n</span><span style=\"font-style:italic;color:#28c6e4;\">fn </span><span style=\"color:#6aaf19;\">get_scroll_height</span><span style=\"color:#000000;\">(</span><span style=\"font-style:italic;color:#fd971f;\">mounted</span><span style=\"color:#000000;\">: UseMounted) -&gt; Option&lt;</span><span style=\"font-style:italic;color:#28c6e4;\">i32</span><span style=\"color:#000000;\">&gt; {{\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#28c6e4;\">let Some</span><span style=\"color:#000000;\">(element) </span><span style=\"color:#f92672;\">= &amp;*</span><span style=\"color:#000000;\">mounted.signal.</span><span style=\"color:#28c6e4;\">read</span><span style=\"color:#000000;\">() {{\n</span><span style=\"color:#000000;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#28c6e4;\">let Some</span><span style=\"color:#000000;\">(raw_elem) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#000000;\"> element.downcast::&lt;web_sys::Element&gt;() {{\n</span><span style=\"color:#000000;\">            </span><span style=\"font-style:italic;color:#28c6e4;\">let</span><span style=\"color:#000000;\"> att </span><span style=\"color:#f92672;\">=</span><span style=\"color:#000000;\"> raw_elem.</span><span style=\"color:#28c6e4;\">get_attribute</span><span style=\"color:#000000;\">(</span><span style=\"color:#f25a00;\">&quot;style&quot;</span><span style=\"color:#000000;\">).</span><span style=\"color:#28c6e4;\">unwrap_or_default</span><span style=\"color:#000000;\">();\n</span><span style=\"color:#000000;\">            raw_elem.</span><span style=\"color:#28c6e4;\">set_attribute</span><span style=\"color:#000000;\">(</span><span style=\"color:#f25a00;\">&quot;style&quot;</span><span style=\"color:#000000;\">, format!(</span><span style=\"color:#f25a00;\">&quot;</span><span style=\"color:#ae81ff;\">{{}}</span><span style=\"color:#f25a00;\">;display: block;&quot;</span><span style=\"color:#000000;\">, att).</span><span style=\"color:#28c6e4;\">as_str</span><span style=\"color:#000000;\">()).</span><span style=\"color:#28c6e4;\">unwrap_or_default</span><span style=\"color:#000000;\">();\n</span><span style=\"color:#000000;\">            </span><span style=\"font-style:italic;color:#28c6e4;\">let</span><span style=\"color:#000000;\"> height </span><span style=\"color:#f92672;\">=</span><span style=\"color:#000000;\"> raw_elem.</span><span style=\"color:#28c6e4;\">scroll_height</span><span style=\"color:#000000;\">();\n</span><span style=\"color:#000000;\">            raw_elem.</span><span style=\"color:#28c6e4;\">set_attribute</span><span style=\"color:#000000;\">(</span><span style=\"color:#f25a00;\">&quot;style&quot;</span><span style=\"color:#000000;\">, att.</span><span style=\"color:#28c6e4;\">as_str</span><span style=\"color:#000000;\">()).</span><span style=\"color:#28c6e4;\">unwrap_or_default</span><span style=\"color:#000000;\">();\n</span><span style=\"color:#000000;\">            </span><span style=\"color:#f92672;\">return </span><span style=\"font-style:italic;color:#28c6e4;\">Some</span><span style=\"color:#000000;\">(height);\n</span><span style=\"color:#000000;\">        }}\n</span><span style=\"color:#000000;\">    }}\n</span><span style=\"color:#000000;\">    </span><span style=\"font-style:italic;color:#28c6e4;\">None\n</span><span style=\"color:#000000;\">}}</span></pre>\n" },
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "When development is done, it's time to add some optimizations. You can find more information "
            a {
                href: "https://dioxuslabs.com/learn/0.6/cookbook/optimizing",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "here"
            }
            ". I use these"
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"color:#f8f8f2;\">[profile.release]\n</span><span style=\"color:#f8f8f2;\">opt</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">level </span><span style=\"color:#f92672;\">= </span><span style=\"color:#e6db74;\">&quot;z&quot;\n</span><span style=\"color:#f8f8f2;\">debug </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">false\n</span><span style=\"color:#f8f8f2;\">lto </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#f8f8f2;\">codegen</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">units </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">1\n</span><span style=\"color:#f8f8f2;\">panic </span><span style=\"color:#f92672;\">= </span><span style=\"color:#e6db74;\">&quot;abort&quot;\n</span><span style=\"color:#f8f8f2;\">strip </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#f8f8f2;\">incremental </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">false</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"color:#000000;\">[profile.release]\n</span><span style=\"color:#000000;\">opt</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">level </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f25a00;\">&quot;z&quot;\n</span><span style=\"color:#000000;\">debug </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">false\n</span><span style=\"color:#000000;\">lto </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#000000;\">codegen</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">units </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">1\n</span><span style=\"color:#000000;\">panic </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f25a00;\">&quot;abort&quot;\n</span><span style=\"color:#000000;\">strip </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#000000;\">incremental </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ae81ff;\">false</span></pre>\n" },
        }
        h1 {
            id: "deployment",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#deployment", "Deployment" }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Dioxus-specific deployment support is coming soon, and you can follow the updates "
            a {
                href: "https://dioxuslabs.com/deploy",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "here"
            }
            ". In the meantime, we can deploy a Dioxus application using Azure Static Web Apps, similar to the steps described "
            a {
                href: "https://www.linkedin.com/pulse/hosting-flutter-web-app-azure-static-apps-dmytro-kravchyna-pwp9e/?trackingId=emI92qzFTVigexUSzOl68Q%3D%3D&lipi=urn%3Ali%3Apage%3Ad_flagship3_pulse_read%3BQIwogWXUT%2F6vQxKinbW7Hg%3D%3D",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "here"
            }
            "."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "We can also explore community deployment samples from "
            a {
                href: "https://dioxuslabs.com/awesome",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Dioxus Awesome"
            }
            ". For instance, try deploying using the "
            a {
                href: "https://vercel.com/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "Vercel"
            }
            " template, as demonstrated in the "
            a {
                href: "https://github.com/lucifer1004/dioxus-vercel-demo",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "dioxus-vercel-demo repository"
            }
            ". Note that I had to modify the deployment instructions slightly to get it working:"
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Replace:"
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"color:#f8f8f2;\"> </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> name: Install trunk\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\">: steps.cache.outputs.cache</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">hit </span><span style=\"color:#f92672;\">!= &#39;</span><span style=\"color:#ae81ff;\">true</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">    uses: actions</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">cargo</span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">v1\n</span><span style=\"color:#f8f8f2;\">    with:\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">use-</span><span style=\"color:#f8f8f2;\">cross: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#f8f8f2;\">      command: install\n</span><span style=\"color:#f8f8f2;\">      args: trunk\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> name: Build web pages\n</span><span style=\"color:#f8f8f2;\">    run: trunk build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">release</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"color:#000000;\"> </span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> name: Install trunk\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#000000;\">: steps.cache.outputs.cache</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">hit </span><span style=\"color:#f92672;\">!= &#39;</span><span style=\"color:#ae81ff;\">true</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">    uses: actions</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">rs</span><span style=\"color:#f92672;\">/</span><span style=\"color:#000000;\">cargo</span><span style=\"color:#f92672;\">@</span><span style=\"color:#000000;\">v1\n</span><span style=\"color:#000000;\">    with:\n</span><span style=\"color:#000000;\">      </span><span style=\"color:#f92672;\">use-</span><span style=\"color:#000000;\">cross: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#000000;\">      command: install\n</span><span style=\"color:#000000;\">      args: trunk\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> name: Build web pages\n</span><span style=\"color:#000000;\">    run: trunk build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">release</span></pre>\n" },
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "With:"
        }
        CodeBlock {
            darkmode: dark_mode().0,
            contents: if dark_mode().0 { "<pre style=\"background-color:#272822;\">\n<span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> name: Install Dioxus\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\">: steps.cache.outputs.cache</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">hit </span><span style=\"color:#f92672;\">!= &#39;</span><span style=\"color:#ae81ff;\">true</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#f8f8f2;\">    uses: actions</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">cargo</span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">v1\n</span><span style=\"color:#f8f8f2;\">    with:\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">use-</span><span style=\"color:#f8f8f2;\">cross: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#f8f8f2;\">      command: install\n</span><span style=\"color:#f8f8f2;\">      args: dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">cli\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> name: Build web pages\n</span><span style=\"color:#f8f8f2;\">    run: dx build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">release</span></pre>\n" } else { "<pre style=\"background-color:#ffffff;\">\n<span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> name: Install Dioxus\n</span><span style=\"color:#000000;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#000000;\">: steps.cache.outputs.cache</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">hit </span><span style=\"color:#f92672;\">!= &#39;</span><span style=\"color:#ae81ff;\">true</span><span style=\"color:#f92672;\">&#39;\n</span><span style=\"color:#000000;\">    uses: actions</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">rs</span><span style=\"color:#f92672;\">/</span><span style=\"color:#000000;\">cargo</span><span style=\"color:#f92672;\">@</span><span style=\"color:#000000;\">v1\n</span><span style=\"color:#000000;\">    with:\n</span><span style=\"color:#000000;\">      </span><span style=\"color:#f92672;\">use-</span><span style=\"color:#000000;\">cross: </span><span style=\"color:#ae81ff;\">true\n</span><span style=\"color:#000000;\">      command: install\n</span><span style=\"color:#000000;\">      args: dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\">cli\n</span><span style=\"color:#000000;\">\n</span><span style=\"color:#000000;\">  </span><span style=\"color:#f92672;\">-</span><span style=\"color:#000000;\"> name: Build web pages\n</span><span style=\"color:#000000;\">    run: dx build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#000000;\">release</span></pre>\n" },
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "The component described here is part of a larger application, which you can view "
            a {
                href: "https://dmytro-kravchyna.vercel.app/",
                class: "text-sm underline hover:decoration-2 focus:outline-none focus:decoration-2",
                class: if dark_mode().0 { "text-neutral-500 hover:text-neutral-400" } else { "text-gray-500 hover:text-gray-800" },
                "here"
            }
            "."
        }
        h1 {
            id: "conclusion",
            class: "text-lg font-medium",
            class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
            a { href: "#conclusion", "Conclusion" }
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "Rust has gained widespread popularity for its performance, memory safety, and suitability for critical applications. This article outlines Rust’s adoption across various sectors, from Google’s Android development to Microsoft’s Windows core components, Linux kernel, AI infrastructure, and even aerospace and automotive industries. Rust's integration into Mozilla's Gecko engine, blockchain platforms, and privacy tools underscores its versatility and reliability."
        }
        p {
            class: "text-sm",
            class: if dark_mode().0 { "text-neutral-400" } else { "text-gray-600" },
            "For web development, Rust's compatibility with WebAssembly has enabled fast, secure web applications. Frameworks like Dioxus, Yew, and Leptos provide Rust developers with efficient tools for building web applications. Notably, Dioxus extends Rust’s reach to desktop and mobile, creating a unified development experience."
        }
    }
}
