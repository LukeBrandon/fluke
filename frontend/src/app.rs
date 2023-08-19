use crate::components::spinner::Spinner;
use crate::router::{switch, Route};
use crate::store::Store;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

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
