use crate::blog::post::post::BlogPost;
use yew::prelude::*;
use crate::app::{get_raw_contents_url, AssetType};

pub fn BlogPosts() -> Vec<Html> {
    // add new posts
    vec![
        html! {
            <BlogPost
                url={get_raw_contents_url("1.md", AssetType::BlogPost)}
            />
        },
    ]
}