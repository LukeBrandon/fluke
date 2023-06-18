use crate::components::input::InputField;
use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlInputElement, RequestInit, RequestMode};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct RegistrationForm {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let registration_form = use_state(|| RegistrationForm::default());

    let username = use_node_ref();
    let first_name_ref = use_node_ref();
    let last_name_ref = use_node_ref();
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();
    let confirm_password_ref = use_node_ref();

    let password_is_valid = use_state(|| true);
    let onsubmit = {
        let registration_form = registration_form.clone();
        let username = username.clone();
        let first_name_ref = first_name_ref.clone();
        let last_name_ref = last_name_ref.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let confirm_password_ref = confirm_password_ref.clone();
        let password_is_valid = password_is_valid.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let username_value = username.cast::<HtmlInputElement>().unwrap().value();
            let first_name_value = first_name_ref.cast::<HtmlInputElement>().unwrap().value();
            let last_name_value = last_name_ref.cast::<HtmlInputElement>().unwrap().value();
            let email_value = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password_value = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let confirm_password_value = confirm_password_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            if password_value != confirm_password_value {
                password_is_valid.set(false);
                return;
            } else {
                password_is_valid.set(true);
            }

            let registration_form = RegistrationForm {
                username: username_value,
                first_name: first_name_value,
                last_name: last_name_value,
                email: email_value,
                password: password_value,
                confirm_password: confirm_password_value,
            };

            log::info!("registration_form {:?}", &registration_form);

            let post_request = async move {
                let response = Request::post("http://127.0.0.1:8000/signup")
                    .headers({
                        let headers = Headers::new();
                        headers.set("Content-Type", "application/json");
                        headers
                    })
                    .body(JsValue::from_str(
                        &serde_json::to_string(&registration_form)
                            .expect("Failed to serialize data"),
                    ))
                    .send()
                    .await
                    .unwrap();
                log::info!("Response: {:?}", response);
                if response.ok() {
                    let response_text = response.text().await.unwrap();
                    log::info!("Response Text: {:?}", response_text);

                    let response_json: serde_json::Value =
                        serde_json::from_str(&response_text).unwrap();
                    log::info!("Response JSON: {:?}", response_json);

                    if let Some(status) = response_json.get("status") {
                        match status {
                            serde_json::Value::String(status) if status == "success" => {
                                log::info!("Registration was successful");
                            }
                            _ => {
                                log::warn!("Registration failed");
                            }
                        }
                    }
                } else {
                    log::warn!("Request failed with status: {:?}", response.status());
                }
            };

            wasm_bindgen_futures::spawn_local(post_request);
        })
    };
    html! {
        <main class="home">
            <h1>{"User Registration"}</h1>
            <form {onsubmit} method="post" class="registration-form">
                <InputField input_node_ref={username} name={"username".clone()} field_type={"text".clone()} placeholder={"Username".clone()} />
                <InputField input_node_ref={email_ref} name={"email".clone()} field_type={"email".clone()}  placeholder={"Email".clone()}/>
                <InputField input_node_ref={first_name_ref} name={"first_name".clone()} field_type={"text".clone()} placeholder={"First name".clone()}  />
                <InputField input_node_ref={last_name_ref}  name={"last_name".clone()} field_type={"text".clone()}  placeholder={"Last name".clone()}/>
                <InputField input_node_ref={password_ref} name={"password".clone()} field_type={"password".clone()}  placeholder={"Password".clone()}/>
                <InputField input_node_ref={confirm_password_ref} name={"confirm_password".clone()} field_type={"password".clone()}  placeholder={"Retype password".clone()}/>
                <p class="error-text">{ if *password_is_valid { "" } else { "Passwords do not match" } }</p>
                <button type="submit" class="button button-primary form-button">{"Submit"}</button>
            </form>
        </main>
    }
}
