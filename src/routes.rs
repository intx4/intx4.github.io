use yew::prelude::*;
use yew_router::prelude::*;

use crate::blog::{blog::Blog, post::posts::BlogPosts};
use crate::components::not_found::NotFound;
use crate::home::Home;

#[derive(Clone, Routable, PartialEq, Debug)]
enum Routes {
    #[at("/home")]
    Home,
    #[at("/blog")]
    BlogRoute,
    #[at("/")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/blog/post/:index")]
    BlogPostRoute { index: usize },
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Home | Routes::Root => html! { <Home /> },
        Routes::BlogRoute => html! {<Blog />},
        Routes::BlogPostRoute { index } => {
            let posts = BlogPosts();
            if index > 0 && index <= posts.len() {
                html! { posts[index - 1].clone() }
            } else {
                html! { <NotFound /> }
            }
        }
        _ => html! {< NotFound />},
    }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Routes> render={switch} />
        </BrowserRouter>
    }
}
