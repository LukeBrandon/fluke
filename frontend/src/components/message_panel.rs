use crate::api::message_api::api_create_message;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component(MessagePanel)]
pub fn message_panel() -> Html {
    let current_user_id: i64 = 2;
    let current_channel_id: i64 = 2;
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let on_send = {
        let input_value_handle = input_value_handle.clone();
        let input_value = input_value.clone();
        Callback::from(move |_e: MouseEvent| {
            let iv_handle = input_value_handle.clone();
            let msg = input_value.clone();
            spawn_local(async move {
                if let Err(err) =
                    api_create_message(&current_channel_id, &msg, current_user_id).await
                {
                    // Handle the error in the way you prefer. For example:
                    log::error!("Failed to send message: {}", err);
                }

                // Reset the input field to ""
                iv_handle.set(String::default());
            });
        })
    };

    html! {
        <>
            <label for="input">
                { "Enter your message:" }
                <input onchange={on_change}
                    id="input"
                    type="text"
                    value={input_value.clone()}
                />
            </label>
            <button onclick={on_send}>{ "Send" }</button>  // This is the new "send" button
        </>
    }
}
