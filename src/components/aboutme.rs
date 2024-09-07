use yew::prelude::*;
use crate::ui::{TerminalCard, Whoami};

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
                        <p>{"Lorem ipsum "}</p>
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
