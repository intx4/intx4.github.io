use crate::ui::Button;
use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(PartialEq)]
pub enum ProjectCardColor {
    Black,
    LightBlue,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub img: AttrValue,
    pub title: Html,
    pub subtitle: Html,
    pub description: Html,
    pub page_link: AttrValue,
    pub source_link: AttrValue,
    #[prop_or(ProjectCardColor::LightBlue)]
    pub color: ProjectCardColor,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &Props) -> Html {
    html! {
        <div class="py-8">
            <div
                class="bg-white border border-gray-200 flex flex-wrap gap-4 justify-center p-4 rounded-lg dark:bg-slate-800 dark:border-slate-700"
            >
                // left
                // TODO change min width for smaller screens make it smaller
                <div class="flex flex-col flex-1 grow min-w-[300px] gap-6 self-start max-[850px]:min-w-full">
                    <div class="flex flex-col gap-4">
                        <h4 class="text-2xl font-bold bg-sky-50 dark:bg-blue-300/10 py-2 px-3 w-fit rounded-lg">
                            {props.title.clone()}
                        </h4>
                        <h5 class="text-xl font-bold">{props.subtitle.clone()}</h5>
                        <p>{props.description.clone()}</p>
                    </div>

                    <div class="flex flex-wrap gap-2">
                        <a
                            href={props.page_link.clone()}
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <Button is_primary={true}>
                                {"visit"}
                            </Button>
                        </a>

                        <a
                            href={props.source_link.clone()}
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <Button is_secondary={false}>
                                {"???"}
                            </Button>
                        </a>
                    </div>
                </div>

                // right
                if !props.img.is_empty() {
                    <img
                        class="w-[500px] rounded-lg shadow self-end border border-gray-200 dark:border-slate-700"
                        src={props.img.clone()}
                        alt="project img"
                        loading="lazy"
                    />
                }
            </div>
        </div>
    }
}
