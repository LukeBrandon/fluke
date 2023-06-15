use crate::components::input::InputField;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{HtmlInputElement, ReadableStream, Request, RequestInit, RequestMode};
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

            // Send the registration form data to the backend server using gloo
            // Refrerence: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/fetch/src/lib.rs
            let mut options = RequestInit::new();

            options.method("POST");
            options.mode(RequestMode::Cors);
            let db_env_url: &str = "ROCKET_DEV_URL";
            let url: String = match env::var(db_env_url) {
                Ok(val) => {
                    println!("URL: {:?}", val);
                    val
                }
                Err(e) => {
                    println!("Couldn't interpret {}: {}", db_env_url, e);
                    return;
                }
            };

            let registration_form_clone = registration_form.clone();

            let inner_function = async move {
                let mut options = web_sys::RequestInit::new();
                options.method("POST");
                options.mode(web_sys::RequestMode::Cors);
                let body = serde_json::to_string(&registration_form).unwrap();
                let body = JsValue::from_str(&body);
                options.body(Some(&body));
                let request = web_sys::Request::new_with_str_and_init(&url, &options).unwrap();
                request.headers().set("Accept", "application/json").unwrap();
                request
                    .headers()
                    .set("Content-Type", "application/json")
                    .unwrap();

                let window = web_sys::window().unwrap();
                let request_promise = window.fetch_with_request(&request);
                let resp_value = wasm_bindgen_futures::JsFuture::from(request_promise)
                    .await
                    .unwrap();

                let resp: web_sys::Response = resp_value.dyn_into().unwrap();
                let json = wasm_bindgen_futures::JsFuture::from(resp.json().unwrap())
                    .await
                    .unwrap();
               let parsed_json: serde_json::Value = serde_wasm_bindgen::from_value(json).unwrap();

                match parsed_json.get("status") {
                    Some(serde_json::Value::String(status)) if status == "success" => {
                        log::info!("Registration was successful");
                    }
                    _ => {
                        log::warn!("Registration failed");
                    }
                }
                Ok::<(), wasm_bindgen::JsValue>(())
            };

            wasm_bindgen_futures::spawn_local(async {
                if let Err(err) = inner_function.await {
                    // Handle the error case here
                    log::warn!("Error occurred: {:?}", err);
                }
            });
        })
    };
    html! {
        <main class="home">
            <h1>{"User Registration"}</h1>
            <form {onsubmit} class="registration-form">
                <InputField input_node_ref={username} label={"Username".to_owned()} name={"username".clone()} field_type={"text".clone()} />
                <InputField input_node_ref={first_name_ref} label={"First Name".to_owned()} name={"first_name".clone()} field_type={"text".clone()}  />
                <InputField input_node_ref={last_name_ref} label={"Last Name".to_owned()} name={"last_name".clone()} field_type={"text".clone()}  />
                <InputField input_node_ref={email_ref} label={"Email".to_owned()} name={"email".clone()} field_type={"email".clone()}  />
                <InputField input_node_ref={password_ref} label={"Password".to_owned()} name={"password".clone()} field_type={"password".clone()}  />
                <InputField input_node_ref={confirm_password_ref} label={"Confirm Password".to_owned()} name={"confirm_password".clone()} field_type={"password".clone()}  />
                <p class="error-text">{ if *password_is_valid { "" } else { "Passwords do not match" } }</p>
                <button type="submit" class="button button-primary">{"Submit"}</button>
            </form>
        </main>
    }
}
