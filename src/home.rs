use yew::{function_component, html};
use yew::prelude::*;

use crate::components::{aboutme::Aboutme, contact::Contact, landing::Landing, nav::Nav};

#[function_component(Home)]
pub fn home() -> Html {
   html! {
        <div>
            <Landing />
            <Aboutme />
            <Contact />
        </div>
    }
}