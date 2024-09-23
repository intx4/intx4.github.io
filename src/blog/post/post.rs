use pulldown_cmark::{Options, Parser};
use reqwest::get;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html, Html};

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub url: String,
}

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let content = use_state_eq(|| String::new());

    {
        let content = content.clone();
        let url = props.url.clone();
        use_effect(move || {
            let content = content.clone();
            let url = url.clone();

            spawn_local(async move {
                match get(url.as_str()).await {
                    Ok(response) => {
                        if let Ok(text) = response.text().await {
                            let mut options = Options::empty();
                            options.insert(Options::ENABLE_STRIKETHROUGH);
                            let parser = Parser::new_ext(&text, options);
                            let mut html_output = String::new();
                            pulldown_cmark::html::push_html(&mut html_output, parser);
                            content.set(html_output);
                        }
                    }
                    Err(err) => {
                        gloo::console::error!(format!("Failed to fetch markdown: {:?}", err));
                    }
                }
            });

            || ()
        });
    }

    html! {
        <>
        <style>
        {r#"
            .blog-post {
                max-width: 99%;
                margin: 1rem auto;
                padding: 20px;
                background-color: #ffffff;
                border: 1px solid #e1e4e8;
                border-radius: 8px;
                box-shadow: 0px 4px 6px rgba(0, 0, 0, 0.05);
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji';
                line-height: 1.5;
                word-wrap: break-word;
            }
            .blog-post h1, .blog-post h2, .blog-post h3 {
                color: #24292e;
                font-weight: 600;
                margin-bottom: 16px;
                padding-bottom: 0.3em;
                border-bottom: 1px solid #eaecef;
            }
            .blog-post h1 {
                font-size: 2em;
            }
            .blog-post h2 {
                font-size: 1.5em;
            }
            .blog-post h3 {
                font-size: 1.25em;
            }
            .blog-post p {
                color: #24292e;
                margin-bottom: 16px;
            }
            .blog-post a {
                color: #0366d6;
                text-decoration: none;
            }
            .blog-post a:hover {
                text-decoration: underline;
            }
            .blog-post img {
                max-width: 100%;
                height: auto;
                border-radius: 4px;
                margin-top: 16px;
            }
            .blog-post pre {
                background-color: #f6f8fa;
                padding: 16px;
                border-radius: 6px;
                font-size: 85%;
                line-height: 1.45;
                overflow-x: auto;
                border: 1px solid #d1d5da;
            }
            .blog-post code {
                background-color: #f6f8fa;
                padding: 0.2em 0.4em;
                border-radius: 3px;
                font-family: SFMono-Regular, Consolas, 'Liberation Mono', Menlo, Courier, monospace;
                font-size: 85%;
            }
            .blog-post blockquote {
                padding: 0.5em 1em;
                margin-left: 0;
                border-left: 0.25em solid #dfe2e5;
                color: #6a737d;
                background-color: #f6f8fa;
            }
            .blog-post ul {
                padding-left: 2em;
                margin-bottom: 16px;
            }
            .blog-post ol {
                padding-left: 2em;
                margin-bottom: 16px;
            }
            .blog-post li {
                margin-bottom: 0.25em;
            }
            .blog-post table {
                display: block;
                width: 100%;
                overflow: auto;
                margin-bottom: 16px;
                border-spacing: 0;
                border-collapse: collapse;
            }
            .blog-post table th,
            .blog-post table td {
                padding: 6px 13px;
                border: 1px solid #dfe2e5;
            }
            .blog-post table th {
                background-color: #f6f8fa;
            }
            .blog-post table tr {
                background-color: #ffffff;
            }
            .blog-post table tr:nth-child(2n) {
                background-color: #f6f8fa;
            }
            .content {
                text-align: center;
            }
        "#}
        </style>
        <div class="blog-post">
            <div class="content">
                {Html::from_html_unchecked(AttrValue::from(content.clone().to_string()))}
            </div>
        </div>
        </>
    }
}
