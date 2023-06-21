use crate::components::input::InputField;
use gloo_net::{http::Request, Error};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct RegistrationForm {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub enum Msg {
    SubmitForm(web_sys::Event),
    ReceiveResponse(Result<String, Error>),
}

pub struct SignupForm {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    password_is_valid: bool,
    error_message: Option<String>,
}

impl Component for SignupForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            username: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            password: String::new(),
            password_is_valid: true,
            error_message: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm(event) => {
                event.prevent_default();

                if self.password_is_valid {
                    let registration_form = RegistrationForm {
                        username: self.username.clone(),
                        first_name: self.first_name.clone(),
                        last_name: self.last_name.clone(),
                        email: self.email.clone(),
                        password: self.password.clone(),
                    };

                    let request = Request::post("http://127.0.0.1:8000/signup")
                        .header("Content-Type", "application/json")
                        .body(JsValue::from_str(
                            &serde_json::to_string(&registration_form).unwrap(),
                        ));

                    let future = async {
                        let resp = request.send().await.unwrap();
                        assert_eq!(resp.status(), 200);
                        resp.text().await.unwrap()
                    };

                    ctx.link().send_future(future);
                    ctx.link().send_message(Msg::ReceiveResponse);
                }
                true
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(data) => {
                        self.error_message = Some(data);
                    }
                    Err(error) => {
                        self.error_message = Some(error.to_string());
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class="home">
                <h1 class="text-lg py-5">{"User Registration"}</h1>
                <form onsubmit={ctx.link().callback(|e: FormSubmit| Msg::SubmitForm(e))} class="registration-form">
                    <InputField input_node_ref={username} name={"username".clone()} field_type={"text".clone()} placeholder={"Username".clone()} />
                    <InputField input_node_ref={email_ref} name={"email".clone()} field_type={"email".clone()}  placeholder={"Email".clone()}/>
                    <InputField input_node_ref={first_name_ref} name={"first_name".clone()} field_type={"text".clone()} placeholder={"First name".clone()}  />
                    <InputField input_node_ref={last_name_ref}  name={"last_name".clone()} field_type={"text".clone()}  placeholder={"Last name".clone()}/>
                    <InputField input_node_ref={password_ref} name={"password".clone()} field_type={"password".clone()}  placeholder={"Password".clone()}/>
                    <InputField input_node_ref={confirm_password_ref} name={"confirm_password".clone()} field_type={"password".clone()}  placeholder={"Retype password".clone()}/>
                    <p class="error-text">{ if self.password_is_valid { "" } else { "Passwords do not match" } }</p>
                    <p class="error-text">{self.error_message.as_ref().unwrap_or(&"".to_string())}</p>
                    <button type="submit" class="button button-primary form-button">{"Submit"}</button>
                </form>
            </main>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }
}
