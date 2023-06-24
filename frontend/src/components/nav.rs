use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::Route;

struct NavItem {
    link: Route,
    label: String,
    is_active: bool,
    id: u32,
}

#[function_component(Nav)]
pub fn nav() -> Html {
    let nav_items = use_state(|| {
        vec![
            NavItem {
                link: Route::Home,
                label: "Home".to_owned(),
                is_active: false,
                id: 0,
            },
            NavItem {
                link: Route::SignupForm,
                label: "Signup".to_owned(),
                is_active: false,
                id: 1,
            },
            NavItem {
                link: Route::LoginForm,
                label: "Login".to_owned(),
                is_active: false,
                id: 2,
            },
        ]
    });

    html! {
        <nav class="nav">
        <Link<Route> classes={classes!("logo")} to={Route::Home}>
        <img class={classes!("logo-img")} src="logo_transparent.png" alt="Logo" />
        </Link<Route>>
        <ul class="nav-list">
        {
            nav_items.iter().map(|nav_item| {
                html!{<li key={nav_item.id} class={classes!("nav-item", if nav_item.is_active { "active" } else { "" })}>
                <Link<Route> to={nav_item.link.clone()}>{nav_item.label.clone()}</Link<Route>>
                    </li>}
            }).collect::<Html>()
        }
        </ul>
        <ul class="nav-social-list">
        
        </ul>
        </nav>
    }
}