use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <nav>
                <ul>
                    <li class="nav-item">
                        <a class="nav-link" href="#">{"Home"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="#">{"About"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="#">{"Sign-in"}</a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
