use crate::app::*;
use crate::components::svg::logos::{Github, Linkedin, Mail, PdfIcon, Twitter};
use crate::ui::{TextLink, Title};
use yew::prelude::*;

struct Item {
    link: String,
    icon: Html,
    title: &'static str,
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let cv_url_string = get_raw_contents_url("cv.pdf", AssetType::Generic);

    let items: Vec<Item> = vec![
        Item {
            link: "https://www.x.com/ccintoci".to_string(),
            icon: html! { <Twitter /> },
            title: "Twitter",
        },
        Item {
            link: "https://github.com/intx4".to_string(),
            icon: html! { <Github /> },
            title: "Github",
        },
        Item {
            link: "https://www.linkedin.com/in/francesco-intoci-94465719b/".to_string(),
            icon: html! { <Linkedin /> },
            title: "Linkedin",
        },
        Item {
            link: "mailto:francesco.intoci@protonmail.com".to_string(),
            icon: html! { <Mail /> },
            title: "Email",
        },
        Item {
            link: cv_url_string,
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
                                link={item.link.clone()}
                                open_in_tab={ !item.link.as_str().contains("mailto") }
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
