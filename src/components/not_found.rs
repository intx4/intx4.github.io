use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="flex flex-col justify-center items-center">
            <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-12">
                { "404: NOT FOUND" }
            </h1>
        </div>
    }
}

