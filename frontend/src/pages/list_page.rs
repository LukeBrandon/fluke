use crate::api::{message_api::api_get_messages, types::Message};
use crate::components::channels_list::ChannelsList;
use crate::composables::messages;
use crate::store::{set_page_loading, Store};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct ListPage {
    link: dyn Component,
    store: Store,
    dispatch: Dispatch<Store>,
    messages: Rc<Vec<Message>>,
    input_value: String,
    current_user_id: i64,
    current_channel_id: i64,
}

enum Msg {
    FetchedMessages(Result<Vec<Message>, String>),
}

impl Component for ListPage {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (store, dispatch) = use_store::<Store>();
        Self {
            link,
            store,
            dispatch,
            messages: Rc::new(Vec::new()),
            input_value: String::default(),
            current_user_id: 2,
            current_channel_id: 2,
        }
    }

    fn update(&mut self, msg: Self::Message, ctx: &Context<Self>) -> bool {
        match msg {
            Msg::FetchedMessages(Ok(response_messages)) => {
                self.messages = Rc::new(response_messages);
                true
            }
            Msg::FetchedMessages(Err(err_msg)) => {
                log::error!("{}", err_msg);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = messages::on_change(ctx.link().callback(|e: InputData| e.value));
        let fetch_messages: Callback<MouseEvent> = {
            let link = ctx.link().clone();
            Callback::from(move |_| {
                spawn_local(async move {
                    match api_get_messages(2).await {
                        Ok(response_messages) => {
                            link.send_message(Msg::FetchedMessages(Ok(response_messages)));
                        }
                        Err(_) => {
                            log::warn!("Failed to fetch messages");
                            link.send_message(Msg::FetchedMessages(Err(String::from(
                                "Failed to fetch messages",
                            ))));
                        }
                    }
                });
            })
        };

        let on_send_and_fetch: Callback<MouseEvent> = {
            let current_channel_id = self.current_channel_id;
            let current_user_id = self.current_user_id;
            let input_value = self.input_value.clone();
            let fetch_messages = fetch_messages.clone();

            Callback::from(move |_| {
                spawn_local(async move {
                    messages::on_send_async(current_channel_id, current_user_id, input_value).await;
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
                        { for self.messages.iter().map(|msg| html! {
                             <tr>
                                <td>{ &msg.id }</td>
                                <td>{ &msg.message }</td>
                             </tr>
                        }) }
                    </tbody>
            </table>
        };

        html! {
            <div class="container">
                <section class="section">
                    <ChannelsList/>
                </section>
                { if self.store.page_loading {
                    html! { <div class="loading"> { "Loading messages..." } </div> }
                } else {
                    html! {
                        <>
                            { message_list }
                            <label for="input">
                                { "Enter your message:" }
                                <input onchange={on_change}
                                    id="input"
                                    type="text"
                                    value={self.input_value.clone()}
                                />
                            </label>
                            <button onclick={on_send_and_fetch}>{ "Send" }</button>
                        </>
                    }
                }}
            </div>
        }
    }
}
