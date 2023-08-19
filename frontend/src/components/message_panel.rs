use crate::api::types::Message;
use crate::composables::messages;
use crate::store::Store;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(MessagePanel)]
pub fn message_panel() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let messages: UseStateHandle<Rc<Vec<Message>>> = use_state(|| Rc::new(Vec::new()));

    let current_user_id: i64 = 2;
    let current_channel_id: i64 = 2;
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_change = messages::on_change(input_value_handle.clone());

    let fetch_messages = messages::fetch_messages(messages.clone(), dispatch.clone());

    let on_send_and_fetch = {
        let current_channel_id = current_channel_id;
        let current_user_id = current_user_id;
        let input_value_handle = input_value_handle.clone();
        let fetch_messages = fetch_messages.clone();

        Callback::from(move |_: MouseEvent| {
            let current_channel_id = current_channel_id;
            let current_user_id = current_user_id;
            let input_value_handle = input_value_handle.clone();
            let fetch_messages = fetch_messages.clone();

            spawn_local(async move {
                messages::on_send_async(current_channel_id, current_user_id, input_value_handle)
                    .await;
                if let Ok(event) = MouseEvent::new("click") {
                    fetch_messages.emit(event);
                } else {
                    log::error!("Failed to create MouseEvent");
                }
            });
        })
    };

    let message_list: Html = html! {
        <table class="message_table">
            <thead>
            <tr>
            <th>{ "ID" }</th>
            <th>{ "Message" }</th>
            </tr>
            </thead>
                <tbody>
                    { for messages.iter().map(|msg| html! {
                         <tr>
                            <td>{ &msg.id }</td>
                            <td>{ &msg.message }</td>
                         </tr>
                    }) }
                </tbody>
        </table>
    };

    if store.page_loading {
        return html! {
            <div class="loading"> { "Loading messages..." } </div>
        };
    }

    html! {
        <>
            { message_list }
            <label for="input">
                { "Enter your message:" }
                <input onchange={on_change}
                    id="input"
                    type="text"
                    value={input_value.clone()}
                />
            </label>
            <button onclick={on_send_and_fetch}>{ "Send" }</button>
        </>
    }
}
