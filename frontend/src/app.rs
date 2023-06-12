use crate::landing::LandingPage;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <LandingPage />
        </>
    }
}
