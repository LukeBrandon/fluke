use crate::router::Route;
use crate::store::Store;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

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
              <>
                <li>
                  <Link<Route> to={Route::ListPage} classes="">{"Login"}</Link<Route>>
                </li>
              </>
          </ul>
        </nav>
      </header>
    }
}
