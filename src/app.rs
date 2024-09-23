use std::default;

use crate::components::nav::Nav;
use crate::contexts::theme::{use_theme_context, ThemeState};
use crate::contexts::url::{use_url_context, UrlState};
use crate::routes::RouteOutlet;
use yew::{function_component, html, ContextProvider, Html, UseReducerHandle};
use yew_router::switch;

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub theme: UseReducerHandle<ThemeState>,
    pub theme_cycle: Vec<&'static str>,
    pub url: UseReducerHandle<UrlState>,
}

const REPO: &str = "intx4/intx4.github.io/";
const IS_MASTER: bool = true; // set to true to exclude the BRANCH_PREFIX when fetching assets
const BRANCH_PREFIX: &str = "refs/heads/";
const BRANCH_NAME: &str = "rust_wasm/"; // set to main, master or whatever branch name

#[derive(PartialEq)]
pub enum AssetType {
    Generic, // leaves in top level assets
    Image,
    BlogPost,
}

impl Default for AssetType {
    fn default() -> Self {
        return AssetType::Generic;
    }
}

pub fn get_raw_contents_url(content_name: &str, content_type: AssetType) -> String {
    let mut url = "https://raw.githubusercontent.com/".to_owned();
    url.push_str(REPO);

    if !IS_MASTER || content_type == AssetType::BlogPost {
        url.push_str(BRANCH_PREFIX);
    }

    url.push_str(BRANCH_NAME);
    url.push_str("/src/assets/");
    match content_type {
        AssetType::Image => url.push_str(format!("images/{}", content_name).as_str()),
        AssetType::BlogPost => url.push_str(format!("posts/{}", content_name).as_str()),
        _ => url.push_str(content_name),
    }
    return url;
}

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
            <main class={format!("{} flex-grow", theme.current)} style="min-height: 100vh; width: 100vw; overflow-x: hidden;">
                <div class="w-full min-h-screen h-full bg-gray-50 dark:bg-slate-900 text-black dark:text-slate-300 transition flex flex-col justify-between">
                    <div class="max-w-[1200px] m-auto p-4 flex-grow">
                        <Nav />
                        <RouteOutlet />
                    </div>
                </div>
            </main>
        </ContextProvider<AppContext>>
    }
}
