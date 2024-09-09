use yew::prelude::*;
use crate::components::svg::{
    emojis::{ Blushing },
    undraw::QuickChat,
};
use crate::components::svg::logos::{Linkedin, Mail, Twitter, Github};
use crate::ui::{Title, Card, TextLink};

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

    html!{
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
