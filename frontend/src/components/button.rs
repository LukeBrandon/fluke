use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

pub struct Button;

impl Component for Button {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
               <Link<Route> to={Route::SignupForm}>{ "click here to go home" }</Link<Route>>
            </div>
        }
    }
}
