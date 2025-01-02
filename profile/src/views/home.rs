use dioxus::prelude::*;
use dioxus_use_mounted::use_mounted;

use crate::components::{
    Biography, BookSection, CardBlog, CardSection, Category, Contact, ContactType, DescriptionList,
    Education, Feed, Gallery, KnownColleges, KnownLogo, Media, NavBar, NavItem, Section, Skill,
    Testimonial, Testimonials, Timeline, UserProfile,
};
use crate::docs::router_articles::BookRoute as BlogRoute;
use crate::DarkMode;

#[component]
pub fn Home() -> Element {
    let home_nav: NavItem = NavItem {
        id: "home",
        href: "#home",
        title: "Home",
        section: use_mounted(),
    };
    let projects_nav: NavItem = NavItem {
        id: "projects",
        href: "#projects",
        title: "Projects",
        section: use_mounted(),
    };
    let work_nav: NavItem = NavItem {
        id: "work",
        href: "#work",
        title: "Work",
        section: use_mounted(),
    };
    let articles_nav: NavItem = NavItem {
        id: "articles",
        href: "#articles",
        title: "Articles",
        section: use_mounted(),
    };

    let dark_mode = use_context::<Signal<DarkMode>>();

    rsx! {
        header { class: "top-0 inset-x-0 flex flex-wrap md:justify-start md:flex-nowrap w-full z-50 text-sm sticky",
            // onload: move |data| { },
            NavBar {
                darkmode: dark_mode().0,
                menu: vec![ home_nav, projects_nav, work_nav, articles_nav],
                brandlink: "https://github.com/KDet",
            }
        }
        main {
            div { class: "w-full max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 pt-10 md:pt-16",
                UserProfile {
                    darkmode: dark_mode().0,
                    onmounted: move |cx| home_nav.section.onmounted(cx),
                    bio: Biography {
                        photo: "https://media.licdn.com/dms/image/v2/C4E03AQHVjVsfWdcStA/profile-displayphoto-shrink_800_800/profile-displayphoto-shrink_800_800/0/1516850185499?e=1740614400&v=beta&t=klwiDgG7VDdiChFBrW7IzDeW-5FztkhEPHqZPAmA4LA",
                        fullname: "Dmytro Kravchyna",
                        title: "Software Developer",
                        about: Some("I am a software developer with over a decade of experience in Building Information Modeling (BIM) integration, cloud solutions, and mobile application development.\nMy expertise lies in cross-platform development, utilizing advanced technologies such as Flutter, Xamarin, and ASP.NET Core microservices. My favorite project was developing a multi-platform construction management system, which significantly improved team collaboration, reduced communication delays, and boosted workflow efficiency.\nI am deeply committed to developing technical proofs of concept (PoCs), pilot projects, and foundational software components that address complex, real-world challenges. Additionally, I have extensive experience designing and managing CI/CD pipelines, leveraging Docker to deploy applications in an efficient, scalable manner."),
                        contacts: Some(vec![
                            Contact { platform: ContactType::Email, title: "kravchyna@gmail.com", href: "mailto:kravchyna@gmail.com" },
                            Contact { platform: ContactType::Telegram, title: "@dimakrkr", href: "https://telegram.me/dimakrkr" },
                            // Contact { platform: ContactType::TwitterX, title: "@KDet00", href: "https://x.com/intent/post?screen_name=KDet00" },
                        ])
                    }
                }
                Section {
                    // id: PROJECTS_NAV.id,
                    title: "Projects",
                    onmounted: move |cx| projects_nav.section.onmounted(cx),
                    darkmode: dark_mode().0,
                    Gallery { darkmode: dark_mode().0, items: vec![
                        Media {
                            title: "Business Analysis Tool",
                            href: "https://bitimpulse.com/en/business-analysis-tool-enterprise/",
                            image: "https://bitimpulse.com/wp-content/uploads/2024/08/7-11.png"
                        },
                        Media {
                            title: "Emergency Notifier",
                            href: "https://exoft.net/our-works/emergency-notifier/",
                            image: "https://exoft.net/articles/emergency-notifier-screenshot-2.webp"
                        },
                        Media { title: "Sensors Security", href: "#", image: "https://tresslerassociates.com/wp-content/uploads/2023/02/nda.webp" },
                        Media { title: "Media Streaming", href: "#", image: "https://tresslerassociates.com/wp-content/uploads/2023/02/nda.webp" },
                        Media { title: "Construction (BIM)", href: "#", image: "https://tresslerassociates.com/wp-content/uploads/2023/02/nda.webp" },
                    ]},
                }
                Section {
                    darkmode: dark_mode().0,
                    secondary: true,
                    title: "Testimonials",
                    Testimonials { darkmode: dark_mode().0, items: vec![
                        Testimonial {
                            quote: "He [Dmytro] got a can-do attitude and combined the deep technical skillset makes him a awesome parter-in-crime for any digital development or leading a team of IT developers.",
                            photo: "https://media.licdn.com/dms/image/v2/C4E03AQE5B_e6IXH_Bg/profile-displayphoto-shrink_100_100/profile-displayphoto-shrink_100_100/0/1662121356367?e=1741219200&v=beta&t=6d50VHHptnklvpnlnpBNLOJ6HAqhrXy7CB0F3FvhdFs",
                            fullname: "Lucas Fuglsang",
                        },
                        Testimonial {
                            quote: "Dmytro is a very talented and professional person, who always does his job in time and with the best possible quality. Willing to learn new things, has extremely strong mentoring skills.",
                            photo: "https://media.licdn.com/dms/image/v2/D4D03AQEDtlWltRc_sQ/profile-displayphoto-shrink_100_100/profile-displayphoto-shrink_100_100/0/1679955653189?e=1741219200&v=beta&t=vu7rvNEIh0dT4ubETGjoIQ9p87gB-Lm6EjA_XQLROOY",
                            fullname: "Nazar Fedyk",
                        },
                    ]}
                }

                Section {
                    darkmode: dark_mode().0,
                    title: "Skills",
                    DescriptionList { darkmode: dark_mode().0, items: vec![
                        Category {
                            name: "Languages",
                            values: vec![Skill::CSharp, Skill::Dart, Skill::Cpp, Skill::Rust, Skill::Swift ]
                        },
                        Category {
                            name: "Development",
                            values: vec![ Skill::AspNetCore, Skill::Flutter, Skill::Xamarin, Skill::WPF,  Skill::Dioxus, Skill::SwiftUI ]
                        },

                        Category {
                            name: "Cloud and DevOps",
                            values: vec![Skill::Kubernetes, Skill::Docker, Skill::YAML, Skill::Git ]
                        },
                        Category {
                            name: "Design Tools",
                            values: vec![Skill::Sketch, Skill::Figma]
                        },
                        // Category {
                        //     name: "Collaboration Tools",
                        //     values: vec![Skill::MSTeams, Skill::Slack, Skill::GoogleChat ]
                        // },
                        Category {
                            name: "Project Experience",
                            values: vec![
                                Skill::Name("Building Information Modeling"),
                                Skill::Name("Security Monitoring Systems"),
                                Skill::Name("Business Analysis"),
                                Skill::Name("Emergency Notification"),
                                Skill::Name("Media Streaming")
                            ]
                        },
                        Category {
                            name: "Soft Skills",
                            values: vec![
                                Skill::Name("Problem-solving"),
                                Skill::Name("Attention to detail"),
                                Skill::Name("Stress resilience"),
                                Skill::Name("Time management")
                            ]
                        }
                    ]}
                }
                Section {
                    darkmode: dark_mode().0,
                    title: "Work experience",
                    // id: WORK_NAV.id,
                    onmounted: move |cx| work_nav.section.onmounted(cx),
                    Timeline { darkmode: dark_mode().0, items: vec![
                        Feed {
                            logo: KnownLogo::Netminds,
                            date: "May 2022 - Present",
                            title: "Lead Software Developer at Netminds (ex InterLogic)",
                            description: Some("The outsourcing company is a Danish IT company with more than 300 employees in our own development centers in Poland and Ukraine."),
                            notes: Some(vec![
                                "Lead investigations and create technical Proofs of Concept (PoCs) to ensure the right technology stack is selected before development.",
                                "Work on pilot projects by developing small demos that can be validated with customers before making larger investments.",
                                "Develop core Flutter components.",
                                "Develop extensions for construction applications, like Revit plugins.",
                            ]),
                            blog: Some(CardBlog {
                                title: "Netminds's web page",
                                description: "Scandinavian spirit with a global touch",
                                image: "https://media.licdn.com/dms/image/v2/D4D22AQEEQcNEufP8Ew/feedshare-shrink_2048_1536/B4DZPhBfo.G0Ao-/0/1734647107300?e=1737590400&v=beta&t=12waMs5zDZHUs84H6rBegOwmOF3bY_PENWFv_gQU0aA",
                                url: "https://www.netminds.dk/en"
                            }),
                        },
                        Feed {
                            logo: KnownLogo::Netminds,
                            date: "May 2019 - May 2022",
                            title: "Senior Software Developer at Netminds (ex InterLogic)",
                            description: None,
                            notes: Some(vec![
                                "Develop and maintain a cross-platform (iOS, Android, Windows) field construction application using Flutter.",
                                "Integrate a 3D BIM viewer (C++ Vulkan-based) into the Flutter (Skia) application for advanced visualization capabilities.",
                                "Develop a cloud-based 3D model converter as an ASP.NET Core microservice in Linux containers using Docker and Azure Kubernetes Service.",
                                "Design and implement Azure CI/CD YAML build pipelines to automate the deployment process.",
                                "Provide technical support and guidance to other developers working on cloud conversion services and mobile/desktop applications."
                            ]),
                            blog: None,
                        },
                        Feed {
                            logo: KnownLogo::GlobalLogic,
                            date: "Jan 2018 - May 2019",
                            title: "Senior Software Developer at GlobalLogic",
                            description: Some("GlobalLogic, a Hitachi Group Company, is a full-lifecycle product development services leader that combines chip-to-cloud software engineering expertise and vertical industry experience to help our customers design, build, and deliver their next generation products and digital experiences."),
                            notes: Some(vec![
                                "Windows Desktop client-side modules development",
                                "Web server side modules development",
                                "Code review and knowledge sharing",
                                "Collaboration with the different teams",
                                "Leading 2 direct reporters (code review, time management, hand out tasks)",
                                "Participated in media expertise meetings (architecture/known solution discussions, process documentation)",
                                "Develop main AppleTV modules",
                                "Assist other clients (Xbox, PS4)",
                                "Code translation and adoption to native solutions (ex.: from Xamarin.iOS to Swift.AppleTV)",
                            ]),
                            blog: Some(CardBlog {
                                title: "GlobalLogic's Insights",
                                description: "We are advocates of knowledge sharing - so we encourage our talented engineers and designers to share their experiences and insights with the world.",
                                image: "https://www.globallogic.com/wp-content/uploads/2019/12/Insights-min.jpg",
                                url: "https://www.globallogic.com/insights/"
                            }),
                        },
                        Feed {
                            logo: KnownLogo::GlobalLogic,
                            date: "Apr 2017 - Jan 2018",
                            title: "Software Developer at GlobalLogic",
                            description: Some("The key project provides the insights that enable enterprises and security-conscious organizations to take the best action at the right time by correlating structured and unstructured data from multiple sensors and channels, detecting irregular patterns, and recognizing trends."),
                            notes: None,
                            blog: None,
                        },
                        Feed {
                            logo: KnownLogo::Exoft,
                            date: "Mar 2016 - Apr 2017",
                            title: "Software Developer at Exoft",
                            description: Some("Exoft is a custom software development company headquartered in Ukraine with over a decade of experience providing software solutions to clients worldwide."),
                            notes: Some(vec![
                                "Mobile (iOS, Android, UWP) and desktop (macOS, Linux Red Hat/Fedora) parts of application.",
                                "Creating architecture.",
                                "Creating UI with Xamarin.Forms (mobile) and Eto.Forms (desktop).",
                                "Connecting to V.ALRT device via Bluetooth LE; widgets support (iOS, Android) and other features implementing."
                            ]),
                            blog: Some(CardBlog {
                                title: "Blogs by Exoft",
                                description: "Discover insightful articles, engaging stories, and expert perspectives on diverse topics.",
                                image: "https://exoft.net/blog.webp",
                                url: "https://exoft.net/blog/"
                            }),
                        },
                        Feed {
                            logo: KnownLogo::Bitimpulse,
                            date: "Aug 2014 - Aug 2016",
                            title: "Software Developer at BIT Impulse",
                            description: Some("BIT Impulse specializes in analytical projects. It helps businesses analyze all business processes, create and implement effective analytical steps."),
                            notes: Some(vec![
                                "Maintaining the Windows Desktop app for large data analysis.",
                                "Working with SQL Server databases and data consuming from OLAP cubes.",
                                "Researching the porting of the application to macOS, iOS (Xamarin.Forms), and UWP.",
                            ]),
                            blog: Some(CardBlog {
                                title: "Blogs by BIT Impulse",
                                description: "Make your business efficient! Take effective management decisions and control business with Business Analysis Tool from BIT Impulse Company.",
                                image: "https://bitimpulse.com/wp-content/uploads/2021/06/Group-153-1.jpg.webp",
                                url: "https://bitimpulse.com/en/category/blog-2/"
                            }),
                        },
                        Feed {
                            logo: KnownLogo::Bitimpulse,
                            date: "Aug 2013 - Aug 2014",
                            title: "Junior Software Developer at BIT Impulse",
                            description: Some("As a junior software developer, I focused on writing code for new features and fixing bugs while collaborating closely with senior developers. I created tests to ensure the functionality of my work and actively participated in code reviews to improve code quality. I documented my projects and took the initiative to learn new technologies and best practices. This experience helped me build my skills and gain valuable insights into the software development process."),
                            notes: None,
                            blog: None,
                        },
                        // Feed {
                        //     logo: KnownLogo::Other,
                        //     date: "2010 - 2011",
                        //     title: "Freelance Graphic Designer",
                        //     description: None,
                        //     notes: Some(vec![
                        //         "Worked with a diverse range of clients, delivering tailored design solutions.",
                        //         "Developed and maintained strong client relationships through effective communication and project management.",
                        //         "Utilized tools such as Notion for project tracking, Mailchimp for email marketing designs, Slack for team collaboration, and GitHub for version control and project sharing.",
                        //     ]),
                        //     blog: None,
                        // }
                    ]}
                }
                Section {
                    darkmode: dark_mode().0,
                    title: "Education",
                    CardSection { darkmode: dark_mode().0, items: vec![
                        Education {
                            logo: KnownColleges::LPNU,
                            title: "Lviv Polytechnic National University",
                            details: "2014 - 2016",
                            subtitle: "Master's degree in Artificial Intelligence",
                            href: "https://lpnu.ua/en/ais",
                        },
                        Education {
                            logo: KnownColleges::LPNU,
                            title: "Lviv Polytechnic National University",
                            details: "2010 - 2014",
                            subtitle: "Bachelor's degree in Information Systems and Networks",
                            href: "https://lpnu.ua/en/isn",
                        },
                    ]},
                }
                Section {
                    darkmode: dark_mode().0,
                    title: "Articles",
                    // id: ARTICLES_NAV.id,
                    onmounted: move |cx| articles_nav.section.onmounted(cx),
                    BookSection { darkmode: dark_mode().0, routes: BlogRoute::static_routes()
                    // Note {
                    //     // href: "1",
                    //     href: "https://www.linkedin.com/pulse/open-source-c-version-vimmath3d-dmytro-kravchyna-vjice/",
                    //     subtitle: "2024",
                    //     title: "Open-Source C++ Version of Vim.Math3D",
                    //     description: "Does your project need an efficient library for 3D mathematics? VIM Math 3D is a lightweight and easy-to-use library. It is now available for C++. It retains all the features of the original version, without third-party dependencies, and offers cross-platform compatibility achieved with CMake.",
                    // },
                    // Note {
                    //     // href: "1",
                    //     href: "https://www.linkedin.com/pulse/hosting-flutter-web-app-azure-static-apps-dmytro-kravchyna-pwp9e/",
                    //     subtitle: "2024",
                    //     title: "Hosting a Flutter Web App in Azure Static Web Apps",
                    //     description: "Flutter provides a versatile way to build web applications, and Azure Static Web Apps offers a good platform for hosting them. This guide will walk you through building a Flutter web app using Azure DevOps and deploying it to Azure Static Web Apps, ensuring a clear understanding of each step.",
                    // },
                    // Note {
                    //     // href: "1",
                    //     href: "https://www.linkedin.com/pulse/rust-experiments-webassembly-development-dmytro-kravchyna-9mcde/",
                    //     subtitle: "2024",
                    //     title: "Rust Experiments: WebAssembly development",
                    //     description: "Rust is worth experimenting with, and WebAssembly is the first and fun stop for diving into its potential!",
                    // },
                    },
                }
                // Section {
                //     darkmode: dark_mode().0,
                //     title: "Subscribe",
                //     Subscribe { darkmode: dark_mode().0 }
                // }
            }
        }
    }
}
