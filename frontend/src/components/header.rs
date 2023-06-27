use crate::store::Store;
use yew_router::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::router::Route;

#[function_component(Header)]
pub fn header_component() -> Html {
    let (store, _) = use_store::<Store>();
    let user = store.auth_user.clone();

    html! {
        <header>

        <nav>
            <Link<Route> classes={"fluke-logo"} to={Route::HomePage} >{"Fluke"}</Link<Route>>
          <ul class="">
            <li>
              <Link<Route> to={Route::HomePage} >{"Home"}</Link<Route>>
            </li>
            if user.is_some() {
               <>
                <li>
                  <Link<Route> to={Route::ProfilePage} classes="">{"Profile"}</Link<Route>>
                </li>
                <li
                >
                  {"Create Post"}
                </li>
                <li class="">
                  {"Logout"}
                </li>
              </>

            } else {
              <>
                <li>
                  <Link<Route> to={Route::RegisterPage} classes="">{"Sign-Up"}</Link<Route>>
                </li>
                <li>
                  <Link<Route> to={Route::LoginPage} classes="">{"Login"}</Link<Route>>
                </li>
              </>
            }
          </ul>
        </nav>
      </header>
    }
}