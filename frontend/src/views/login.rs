use crate::{components::input::InputField};
use gloo_net::{http::Request};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum LoginMsg {
    SubmitForm(web_sys::SubmitEvent),
    ReceiveResponse(String),
}

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct UserFormSubmission {
    pub username: String,
    pub password: String
}

#[derive(Clone, PartialEq, Properties, Debug, Default)]
pub struct LoginForm {
    pub login_username_ref: NodeRef,
    pub login_password_ref: NodeRef,
    pub error_message: String,
}


impl Component for LoginForm {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let s = Self {
            login_username_ref: NodeRef::default(),
            login_password_ref: NodeRef::default(),
            error_message: String::default()
        };
        s
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::SubmitForm(_) => {
                let link: html::Scope<LoginForm> = ctx.link().clone();
                let username: String = self.login_username_ref.cast::<HtmlInputElement>().unwrap().value();
                let password: String = self.login_password_ref.cast::<HtmlInputElement>().unwrap().value();
                let login_form: UserFormSubmission = UserFormSubmission {
                    username, password
                };
                let post_request = async move {
                    let response_result: Result<gloo_net::http::Response, gloo_net::Error> =
                        Request::post("http://127.0.0.1:8000/login")
                            .header("Content-Type", "application/json")
                            .body(JsValue::from_str(
                                &serde_json::to_string(&login_form).unwrap(),
                            ))
                            .send()
                            .await;
                    match response_result {
                        Ok(response) => {
                           if response.ok() {
                                let response_text: String = response.text().await.unwrap();
                                log::info!("Response Text: {:?}", response_text);
                            } else {
                                link.send_message(LoginMsg::ReceiveResponse("Incorrect username or password.".to_string()));
                                log::warn!(
                                    "Request failed with status: {:?}",
                                    response.status()
                                );
                            }
                        }
                        Err(error) => {
                            log::warn!("Failed to make request: {:?}", error);
                        }
                    }
                };
                wasm_bindgen_futures::spawn_local(post_request);
                true
            }
            LoginMsg::ReceiveResponse(response) => {
                let message: String = response.clone();
                self.error_message = message;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit: Callback<web_sys::SubmitEvent> =
            ctx.link().callback(|e: web_sys::SubmitEvent| {
                e.prevent_default();
                LoginMsg::SubmitForm(e)
            });

        html! {
            <main class="home">
                <h1 class="">{"Sign In"}</h1>
               <form onsubmit={onsubmit} class="registration-form">
                    <InputField input_node_ref={self.login_username_ref.clone()} name={"username".clone()} field_type={"text".clone()} placeholder={"Username".clone()} />
                    <InputField input_node_ref={self.login_password_ref.clone()} name={"password".clone()} field_type={"password".clone()}  placeholder={"Password".clone()}/>
                    { if !self.error_message.is_empty() {
                           html!{<p class="error-text">{ &self.error_message }</p>}
                        } else {
                            html!{}
                    } }
                    <button type="submit" class="button button-primary form-button">{"Submit"}</button>
                </form>
            </main>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }
}
