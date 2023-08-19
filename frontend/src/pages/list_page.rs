use crate::components::channels_list::ChannelsList;
use crate::components::message_panel::MessagePanel;
use crate::components::messages_list::MessagesList;

use yew::prelude::*;
pub struct ListPage;

impl Component for ListPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <section class="section">
                <ChannelsList/>
                </section>
                <section class="section">
                <MessagesList/>
                </section>
                <MessagePanel/>
            </div>
        }
    }
}
