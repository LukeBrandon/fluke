use crate::components::{
    spinner::Spinner,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::store::Store;
use crate::router::{switch, Route};

 
#[function_component(App)]
pub fn app() -> Html {
    let (store, _) = use_store::<Store>();
    let is_page_loading = store.page_loading.clone();

    html! {
        <BrowserRouter>
                <Switch<Route> render={switch} />
                if is_page_loading {
                    <div class="pt-4 pl-2 top-[5.5rem] fixed">
                        <Spinner width={Some("1.5rem")} height={Some("1.5rem")} color="text-ct-yellow-600" />
                    </div>
                }
        </BrowserRouter>
    }
}
