use crate::components::input::InputField;
use gloo_net::{http::Request, Error};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use yew::prelude::*;

pub enum SignupMsg {
    SubmitForm(web_sys::SubmitEvent),
    ReceiveResponse(Result<String, Error>),
}

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct SignupForm {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub password_is_valid: bool,
    pub messages: Vec<String>,
}

impl Component for SignupForm {
    type Message = SignupMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let s = Self {
            username: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            password: String::new(),
            password_is_valid: true,
            messages: Vec::new(),
        };
        log::debug!("Created: \n {:?}", s);
        s
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let signup_form_data = self.clone();
        match msg {
            SignupMsg::SubmitForm(e) => {
                log::debug!("{:?}", e);
                if self.password_is_valid {
                    let post_request = async move {
                        let response_result: Result<gloo_net::http::Response, gloo_net::Error> =
                            Request::post("http://127.0.0.1:8000/signup")
                                .header("Content-Type", "application/json")
                                .body(JsValue::from_str(
                                    &serde_json::to_string(&signup_form_data).unwrap(),
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
                                        "Registration failed due to a duplicate username or email"
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
                }

                log::debug!("fn update: SubmitForm: \n {:?}", self);
                true
            }
            SignupMsg::ReceiveResponse(response) => {
                match response {
                    Ok(data) => {
                        self.messages.push(data);
                    }
                    Err(error) => {
                        self.messages.push(error.to_string());
                    }
                }
                log::debug!("fn update: RecieveResponses: \n {:?}", self);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit: Callback<web_sys::SubmitEvent> =
            ctx.link().callback(|e: web_sys::SubmitEvent| {
                e.prevent_default();
                SignupMsg::SubmitForm(e)
            });

        html! {
            <main class="home">
                <h1 class="">{"User Registration"}</h1>
               <form onsubmit={onsubmit} class="registration-form">
                    <InputField name={"username".clone()} field_type={"text".clone()} placeholder={"Username".clone()} />
                    <InputField name={"email".clone()} field_type={"email".clone()}  placeholder={"Email".clone()} />
                    <InputField name={"first_name".clone()} field_type={"text".clone()} placeholder={"First name".clone()} />
                    <InputField name={"last_name".clone()} field_type={"text".clone()}  placeholder={"Last name".clone()} />
                    <InputField name={"password".clone()} field_type={"password".clone()}  placeholder={"Create Password".clone()}/>
                    <InputField name={"confirm_password".clone()} field_type={"password".clone()}  placeholder={"Retype password".clone()}/>
                    <p class="error-text">{ if self.password_is_valid { "" } else { "Passwords do not match" } }</p>
                    <button type="submit" class="button button-primary form-button">{"Submit"}</button>
                </form>
            </main>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }
}
