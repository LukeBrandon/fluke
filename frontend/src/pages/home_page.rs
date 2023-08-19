use crate::components::header::Header;
use crate::pages::register_page::RegisterPage;
use yew::prelude::*;

pub struct HomePage;

impl Component for HomePage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header />
                <section class="section"> // account for fixed nav bar size
                    <RegisterPage />
                </section>
            </>
        }
    }
}
