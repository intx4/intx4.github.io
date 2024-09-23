use crate::contexts::url::{use_url_context, UrlAction, UrlState};
use yew::prelude::*; // Adjust the import path as necessary

#[function_component(NotFound)]
pub fn not_found() -> Html {
    // Get the URL state and dispatcher
    let url: UseReducerHandle<UrlState> = use_url_context();

    // Use use_effect to perform URL state update when the component mounts
    use_effect(move || {
        // Check if the current URL is not "/404"
        if url.current != "/404" {
            url.dispatch(UrlAction::SetUrl("/404".to_string()));
        }
        || () // Return a no-op closure to satisfy the use_effect signature
    });

    html! {
        <div class="flex flex-col justify-center items-center">
            <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-12">
                { "404: NOT FOUND" }
            </h1>
        </div>
    }
}
