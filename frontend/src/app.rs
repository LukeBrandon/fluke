use crate::landing::LandingPage;
use crate::header::Header;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <LandingPage />
        </>
    }
}
