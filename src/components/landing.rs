use yew::prelude::*;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <div class="flex flex-col justify-center items-center">
            <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-8">
                { "Francesco INTOCI" }
            </h1>
            <h2 class="text-center text-2xl font-mono py-8">
                { "R&D Blockchain Engineer" }
            </h2>
            <img
            src="./assets/images/profile.jpg"
            alt="ciccio"
            class="max-h-[500px] shadow-md rounded-full"
            />
        </div>
    }
}
