use crate::contexts::theme::{use_theme_context, ThemeState};
use yew::{function_component, html, ContextProvider, Html, UseReducerHandle};

mod components;
mod contexts;
mod ui;

use components::{aboutme::Aboutme, contact::Contact, landing::Landing, nav::Nav};

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    theme: UseReducerHandle<ThemeState>,
    theme_cycle: Vec<&'static str>,
}

#[function_component]
fn App() -> Html {
    let theme: UseReducerHandle<ThemeState> = use_theme_context();
    let theme_cycle: Vec<&str> = vec!["light", "dark"];

    html! {
            <ContextProvider<AppContext> context={AppContext {
                theme: theme.clone(),
                theme_cycle: theme_cycle
            }}>
                <main class={theme.current}>
                    <div class="w-full h-full bg-gray-50 dark:bg-slate-900 text-black dark:text-slate-300 transition">
                        <div class="max-w-[1200px] m-auto p-4">
                            <Nav />
                            <Landing />
                            <Aboutme />
                            <Contact />
                        </div>
                    </div>
                </main>
            </ContextProvider<AppContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
