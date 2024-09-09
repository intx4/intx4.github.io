use yew::prelude::{Html, Callback, MouseEvent, html, use_context, use_state, use_effect_with, function_component};
use crate::ui::{Button, TextLink};
use crate::components::svg::emojis::Home;
use crate::AppContext;
use crate::contexts::theme::ThemeAction;
use crate::components::svg::themes::{ Light, Dark };

#[function_component]
pub fn Nav() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    let cycle_theme = {
        let app_context = app_context.clone();
        let current_theme: &str = app_context.theme.current;
        let current_theme_index: usize = match app_context.theme_cycle.iter().position(|x: &&str| x == &current_theme) {
            Some(i) => i,
            None => 0,
        };
        let next_theme: &str = match app_context.theme_cycle.iter().nth(current_theme_index + 1) {
            Some(nt) => nt,
            None => "light",
        };
        Callback::from(move|_| {
            match next_theme {
            "dark" => app_context.theme.dispatch(ThemeAction::Dark),
            "light" | _ => app_context.theme.dispatch(ThemeAction::Light)
            }
        })
    };

    fn handle_theme_icon(app_context: AppContext) -> Html {
        match app_context.theme.current {
            "light" => html!{<Light class={Some("h-[1.5rem] w-[1.5rem] fill-orange-400")} />},
            "dark" | _ => html!{<Dark class={Some("h-[1.5rem] w-[1.5rem] fill-slate-300")} />}
        }
    }

    html!{
	<header class="flex flex-wrap gap-2 justify-between items-center select-none">
	    /* Navigation */
	    <nav>
		<ul class="flex flex-wrap gap-4 items-center [&>li]:cursor-pointer">
	            /* Theme Switcher */
                    <li onclick={ cycle_theme }>
                        { handle_theme_icon(app_context.clone()) }
                    </li>

	            /* Link to Projects */
                //    <li>
                //        <a href="#projects">
                //            <Button>
                //                {translate_projects(app_context.clone())}
                //            </Button>
                //        </a>
                //    </li>

                // TODO blog

	            /* Link to blog */
                    <li>
                        <a href="#blog">
                            <Button is_secondary={true}>
                                { "Blog (Coming Soon)" }
                            </Button>
                        </a>
                    </li>
		</ul>
	    </nav>
	</header>
    }
}
