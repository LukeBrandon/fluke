use crate::components::header::Header;
use crate::pages::about_page::AboutPage;
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
                <section class="flex flex-col pt-[72px]"> // account for fixed nav bar size
                    <AboutPage />
                </section>
            </>
        }
    }
}
