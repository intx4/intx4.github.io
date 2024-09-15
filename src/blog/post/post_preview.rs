use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(PartialEq)]
pub enum PostPreviewColor {
    Black,
    LightBlue,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub img: AttrValue,
    pub title: Html,
    pub description: Html,
    pub link: AttrValue,
    #[prop_or(PostPreviewColor::LightBlue)]
    pub color: PostPreviewColor,
}

#[function_component(BlogPostPreview)]
pub fn blog_post_preview(props: &Props) -> Html {
    html! {
        <a href={props.link.clone()} class="block">
            <div class="bg-white border border-gray-200 p-4 rounded-lg dark:bg-slate-800 dark:border-slate-700 flex flex-col items-center text-center cursor-pointer">
                <div class="flex flex-col gap-4">
                    <h4 class="text-2xl font-bold font-mono py-2 px-3 rounded-lg text-center">
                        { props.title.clone() }
                    </h4>
                    <p>{ props.description.clone() }</p>
                    if !props.img.is_empty() {
                        <img
                            class="w-full max-w-[250px] shadow border border-gray-200 dark:border-slate-700"
                            src={ props.img.clone() }
                            alt="project img"
                            loading="lazy"
                        />
                    }
                </div>
            </div>
        </a>
    }
}
