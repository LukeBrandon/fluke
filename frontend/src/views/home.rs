use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use web_sys::{HtmlInputElement, Request, RequestInit, RequestMode, ReadableStream};
use yew::prelude::*;
use crate::components::input::InputField;

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

            let inner_function = || -> Result<(), wasm_bindgen::JsValue> {
                let request_res: Result<Request, wasm_bindgen::JsValue> =
                    Request::new_with_str_and_init(&url, &options);
                let request = match request_res {
                    Ok(val) => val,
                    Err(err) => {
                        println!("Error creating request: {:?}", err.as_string());
                        log::info!("Error creating request: {:?}", err.as_string());
                        return Err(err);
                    }
                };

                request.headers().set("Accept", "application/json")?;
                request.headers().set("Content-Type", "application/json")?;

                // May need some special error handling depending on how the form is submitted here ----
                let body = serde_json::to_string(&registration_form).unwrap();

                request.body().insert(body.into_bytes());

                Request::new(&request)
                    .and_then(|response| response.text())
                    .then(|result| {
                        match result {
                            Ok(text) => {
                                log::info!("Data sent to the server successfully");
                                // Handle success response here
                                Ok(())
                            }
                            Err(error) => {
                                log::info!("Error sending data to the server: {:?}", error);
                                // Handle error response here
                                Err(error)
                            }
                        }
                    })
            };

            // Call the inner function and handle any errors
            if let Err(err) = inner_function() {
                // Handle the error case here
                // ...
            }
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
