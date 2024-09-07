use yew::prelude::*;
use crate::ui::whoami::Whoami;

#[function_component(Landing)]
pub fn landing() -> Html {
    html!{
        <div class="flex flex-col justify-center items-center">
    <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-28">
        { "Hello! I'm Francesco" }
    </h1>
    <img
        src="./assets/images/me.jpg"
        alt="ciccio"
        class="max-h-[250px] shadow-md rounded-full"
    />
</div>
    }
}
