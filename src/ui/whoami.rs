use yew::{function_component, html, Properties, Html};

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
                        from { width: 0; }
                        to { width: 100%; }
                    }
                    @keyframes blink-caret {
                        from, to { border-color: transparent; }
                        50% { border-color: green; }
                    }
                    .typing-effect {
                        font-size: 1rem;
                        color: green;
                        white-space: nowrap;
                        overflow: hidden;
                        border-right: 0.15em solid green;
                        width: 0; /* Initial width for the animation */
                        animation:
                            typing 0.15s steps(40, end) forwards, /* Typing effect */
                            blink-caret 0.66s step-end infinite; /* Blinking caret */
                    }
                "}
            </style>
            <div class="font-bold typing-effect">
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
                <h1 class="font-bold text-xl text-terminal">{"~$ whoami"}</h1>
            </div>
            <br />
            <Typing>{props.children.clone()}</Typing>
        </>
    }
}