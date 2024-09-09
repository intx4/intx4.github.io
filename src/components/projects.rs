use crate::components::svg::emojis::Robot;
use crate::ui::Title;
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
    <>
        <Title id="projects">
            { "Projects" }{ " " }
            <Robot />
        </Title>
        <div>{"TODO"}</div>
    </>
    }
}
