use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Html,
    #[prop_or_default]
    pub style: Option<String>, // Optional style property
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div style={props.style.clone().unwrap_or_default()}>
            <div
                class="p-6 bg-white border border-gray-200 rounded-lg dark:bg-slate-800 dark:border-slate-700"
            >
                { props.children.clone() }
            </div>
        </div>
    }
}
