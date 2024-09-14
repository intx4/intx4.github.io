use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Home;
use crate::components::not_found::NotFound;

#[derive(Clone, Routable, PartialEq, Debug)]
enum Routes {
    #[at("/home")]
    Home,
    //#[to = "/#xamarin-why-and-why-not"]
    //Blog1Route,
    //#[to = "/#cat-dead-or-not"]
    //Blog2Route,
    //#[to = "/#blog"]
    //BlogRoute,
    #[at("/")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Home | Routes::Root => html! { <Home /> },
        // RootRoutes::Projects => html! { <Projects /> },
        //Routes::Post { filename } => html! {<Post filename={filename.clone()} />},
        _ => html! {< NotFound />}
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
