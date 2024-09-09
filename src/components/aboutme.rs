use yew::prelude::*;
use crate::ui::TerminalCard;

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
    //let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    // add new programming langs here
    //let programming_languages: [&str; 1] = [""];

    //fn aboutme_bulletpoints(app_context: AppContext, programming_languages: [&'static str; 1]) -> Html {
    //    let programming_languages: Vec<Html> = programming_languages.into_iter().map(|lang: &'static str| {
    //        html! {
    //            <img
    //                src={lang}
    //                alt={lang}
    //                class="inline-block relative -top-[0.1rem] h-[1.2rem] w-[1.2rem] rounded ml-[0.2rem]"
    //            />
    //        }
    //    }).collect::<Vec<Html>>();

    //    html! {
    //            <>
    //                <li>
    //                    {"and programming nerd"}
    //                    {programming_languages}
    //                </li>
    //            </>
    //        }
    //    }

    html!{
        <TerminalCard >
            <div class="flex gap-4 justify-between items-center max-[650px]:flex-col max-[650px]:gap-8">
                <section class="flex flex-col gap-4 justify-center">
                    <Whoami>
                        // each line should live in its own <p>
                        // add a child in whoami.rs (typing style)
                        <p>{"==> Francesco Intoci: SWE living in Zurich, coming from Sicily"}</p>
                        <p>{"    currently working in Blockchain @ Taurus"}</p>
                        <p>{"    strong interest in security, privacy and cryptography engineering"}</p>
                    </Whoami>
                    //<ul class="pl-4 list-disc flex flex-col gap-4 items-between [&>li>a>svg]:w-[1.8rem] [&>li>a>svg]:h-[1.8rem] [&>li>a>svg]:relative [&>li>a>svg]:-top-[0.1rem] [&>li>a>svg]:dark:fill-white [&>li>a>svg]:dark:stroke-white">
                    //    {aboutme_bulletpoints(app_context.clone(), programming_languages)}
                    //</ul>
                </section>
            </div>
            //<div class="flex flex-col gap-4 [&>p>svg]:w-[1.5rem] [&>p>svg]:h-[1.5rem]">
            //    { translate_aboutme_content(app_context) }
            //</div>
        </TerminalCard>
    }
}
