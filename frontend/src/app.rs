use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/logo_transparent.png" class="logo fluke" alt="Fluke Logo"/>
                </a>
            </div>

            <div><p>{"Sign in"} </p></div>

            <form class="row">
                <input id="greet-input" placeholder="Enter your email..." />
            </form>
            <div></div>
            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter your password..." />
                <button type="submit">{"Greet"}</button>
            </form>

            <p><b>{ &*greet_msg }</b></p>
        </main>
    }
}
