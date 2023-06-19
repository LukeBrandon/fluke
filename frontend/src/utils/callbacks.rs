// src/utils/callbacks.rs

use crate::components::input::InputField;
use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::JsValue;
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

pub fn onsubmit_callback(
    username: NodeRef,
    first_name_ref: NodeRef,
    last_name_ref: NodeRef,
    email_ref: NodeRef,
    password_ref: NodeRef,
    confirm_password_ref: NodeRef,
    password_is_valid: StateHandle<bool>,
) -> Callback<SubmitEvent> {
    Callback::from(move |event: SubmitEvent| {
        // existing logic from your onsubmit callback here
    })
}
