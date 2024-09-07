use yew::{Html, html, function_component};
use crate::components::svg::logos::{Gitlab, Linkedin};
use crate::ui::TextLink;

struct Item {
    link: &'static str,
    icon: Html,
    title: &'static str,
}

#[function_component]
pub fn Socials() -> Html {
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
        //Item {
        //    link: "https://www.x.com/marcempunkt",
        //    icon: html! { <Twitter /> },
        //    title: "Twitter",
        //},
        Item {
            link: "https://gitlab.com/marcempunkt",
            icon: html! { <Gitlab /> },
            title: "Gitlab",
        },
        Item {
            link: "https://www.linkedin.com/in/marc-m%C3%A4urer-a877492b8/",
            icon: html! { <Linkedin /> },
            title: "Linkedin",
        },
        // TODO email
    ];

    html! {
        <div>
            <ul class="w-full p-4 flex flex-wrap gap-4 justify-around items-center">
                {items.into_iter().map(|item: Item| html! {
                    <li
                        class="rounded-md hover:bg-gray-50 dark:hover:bg-slate-700 transition"
                        title={item.title}
                    >
                        <TextLink
                            link={item.link}
                            open_in_tab={true}
                            class="p-4 inline-block [&>svg]:w-[2rem] [&>svg]:h-[2rem] [&>svg]:dark:stroke-white [&>svg]:dark:fill-white"
                        >
                            {item.icon}
                        </TextLink>
                    </li>
                }).collect::<Html>()}
            </ul>
        </div>
    }
}
