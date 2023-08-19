use crate::api::message_api::api_get_channels;
use crate::api::types::Channel;
use crate::store::{set_page_loading, Store};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(ChannelsList)]
pub fn channels_dropdown() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let channels: UseStateHandle<Rc<Vec<Channel>>> = use_state(|| Rc::new(Vec::new()));

    let fetch_channels: Callback<MouseEvent> = {
        let channels = channels.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            let channels = channels.clone();
            let dispatch = dispatch.clone();
            spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                match api_get_channels().await {
                    Ok(response_channels) => {
                        channels.set(Rc::new(response_channels));
                        set_page_loading(false, dispatch.clone());
                    }
                    Err(_) => {
                        set_page_loading(false, dispatch.clone());
                        log::warn!("Failed to fetch channels");
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

    let channels_list: Html = html! {
        <table class="message_table">
            <thead>
            <tr>
            <th>{ "ID" }</th>
            <th>{ "Name" }</th>
            </tr>
            </thead>
                <tbody>
                    { for channels.iter().map(|chan| html! {
                         <tr>
                            <td>{ &chan.id }</td>
                            <td>{ &chan.name }</td>
                         </tr>
                    }) }
                </tbody>
        </table>
    };

    html! {
        <section>
            <div class="container">
            <h1> { "Channels" } </h1>
            { channels_list }
            <button onclick={fetch_channels.clone()}> {"Refresh"} </button>
            </div>
        </section>
    }
}
