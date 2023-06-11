use yew::prelude::*;
use crate::landing::LandingPage;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <LandingPage />
        </>
    }
}

