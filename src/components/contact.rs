use crate::components::svg::logos::{Github, Linkedin, Mail, PdfIcon, Twitter};
use crate::ui::{Card, TextLink, Title};
use yew::prelude::*;

struct Item {
    link: &'static str,
    icon: Html,
    title: &'static str,
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let items: Vec<Item> = vec![
        //Item {
        //    link: "https://www.youtube.com/@marcempunkt9737",
        //    icon: html! { <Youtube /> },
        //    title: "Youtube",
        //},
        // Item {
        //     link: "https://www.twitch.tv/marcempunkt",
        //     icon: html! { <Twitch /> },
        // },
        Item {
            link: "https://www.x.com/ccintoci",
            icon: html! { <Twitter /> },
            title: "Twitter",
        },
        Item {
            link: "https://github.com/intx4",
            icon: html! { <Github /> },
            title: "Github",
        },
        Item {
            link: "https://www.linkedin.com/in/francesco-intoci-94465719b/",
            icon: html! { <Linkedin /> },
            title: "Linkedin",
        },
        Item {
            link: "mailto:francesco.intoci@protonmail.com",
            icon: html! { <Mail /> },
            title: "Email",
        },
        Item {
            // TODO: update repo
            link: "https://raw.githubusercontent.com/intx4/intx4.github.io/rust_wasm/assets/cv.pdf",
            icon: html! { <PdfIcon /> },
            title: "CV",
        },
    ];

    let socials = html! {
        <div>
            <ul class="w-full p-4 flex flex-wrap gap-4 justify-around items-center">
            {items.into_iter().map(|item: Item| {
                    html! {
                        <li title={item.title}>
                            <TextLink
                                link={item.link}
                                open_in_tab={ !item.link.contains("mailto") }
                                class="p-4 inline-block [&>svg]:w-[2rem] [&>svg]:h-[2rem] [&>svg]:dark:stroke-white [&>svg]:dark:fill-white"
                            >
                                { item.icon }
                            </TextLink>
                        </li>
                    }
            }).collect::<Html>()}
            </ul>
        </div>
    };

    html! {
    <>
        <Title id="contact">
                { "Contacts" }
        </Title>

    <div class="flex flex-col gap-10">
        { socials }
    </div>

        </>
    }
}
