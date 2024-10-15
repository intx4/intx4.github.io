use crate::blog::{post::post_preview::BlogPostPreview, post::post_preview::PostPreviewColor};
use yew::prelude::*;

pub fn BlogPostsPreviews() -> Vec<Html> {
    // add new post previews here
    vec![html! {
        <BlogPostPreview
            img="./assets/images/qc.jpg"
            title="Did China break 'military-grade' encryption?"
            description={"Once again, cryptography doom's day!"}
            date={"14 October, 2024"}
            link="/blog/post/1" // next post must be 2
            color={PostPreviewColor::Black}
        />
    }]
}
