use crate::api::message_api::{api_create_message, api_get_messages};
use crate::api::types::Message;
use crate::store;
use crate::store::set_page_loading;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

pub fn on_change(input_value_handle: UseStateHandle<String>) -> Callback<Event> {
    let input_value_handle = input_value_handle.clone();

    Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            input_value_handle.set(input.value());
        }
    })
}

pub fn fetch_messages(
    messages: UseStateHandle<Rc<Vec<Message>>>,
    dispatch: Dispatch<store::Store>,
) -> Callback<MouseEvent> {
    let messages = messages.clone();
    let dispatch = dispatch.clone();

    Callback::from(move |_| {
        let messages = messages.clone();
        let dispatch = dispatch.clone();
        spawn_local(async move {
            set_page_loading(true, dispatch.clone());
            match api_get_messages(2).await {
                Ok(response_messages) => {
                    messages.set(Rc::new(response_messages));
                    set_page_loading(false, dispatch.clone());
                }
                Err(_) => {
                    set_page_loading(false, dispatch.clone());
                    log::warn!("Failed to fetch messages");
                }
            }
        });
    })
}

pub async fn on_send_async(
    current_channel_id: i64,
    current_user_id: i64,
    input_value_handle: UseStateHandle<String>,
) {
    let iv_handle = input_value_handle.clone();
    let msg = (*input_value_handle).clone();
    if let Err(err) = api_create_message(&current_channel_id, &msg, current_user_id).await {
        log::error!("Failed to send message: {}", err);
    }
    // Reset the input field to ""
    iv_handle.set(String::default());
}
