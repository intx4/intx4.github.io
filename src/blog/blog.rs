use crate::blog::post::previews::BlogPostsPreviews;
use yew::{function_component, html, Html};

#[function_component()]
pub fn Blog() -> Html {
    let previews = BlogPostsPreviews();

    html! {
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 p-4">
            { for previews }
        </div>
    }
}
