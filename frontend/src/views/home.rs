use crate::components::nav::Nav;
use crate::views::about::About;
use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Nav />
                <div class="flex flex-col pt-[72px]"> // account for fixed nav bar size
                    <About />
                </div>
            </>
        }
    }
}
