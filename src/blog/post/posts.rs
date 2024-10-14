use crate::app::{get_raw_contents_url, AssetType};
use crate::blog::post::post::BlogPost;
use yew::prelude::*;

pub fn BlogPosts() -> Vec<Html> {
    // add new posts
    vec![html! {
        <BlogPost
            url={get_raw_contents_url("china_breaks_aes.md", AssetType::BlogPost)}
        />
    }]
}
