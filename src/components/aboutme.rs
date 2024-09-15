use crate::ui::TerminalWind;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(Typing)]
fn typing(props: &Props) -> Html {
    html! {
    <>
    <style>
    {"
@keyframes typing {
  from {
    width: 0;
    visibility: visible;
  }
  to {
    width: 100%;
    visibility: visible;
  }
}

p {
  overflow: hidden;
  white-space: nowrap;
  font-family: monospace;
  font-size: 20px;
  color: green;
  visibility: hidden; /* Initially hide all lines */
  animation: typing 1s steps(40, end) forwards;
}

/* Specific delays for each line */
p:nth-child(1) {
  animation-delay: 0s;
  animation-fill-mode: forwards; /* Ensure line stays visible after typing */
  visibility: visible; /* Make the first line visible when its animation starts */
}

p:nth-child(2) {
  animation-delay: 1s;
  animation-fill-mode: forwards;
}

p:nth-child(3) {
  animation-delay: 2s;
  animation-fill-mode: forwards;
}

    "}
    </style>
    <div class="text-xl">
        { props.children.clone() }
    </div>
    </>
    }
}

#[function_component(Whoami)]
pub fn whoami(props: &Props) -> Html {
    html! {
        <>
            <div class="flex-row" style="display:flex; white-space:nowrap; overflow:hidden; align-items:baseline">
                <h1 class="font-bold text-xl text-terminal">{"~$ whoami"}</h1><h1 class="font-bold text-xl terminal">{" _"}</h1>
            </div>
            <br />
            <Typing>{props.children.clone()}</Typing>
        </>
    }
}

#[function_component(Aboutme)]
pub fn aboutme() -> Html {
    html! {
        <TerminalWind >
            <div class="flex gap-4 justify-between items-center max-[650px]:flex-col max-[650px]:gap-8">
                <section class="flex flex-col gap-4 justify-center">
                    <Whoami>
                        // each line should live in its own <p>
                        // add a child in the typing style: p:nth-child(i)
                        // and adjust the timing of the animation
                        <p>{"==> Francesco Intoci: SWE living in Zurich, coming from Sicily"}</p>
                        <p>{"==> currently working in Blockchain @ Taurus"}</p>
                        <p>{"==> strong interest in security, privacy and cryptography engineering"}</p>
                    </Whoami>
                </section>
            </div>
        </TerminalWind>
    }
}
