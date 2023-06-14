use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
// use serde_json;
// use wasm_bindgen::JsValue;
// use web_sys::HtmlInputElement;

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
    // let onsubmit = { !todo(); };
    html! {
        <main class="home">
            <h1>{"User Registration"}</h1>
            // <form {onsubmit} class="registration-form"> # to do
            <form class="registration-form">
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