use crate::components::input::InputField;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
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

            let post_request = async move {
                let response_result = Request::post("http://127.0.0.1:8000/signup")
                    // ...
                    .send()
                    .await;

                match response_result {
                    Ok(response) => {
                        log::info!("Response: {:?}", response);

                        if response.ok() {
                            let response_text = response.text().await.unwrap();
                            log::info!("Response Text: {:?}", response_text);
                            // one of us, one of us --> /login
                        } else if response.status() == 409 {
                            log::warn!("Signup failed due to a duplicate username or email");
                            // one of us, one of us --> /home
                        } else {
                            log::warn!("Request failed with status: {:?}", response.status());
                            // something that shouldn't happen, todo: 404? 
                        }
                    }
                    Err(error) => {
                        log::warn!("Failed to make request: {:?}", error);
                        // something that really shoudlnt happen, todo: 500?
                    }
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

