use crate::blog::post::post::BlogPost;
use yew::prelude::*;

const NumPosts: u8 = 1;

pub fn BlogPosts() -> Vec<Html> {
    // add new posts
    vec![
        html! {
            <BlogPost
                url="https://raw.githubusercontent.com/intx4/intx4.github.io/refs/heads/rust-wasm/blog/src/assets/posts/1.md"
            />
        },
    ]
}