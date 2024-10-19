use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TypingProps {
    pub children: String,
    pub delay: usize,
}

#[function_component(Typing)]
fn typing(props: &TypingProps) -> Html {
    let text_length = props.children.len();

    html! {
    <span
      class="type"
      style={format!("--n: {}; --delay: {};", text_length, props.delay)}
    >
        { props.children.clone() }
    </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct WhoamiProps {
    pub lines: Vec<String>,
}

#[function_component(Whoami)]
pub fn whoami(props: &WhoamiProps) -> Html {
    html! {
        <>
            <div class="flex-row" style="display:flex; white-space:nowrap; overflow:hidden; align-items:baseline">
                <h1 class="font-bold text-xl text-terminal">{"~$ whoami"}</h1>
                <h1 class="font-bold text-xl terminal">{" _"}</h1>
            </div>
            <br />
            {
                for props.lines.iter().enumerate().map(|(i, line)| {
                    html! {
                        <Typing delay={i} children={line.clone()} />
                    }
                })
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct TerminalWindProps {
    pub children: Children,
}

#[function_component(TerminalWind)]
pub fn terminalWind(props: &TerminalWindProps) -> Html {
    html! {
        <>
        <style>
        {"
            .terminal-window {
                background-color: black; 
                color: rgb(0, 128, 0); 
                font-family: monospace; 
                padding: 1rem; 
                border-radius: 8px; 
                box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
                margin: 1rem auto; 
                width: calc(100% - 40px); /* Adjust width to account for padding */
                max-width: 900px; 
                position: relative; 
                overflow: hidden; 
                box-sizing: border-box; /* Include padding in the element's total width and height */
            }
        
            .terminal-header {
                background-color: #333; 
                color: white; 
                padding: 0.5rem 1rem; 
                border-top-left-radius: 8px; 
                border-top-right-radius: 8px; 
                display: flex; 
                align-items: center; 
                justify-content: flex-start; 
            }
            
            @media (max-width: 650px) {
                .terminal-window {
                    width: calc(100% - 20px); /* Ensures it fits within smaller screens */
                }
            }
        "}
        </style>

        <div class="terminal-window flex justify-left" style="
            background-color: black; 
            color: green; 
            font-family: monospace; 
            padding: 1rem; 
            border-radius: 8px; 
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
            position: relative;
        ">
            <div style="
                background-color: #333; 
                color: white; 
                padding: 0.5rem 1rem; 
                border-top-left-radius: 8px; 
                border-top-right-radius: 8px;
                display: flex; 
                align-items: center;
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
            ">
                <div style="width: 12px; height: 12px; background-color: red; border-radius: 50%; margin-right: 0.5rem;"></div>
                <div style="width: 12px; height: 12px; background-color: yellow; border-radius: 50%; margin-right: 0.5rem;"></div>
                <div style="width: 12px; height: 12px; background-color: green; border-radius: 50%;"></div>
            </div>

            // Terminal Content
            <div style="padding-top: 2rem;"> // Adds padding to make space for the terminal header.
                { props.children.clone() }
            </div>
        </div>
        </>
    }
}

#[function_component(Aboutme)]
pub fn aboutme() -> Html {
    html! {
        <TerminalWind>
                <section class="flex flex-col gap-4 justify-center">
                    <Whoami lines={vec![
                        "==> Living in Zurich, coming from Sicily".to_string(),
                        "==> working as R&D Blockchain Engineer @ Taurus".to_string(),
                        "==> interests: security, privacy and cryptography engineering".to_string(),
                    ]} />
                </section>
        </TerminalWind>
    }
}
