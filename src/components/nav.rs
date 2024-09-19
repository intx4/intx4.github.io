use crate::components::svg::themes::{Dark, Light};
use crate::contexts::theme::ThemeAction;
use crate::ui::{Button, TextLink};
use crate::app::AppContext;
use yew::prelude::{function_component, html, use_context, Callback, Html};

#[function_component]
pub fn Nav() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_theme_icon(app_context: AppContext) -> Html {
        match app_context.theme.current {
            "light" => html! {<Light class={Some("h-[1.5rem] w-[1.5rem] fill-orange-400")} />},
            "dark" | _ => html! {<Dark class={Some("h-[1.5rem] w-[1.5rem] fill-slate-300")} />},
        }
    }

    fn handle_url_icon(app_context: AppContext) -> Html {
        if app_context.url.current.contains("blog") && !app_context.url.current.contains("post"){
            return html!(
                <a href="/">
                    <Button is_secondary={true}>
                        { "Home" }
                    </Button>
                </a>
            )
        } else {
            return html!(
                <a href="/blog">
                    <Button is_secondary={true}>
                        { "Blog" }
                    </Button>
                </a>
            )
        }
    }

    fn handle_all_buttons(app_context: AppContext) -> Html{
        let cycle_theme = {
            let app_context = app_context.clone();
            let current_theme: &str = app_context.theme.current;
            let current_theme_index: usize = match app_context
                .theme_cycle
                .iter()
                .position(|x: &&str| x == &current_theme)
            {
                Some(i) => i,
                None => 0,
            };
            let next_theme: &str = match app_context.theme_cycle.iter().nth(current_theme_index + 1) {
                Some(nt) => nt,
                None => "light",
            };
            Callback::from(move |_| match next_theme {
                "dark" => app_context.theme.dispatch(ThemeAction::Dark),
                "light" | _ => app_context.theme.dispatch(ThemeAction::Light),
            })
        };
        let path = app_context.url.current.strip_prefix("/").unwrap();

        if !path.eq("404"){
            return html!(
                <ul class="flex justify-end gap-4 items-center">
                    <li onclick={ cycle_theme }>
                        { handle_theme_icon(app_context.clone()) }
                    </li>
                    <li>
                        { handle_url_icon(app_context.clone()) }
                    </li>
                </ul>
            )
        }
        html!()
    }

    html! {
        <nav class="w-full" style="margin-bottom:1.5rem">
        { handle_all_buttons(app_context.clone())}
        </nav>
    }
}
