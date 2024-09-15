use crate::contexts::theme::{use_theme_context, ThemeState};
use crate::contexts::url::{use_url_context, UrlState};
use yew::{function_component, html, ContextProvider, Html, UseReducerHandle};
use crate::components::nav::Nav;
use crate::routes::RouteOutlet;

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub theme: UseReducerHandle<ThemeState>,
    pub theme_cycle: Vec<&'static str>,
    pub url: UseReducerHandle<UrlState>,
}

//#[function_component(Home)]
//pub fn home() -> Html {
//    let theme: UseReducerHandle<ThemeState> = use_theme_context();
//    let theme_cycle: Vec<&str> = vec!["light", "dark"];
//
//    html! {
//        <ContextProvider<AppContext> context={AppContext {
//            theme: theme.clone(),
//            theme_cycle: theme_cycle
//        }}>
//            <main class={format!("{} min-h-screen", theme.current)}> // Ensure main has min height of the screen
//                <div class="w-full min-h-screen bg-gray-50 dark:bg-slate-900 text-black dark:text-slate-300 transition flex flex-col justify-between">
//                    <div class="max-w-[1200px] m-auto p-4 flex-grow"> // Add flex-grow to allow it to stretch
//                        <Nav />
//                        <Landing />
//                        <Aboutme />
//                        <Contact />
//                    </div>
//                </div>
//            </main>
//        </ContextProvider<AppContext>>
//    }
//}

#[function_component(App)]
pub fn app() -> Html {
    let theme: UseReducerHandle<ThemeState> = use_theme_context();
    let theme_cycle: Vec<&str> = vec!["light", "dark"];
    let url: UseReducerHandle<UrlState> = use_url_context();

    html! {
        <ContextProvider<AppContext> context={AppContext {
            theme: theme.clone(),
            theme_cycle: theme_cycle,
            url: url,
        }}>
            <main class={format!("{} min-h-screen", theme.current)}> // Ensure main has min height of the screen
                <div class="w-full min-h-screen bg-gray-50 dark:bg-slate-900 text-black dark:text-slate-300 transition flex flex-col justify-between">
                    <div class="max-w-[1200px] m-auto p-4 flex-grow"> // Add flex-grow to allow it to stretch
                        <Nav />
                        <RouteOutlet />
                    </div>
                </div>
            </main>
        </ContextProvider<AppContext>>   
    }
}
