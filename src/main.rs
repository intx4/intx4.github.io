use yew::prelude::*;

const OWNER: &str = "Francesco Intoci";

struct Index;
impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h1>
                { format!("Hey visitor! Welcome to {} website.\n I am currently revamping it after a long period of inactivity, converting this into a fun side project...stay tuned!", OWNER) }
            </h1>
        }
    }
}

fn main() {
    yew::start_app::<Index>();
}
