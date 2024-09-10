use yew::prelude::*;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <div class="flex flex-col justify-center items-center">
            <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-12">
                { "HELLO, FRIEND" }
            </h1>
            <img
            src="./assets/images/profile.jpg"
            alt="ciccio"
            class="max-h-[500px] shadow-md rounded-full"
            />
        </div>
    }
}

