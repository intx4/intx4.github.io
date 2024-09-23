use yew::{function_component, html, Html};
use crate::blog::post::previews::BlogPostsPreviews;

#[function_component()]
pub fn Blog() -> Html {
    let previews = BlogPostsPreviews();

    html! {
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 p-4">
            { for previews }
        </div>
    }
}
