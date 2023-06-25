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
        <header class="bg-dark h-20">
        <nav class="h-full flex justify-between container items-center">
          <div>
            <Link<Route> to={Route::HomePage} classes="fluke-text-primary text-lg text-bold">{"Fluke"}</Link<Route>>
          </div>
          <ul class="flex items-center gap-10">
            <li>
              <Link<Route> to={Route::HomePage} classes="fluke-text-primary text-lg text-bold">{"Home"}</Link<Route>>
            </li>
            if user.is_some() {
               <>
                <li>
                  <Link<Route> to={Route::ProfilePage} classes="fluke-text-primary text-lg text-bold">{"Profile"}</Link<Route>>
                </li>
                <li
                  class="cursor-pointer"
                >
                  {"Create Post"}
                </li>
                <li class="cursor-pointer">
                  {"Logout"}
                </li>
              </>

            } else {
              <>
                <li>
                  <Link<Route> to={Route::RegisterPage} classes="fluke-text-primary text-lg text-bold">{"Sign-Up"}</Link<Route>>
                </li>
                <li>
                  <Link<Route> to={Route::LoginPage} classes="fluke-text-primary text-lg text-bold">{"Login"}</Link<Route>>
                </li>
              </>
            }
          </ul>
        </nav>
      </header>
    }
}