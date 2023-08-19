use crate::pages::{home_page::HomePage, list_page::ListPage, register_page::RegisterPage};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/register")]
    RegisterPage,
    // #[at("/login")]
    // LoginPage,
    // #[at("/profile")]
    // ProfilePage,
    // #[at("/about")]
    // AboutPage,
    #[at("/lists")]
    ListPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage /> },
        Route::RegisterPage => html! {<RegisterPage /> },
        Route::ListPage => html! {<ListPage /> },
    }
}
