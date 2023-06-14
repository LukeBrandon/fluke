use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

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

    log::info!("registration_form {:?}", &registration_form.clone());
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
            log::info!("registration_form {:?}", &registration_form.clone());

            let username = username.cast::<HtmlInputElement>().unwrap().value();
            let first_name = first_name_ref.cast::<HtmlInputElement>().unwrap().value();
            let last_name = last_name_ref.cast::<HtmlInputElement>().unwrap().value();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let confirm_password = confirm_password_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();


            // Fix: 
            // Check password against DB entry -----            
            if password != confirm_password {
                password_is_valid.set(false);
                return;
            } else {
                password_is_valid.set(true);
            };

            let registration_form = RegistrationForm {
                username,
                first_name,
                last_name,
                email,
                password,
                confirm_password,
            };

            log::info!("registration_form {:?}", &registration_form);

            // see https://github.com/rustwasm/wasm-bindgen/issues/1607#issuecomment-503499362
            wasm_bindgen_futures::spawn_local(async move {
                let post_request = Request::post("https://reqres.in/api/register")
                    .headers({
                        let headers = Headers::new();
                        headers
                            // .append(name, value)
                            .set("Content-Type", "application/json");
                        headers
                    })
                    .body(JsValue::from(
                        serde_json::to_string(&registration_form).unwrap(),
                    ))
                    .send()
                    .await
                    .unwrap();

                log::info!("post_request {:?}", &post_request);
            });
        })
    };
    html! {
        <main class="home">
            <h1>{"User Registration"}</h1>
            <form {onsubmit} class="registration-form">
                <InputField input_node_ref={username} name={"username".clone()} field_type={"text".clone()} placeholder={"Username".clone()} />
                <InputField input_node_ref={email_ref} name={"email".clone()} field_type={"email".clone()}  placeholder={"Email".clone()}/>
                <InputField input_node_ref={first_name_ref} name={"first_name".clone()} field_type={"text".clone()} placeholder={"First name".clone()}  />
                <InputField input_node_ref={last_name_ref}  name={"last_name".clone()} field_type={"text".clone()}  placeholder={"Last name".clone()}/>
                // feat: validate that it is a valid email address
                <InputField input_node_ref={password_ref} name={"password".clone()} field_type={"password".clone()}  placeholder={"Password".clone()}/>
                <InputField input_node_ref={confirm_password_ref} name={"confirm_password".clone()} field_type={"password".clone()}  placeholder={"Retype password".clone()}/>
                <p class="error-text">{ if *password_is_valid { "" } else { "Passwords do not match" } }</p>
                <button type="submit" class="button button-primary">{"Submit"}</button>
            </form>
        </main>
    }
}
