use crate::components::input::InputField;
use gloo_net::{http::Request, Error};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use yew::prelude::*;

pub enum LoginMsg {
    SubmitForm(web_sys::SubmitEvent),
    ReceiveResponse(Result<String, Error>),
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct RegistrationForm {
    pub email: String,
    pub password: String,
}

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
    messages: Vec<String>,
}

impl Component for LoginForm {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            messages: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let login_form_data = self.clone();
        match msg {
            LoginMsg::SubmitForm(_) => {
                    let post_request = async move {
                        let response_result: Result<gloo_net::http::Response, gloo_net::Error> =
                            Request::post("http://127.0.0.1:8000/login")
                                .header("Content-Type", "application/json")
                                .body(JsValue::from_str(
                                    &serde_json::to_string(&login_form_data).unwrap(),
                                ))
                                .send()
                                .await;
                        match response_result {
                            Ok(response) => {
                                log::info!("Response: {:?}", response);
                                if response.ok() {
                                    let response_text = response.text().await.unwrap();
                                    log::info!("Response Text: {:?}", response_text);
                                } else if response.status() == 409 {
                                    log::warn!(
                                        "Login failed due to a duplicate username or email"
                                    );
                                } else {
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

                    // ctx.link().send_message(LoginMsg::ReceiveResponse);
                true
            }
            LoginMsg::ReceiveResponse(response) => {
                match response {
                    Ok(data) => {
                        self.messages.push(data);
                    }
                    Err(error) => {
                        self.messages.push(error.to_string());
                    }
                }
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
                <h1 class="">{"User Registration"}</h1>
               <form onsubmit={onsubmit} class="registration-form">
                    <InputField name={"email".clone()} field_type={"email".clone()}  placeholder={"Email".clone()}/>
                    <InputField name={"password".clone()} field_type={"password".clone()}  placeholder={"Create Password".clone()}/>
                    <button type="submit" class="button button-primary form-button">{"Submit"}</button>
                </form>
            </main>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }
}
