use crate::api::message_api::api_get_messages;
use crate::api::types::Message;
use crate::store::{set_page_loading, Store};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(MessagesPage)]
pub fn messages_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let messages: UseStateHandle<Rc<Vec<Message>>> = use_state(|| Rc::new(Vec::new()));

    let fetch_messages: Callback<MouseEvent> = {
        let messages = messages.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            let messages = messages.clone();
            let dispatch = dispatch.clone();
            spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                match api_get_messages().await {
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
    };

    if store.page_loading {
        return html! {
            <div class="loading"> { "Loading messages..." } </div>
        };
    }

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

    html! {
        <section class="message-container">
            <div class="container">
                <h1> { "Messages" } </h1>
                { message_list }
                <button onclick={fetch_messages.clone()}> { "Refresh" } </button>
            </div>
        </section>
    }
}
