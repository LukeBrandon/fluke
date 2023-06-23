use crate::{components::input::InputField};
use gloo_net::{http::Request, Error};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum SignupMsg {
    SubmitForm(web_sys::SubmitEvent),
    ReceiveResponse(Result<String, Error>),
}

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct UserFormSubmission {
    pub username: String,
    pub email: String,
    pub first_name: String, 
    pub last_name: String,
    pub password: String
}

#[derive(Clone, PartialEq, Properties, Debug, Default)]
pub struct SignupForm {
    pub username_ref: NodeRef,
    pub first_name_ref: NodeRef,
    pub last_name_ref: NodeRef,
    pub email_ref: NodeRef,
    pub password_ref: NodeRef,
    pub password_check_ref: NodeRef,
    pub password_is_valid: bool,
    pub messages: Vec<String>,
}


impl Component for SignupForm {
    type Message = SignupMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let s = Self {
            username_ref: NodeRef::default(),
            first_name_ref: NodeRef::default(),
            last_name_ref: NodeRef::default(),
            email_ref: NodeRef::default(),
            password_ref: NodeRef::default(), 
            password_check_ref: NodeRef::default(),
            password_is_valid: true,
            messages: Vec::new(),
        };
        log::debug!("Created: \n {:?}", s);
        s
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignupMsg::SubmitForm(_) => {
                // No way of getting around this cast to bind input data to form data
                let username = self.username_ref.cast::<HtmlInputElement>().unwrap().value();
                let first_name = self.first_name_ref.cast::<HtmlInputElement>().unwrap().value();
                let last_name = self.last_name_ref.cast::<HtmlInputElement>().unwrap().value();
                let email = self.email_ref.cast::<HtmlInputElement>().unwrap().value();
                let password = self.password_ref.cast::<HtmlInputElement>().unwrap().value();
                let password_check = self.password_check_ref.cast::<HtmlInputElement>().unwrap().value();
                let registration_form = UserFormSubmission {
                    username, first_name, last_name, email, password: password.clone()
                };
                if password.clone() != password_check {
                    self.password_is_valid = false;
                } else {
                    self.password_is_valid = true;
                };
                if self.password_is_valid {
                    let post_request = async move {
                        let response_result: Result<gloo_net::http::Response, gloo_net::Error> =
                            Request::post("http://127.0.0.1:8000/signup")
                                .header("Content-Type", "application/json")
                                .body(JsValue::from_str(
                                    &serde_json::to_string(&registration_form).unwrap(),
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
                    // Spawn the future into the local task queue to be run
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
                <h1 class="">{"Register"}</h1> 
                <form {onsubmit} class="registration-form">
                        <InputField input_node_ref={self.username_ref.clone()} name={"username".clone()} field_type={"text".clone()} placeholder={"Username".clone()} />
                        <InputField input_node_ref={self.email_ref.clone()} name={"email".clone()} field_type={"email".clone()}  placeholder={"Email".clone()} />
                        <InputField input_node_ref={self.first_name_ref.clone()} name={"first_name".clone()} field_type={"text".clone()} placeholder={"First name".clone()} />
                        <InputField input_node_ref={self.last_name_ref.clone()} name={"last_name".clone()} field_type={"text".clone()}  placeholder={"Last name".clone()} />
                        <InputField input_node_ref={self.password_ref.clone()} name={"password".clone()} field_type={"password".clone()}  placeholder={"Create Password".clone()}/>
                        <InputField input_node_ref={self.password_check_ref.clone()} name={"confirm_password".clone()} field_type={"password".clone()}  placeholder={"Retype password".clone()}/>
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
