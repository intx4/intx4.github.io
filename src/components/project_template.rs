use yew::{function_component, html, Html};
use crate::ui::{ProjectCard, ProjectCardColor};

#[function_component(ProjectTemplate)]
pub fn project_template() -> Html {
    html! {
        <ProjectCard
            img="./assets/images/babbeln.png"
            title="babbeln"
            subtitle={"subtitle"}
            description={"a description"}
            page_link=""
            source_link=""
            color={ProjectCardColor::Black}
        />
    }
}
