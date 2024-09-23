use yew::prelude::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default(false)]
    pub open_in_tab: bool,
    #[prop_or_default("")]
    pub link: String,
    #[prop_or_default("")]
    pub class: &'static str,
    pub children: Html,
}

#[function_component]
pub fn TextLink(props: &Props) -> Html {
    let classes: &'static str = match props.class.is_empty() {
        true => "underline text-blue-700 dark:text-blue-400",
        false => props.class,
    };

    if props.open_in_tab {
        return html! {
            <a
                href={props.link.clone()}
                target="_blank"
                rel="noopener noreferrer"
                class={classes}
            >
                {props.children.clone()}
            </a>
        };
    }

    html! {
        <a
            href={props.link.clone()}
            class={classes}
        >
            {props.children.clone()}
        </a>
    }
}
