use crate::blog::{post::post_preview::BlogPostPreview, post::post_preview::PostPreviewColor};
use yew::prelude::*;

pub fn BlogPostsPreviews() -> Vec<Html> {
    // add new post previews here
    vec![
        html! {
            <BlogPostPreview
                img="./assets/images/profile.jpg"
                title="post1"
                description={"a description"}
                link="/post1"
                color={PostPreviewColor::Black}
            />
        },
    ]
}