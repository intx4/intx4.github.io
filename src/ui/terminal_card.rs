use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TerminalCardProps {
    pub children: Children, // The children prop will allow for nested text or components inside the TerminalCard.
}

#[function_component(TerminalCard)]
pub fn terminal_card(props: &TerminalCardProps) -> Html {
    html! {
        <div style="
            background-color: black; 
            color: green; 
            font-family: monospace; 
            padding: 1rem; 
            border-radius: 8px; 
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
            width: 100%;
            max-width: 900px;
            margin: 1rem auto;
            position: relative;
        ">
            // Terminal Header (optional)
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
    }
}
