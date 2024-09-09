use crate::components::svg::props::{handle_props_class, Props};
use yew::prelude::*;

#[function_component(Handshake)]
pub fn handshake(props: &Props) -> Html {
    html! {
    <svg class={ handle_props_class(props) } xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36"><path fill="#EF9645" d="M4.861 9.147c.94-.657 2.357-.531 3.201.166l-.968-1.407c-.779-1.111-.5-2.313.612-3.093 1.112-.777 4.263 1.312 4.263 1.312-.786-1.122-.639-2.544.483-3.331 1.122-.784 2.67-.513 3.456.611l10.42 14.72L25 31l-11.083-4.042L4.25 12.625c-.793-1.129-.519-2.686.611-3.478z"/><path fill="#FFDC5D" d="M2.695 17.336s-1.132-1.65.519-2.781c1.649-1.131 2.78.518 2.78.518l5.251 7.658c.181-.302.379-.6.6-.894L4.557 11.21s-1.131-1.649.519-2.78c1.649-1.131 2.78.518 2.78.518l6.855 9.997c.255-.208.516-.417.785-.622L7.549 6.732s-1.131-1.649.519-2.78c1.649-1.131 2.78.518 2.78.518l7.947 11.589c.292-.179.581-.334.871-.498L12.238 4.729s-1.131-1.649.518-2.78c1.649-1.131 2.78.518 2.78.518l7.854 11.454 1.194 1.742c-4.948 3.394-5.419 9.779-2.592 13.902.565.825 1.39.26 1.39.26-3.393-4.949-2.357-10.51 2.592-13.903L24.515 8.62s-.545-1.924 1.378-2.47c1.924-.545 2.47 1.379 2.47 1.379l1.685 5.004c.668 1.984 1.379 3.961 2.32 5.831 2.657 5.28 1.07 11.842-3.94 15.279-5.465 3.747-12.936 2.354-16.684-3.11L2.695 17.336z"/><g fill="#5DADEC"><path d="M12 32.042C8 32.042 3.958 28 3.958 24c0-.553-.405-1-.958-1s-1.042.447-1.042 1C1.958 30 6 34.042 12 34.042c.553 0 1-.489 1-1.042s-.447-.958-1-.958z"/><path d="M7 34c-3 0-5-2-5-5 0-.553-.447-1-1-1s-1 .447-1 1c0 4 3 7 7 7 .553 0 1-.447 1-1s-.447-1-1-1zM24 2c-.552 0-1 .448-1 1s.448 1 1 1c4 0 8 3.589 8 8 0 .552.448 1 1 1s1-.448 1-1c0-5.514-4-10-10-10z"/><path d="M29 .042c-.552 0-1 .406-1 .958s.448 1.042 1 1.042c3 0 4.958 2.225 4.958 4.958 0 .552.489 1 1.042 1s.958-.448.958-1C35.958 3.163 33 .042 29 .042z"/></g></svg>
    }
}

#[function_component(Heart)]
pub fn heart(props: &Props) -> Html {
    html! {
    <svg class={ handle_props_class(props) } viewBox="0 0 16 16" fill="red" xmlns="http://www.w3.org/2000/svg">
              <path d="M4 1c2.21 0 4 1.755 4 3.92C8 2.755 9.79 1 12 1s4 1.755 4 3.92c0 3.263-3.234 4.414-7.608 9.608a.513.513 0 0 1-.784 0C3.234 9.334 0 8.183 0 4.92 0 2.755 1.79 1 4 1z"/>
        </svg>
    }
}

#[function_component(Nerd)]
pub fn nerd(props: &Props) -> Html {
    html! {
                <svg class={ handle_props_class(props) } xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36"><circle fill="#FFCC4D" cx="18" cy="18" r="18"/><path fill="#664500" d="M27.335 23.629c-.178-.161-.444-.171-.635-.029-.039.029-3.922 2.9-8.7 2.9-4.766 0-8.662-2.871-8.7-2.9-.191-.142-.457-.13-.635.029-.177.16-.217.424-.094.628C8.7 24.472 11.788 29.5 18 29.5s9.301-5.028 9.429-5.243c.123-.205.084-.468-.094-.628z"/><path fill="#65471B" d="M18 26.591c-.148 0-.291-.011-.438-.016v4.516h.875v-4.517c-.145.005-.289.017-.437.017z"/><path fill="#FFF" d="M22 26c.016-.004-1.45.378-2.446.486-.366.042-.737.076-1.117.089v4.517H20c1.1 0 2-.9 2-2V26zm-8 0c-.016-.004 1.45.378 2.446.486.366.042.737.076 1.117.089v4.517H16c-1.1 0-2-.9-2-2V26z"/><path fill="#65471B" d="M27.335 23.629c-.178-.161-.444-.171-.635-.029-.03.022-2.259 1.668-5.411 2.47-.443.113-1.864.43-3.286.431-1.424 0-2.849-.318-3.292-.431-3.152-.802-5.381-2.448-5.411-2.47-.19-.142-.457-.132-.635.029-.178.16-.217.423-.094.628.097.162 1.885 3.067 5.429 4.481v-1.829c-.016-.004 1.45.378 2.446.486.366.042.737.076 1.117.089.146.005.289.016.437.016.148 0 .291-.011.438-.016.38-.013.751-.046 1.117-.089.996-.108 2.462-.49 2.446-.486v1.829c3.544-1.414 5.332-4.319 5.429-4.481.122-.205.083-.468-.095-.628zm-.711-9.605c0 1.714-.938 3.104-2.096 3.104-1.157 0-2.096-1.39-2.096-3.104s.938-3.104 2.096-3.104c1.158 0 2.096 1.39 2.096 3.104zm-17.167 0c0 1.714.938 3.104 2.096 3.104 1.157 0 2.096-1.39 2.096-3.104s-.938-3.104-2.096-3.104c-1.158 0-2.096 1.39-2.096 3.104z"/><path fill="#292F33" d="M34.808 9.627c-.171-.166-1.267.274-2.376-.291-2.288-1.166-8.07-2.291-11.834.376-.403.285-2.087.333-2.558.313-.471.021-2.155-.027-2.558-.313-3.763-2.667-9.545-1.542-11.833-.376-1.109.565-2.205.125-2.376.291-.247.239-.247 1.196.001 1.436.246.239 1.477.515 1.722 1.232.247.718.249 4.958 2.213 6.424 1.839 1.372 6.129 1.785 8.848.238 2.372-1.349 2.289-4.189 2.724-5.881.155-.603.592-.907 1.26-.907s1.105.304 1.26.907c.435 1.691.351 4.532 2.724 5.881 2.719 1.546 7.009 1.133 8.847-.238 1.965-1.465 1.967-5.706 2.213-6.424.245-.717 1.476-.994 1.722-1.232.248-.24.249-1.197.001-1.436zm-20.194 3.65c-.077 1.105-.274 3.227-1.597 3.98-.811.462-1.868.743-2.974.743h-.001c-1.225 0-2.923-.347-3.587-.842-.83-.619-1.146-3.167-1.265-4.12-.076-.607-.28-2.09.388-2.318 1.06-.361 2.539-.643 4.052-.643.693 0 3.021.043 4.155.741 1.005.617.872 1.851.829 2.459zm16.278-.253c-.119.954-.435 3.515-1.265 4.134-.664.495-2.362.842-3.587.842h-.001c-1.107 0-2.163-.281-2.975-.743-1.323-.752-1.52-2.861-1.597-3.966-.042-.608-.176-1.851.829-2.468 1.135-.698 3.462-.746 4.155-.746 1.513 0 2.991.277 4.052.638.668.228.465 1.702.389 2.309z"/></svg>
    }
}

#[function_component(Robot)]
pub fn robot(props: &Props) -> Html {
    html! {
                <svg class={ handle_props_class(props) } xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36"><ellipse fill="#F4900C" cx="33.5" cy="14.5" rx="2.5" ry="3.5"/><ellipse fill="#F4900C" cx="2.5" cy="14.5" rx="2.5" ry="3.5"/><path fill="#FFAC33" d="M34 19c0 .553-.447 1-1 1h-3c-.553 0-1-.447-1-1v-9c0-.552.447-1 1-1h3c.553 0 1 .448 1 1v9zM7 19c0 .553-.448 1-1 1H3c-.552 0-1-.447-1-1v-9c0-.552.448-1 1-1h3c.552 0 1 .448 1 1v9z"/><path fill="#FFCC4D" d="M28 5c0 2.761-4.478 4-10 4C12.477 9 8 7.761 8 5s4.477-5 10-5c5.522 0 10 2.239 10 5z"/><path fill="#F4900C" d="M25 4.083C25 5.694 21.865 7 18 7c-3.866 0-7-1.306-7-2.917 0-1.611 3.134-2.917 7-2.917 3.865 0 7 1.306 7 2.917z"/><path fill="#269" d="M30 5.5C30 6.881 28.881 7 27.5 7h-19C7.119 7 6 6.881 6 5.5S7.119 3 8.5 3h19C28.881 3 30 4.119 30 5.5z"/><path fill="#55ACEE" d="M30 6H6c-1.104 0-2 .896-2 2v26h28V8c0-1.104-.896-2-2-2z"/><path fill="#3B88C3" d="M35 33v-1c0-1.104-.896-2-2-2H22.071l-3.364 3.364c-.391.391-1.023.391-1.414 0L13.929 30H3c-1.104 0-2 .896-2 2v1c0 1.104-.104 2 1 2h32c1.104 0 1-.896 1-2z"/><circle fill="#FFF" cx="24.5" cy="14.5" r="4.5"/><circle fill="#DD2E44" cx="24.5" cy="14.5" r="2.721"/><circle fill="#FFF" cx="11.5" cy="14.5" r="4.5"/><path fill="#F5F8FA" d="M29 25.5c0 1.381-1.119 2.5-2.5 2.5h-17C8.119 28 7 26.881 7 25.5S8.119 23 9.5 23h17c1.381 0 2.5 1.119 2.5 2.5z"/><path fill="#CCD6DD" d="M17 23h2v5h-2zm-5 0h2v5h-2zm10 0h2v5h-2zM7 25.5c0 1.21.859 2.218 2 2.45v-4.9c-1.141.232-2 1.24-2 2.45zm20-2.45v4.899c1.141-.232 2-1.24 2-2.45s-.859-2.217-2-2.449z"/><circle fill="#DD2E44" cx="11.5" cy="14.5" r="2.721"/></svg>
    }
}

#[function_component(Blushing)]
pub fn blushing(props: &Props) -> Html {
    html! {
        <svg class={ handle_props_class(props) } xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36"><path fill="#FFCC4D" d="M36 18c0 9.941-8.059 18-18 18S0 27.941 0 18 8.059 0 18 0s18 8.059 18 18"/><circle fill="#FF7892" cx="7" cy="18" r="5"/><circle fill="#FF7892" cx="29" cy="18" r="5"/><path fill="#664500" d="M27.335 21.629c-.178-.161-.444-.171-.635-.029-.039.029-3.922 2.9-8.7 2.9-4.766 0-8.662-2.871-8.7-2.9-.191-.142-.457-.13-.635.029-.177.16-.217.424-.094.628C8.7 22.472 11.788 27.5 18 27.5s9.301-5.028 9.429-5.243c.123-.205.084-.468-.094-.628zM7.999 15c-.15 0-.303-.034-.446-.106-.494-.247-.694-.848-.447-1.342C7.158 13.448 8.424 11 12 11c3.577 0 4.842 2.449 4.894 2.553.247.494.047 1.095-.447 1.342-.492.245-1.085.049-1.336-.436C15.068 14.379 14.281 13 12 13c-2.317 0-3.099 1.433-3.106 1.447-.175.351-.528.553-.895.553zm20.002 0c-.367 0-.72-.202-.896-.553C27.08 14.401 26.299 13 24 13s-3.08 1.401-3.112 1.46c-.26.481-.859.67-1.345.42-.485-.252-.682-.839-.438-1.328C19.157 13.449 20.423 11 24 11s4.843 2.449 4.895 2.553c.247.494.047 1.095-.447 1.342-.144.071-.297.105-.447.105z"/></svg>
    }
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    html! {
        <svg
            class={ handle_props_class(props) }
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            width="36"
            height="36"
        >
            <path d="M12 3l-9 9h3v9h12v-9h3l-9-9zm-4 15v-7h2v7h-2zm8 0h-2v-7h2v7zm-1-12.5l-5 5v1.5h10v-1.5l-5-5z"/>
        </svg>
    }
}
