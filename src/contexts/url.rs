use gloo::console;
use std::rc::Rc;
use web_sys::{window, MediaQueryList, Window};
use yew::{use_reducer, Hook, Reducible, UseReducerHandle};

#[derive(PartialEq, Debug)]
pub struct UrlState {
    pub current: String,
}

pub fn use_url_context() -> impl Hook<Output = UseReducerHandle<UrlState>> + 'static {
    use_reducer(UrlState::default)
}

// Default implementation to initialize with the current URL
impl Default for UrlState {
    fn default() -> Self {
        // Get the current URL from the window object
        let window: Window = window().expect("No Window Object!");
        let current_url = window
            .location()
            .pathname()
            .unwrap_or_else(|_| "/".to_string());
        console::log!(format!("Current url: {}", current_url));

        Self {
            current: current_url, // Assign directly as an owned String
        }
    }
}

// Action to update the URL state
pub enum UrlAction {
    SetUrl(String),
}

impl Reducible for UrlState {
    type Action = UrlAction;

    // The reducer function that handles actions and updates the state
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            UrlAction::SetUrl(new_url) => {
                // Access the window object and update the location
                if let Some(window) = window() {
                    let location = window.location();
                    if let Err(err) = location.set_pathname(&new_url) {
                        console::error!(format!("Failed to set pathname: {:?}", err));
                    }
                }
                UrlState { current: new_url }.into()
            }
        }
    }
}
