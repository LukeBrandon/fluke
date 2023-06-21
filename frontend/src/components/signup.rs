use crate::components::input::InputField;
use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::services::ConsoleService;

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct RegistrationForm {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub struct Home {
    fields: HashMap<String, NodeRef>,
    password_is_valid: StateHandle<bool>,
    link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let field_names = vec![
            "username",
            "first_name",
            "last_name",
            "email",
            "password",
            "confirm_password",
        ];

        let fields = field_names
            .into_iter()
            .map(|name| (name.to_string(), NodeRef::default()))
            .collect();

        Self {
            fields,
            password_is_valid: use_state(|| true),
            link,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(move |event: SubmitEvent| {
            event.prevent_default();
            // TODO: Add implementation for onsubmit callback
        });

        html! {
            <main class="home">
                <h1>{"User Registration"}</h1>
                <form {onsubmit} method="post" class="registration-form">
                    { for self.fields.iter().map(|(name, field)| self.create_input_field(field.clone(), name)) }
                    <p class="error-text">{ if *self.password_is_valid.get() { "".to_string() } else { "Passwords do not match".to_string() } }</p>
                    <button type="submit" class="button button-primary form-button">{"Submit"}</button>
                </form>
            </main>
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, _: bool) {}

    fn destroy(&mut self) {}
}

impl Home {
    fn create_input_field(&self, input_node_ref: NodeRef, name: &str) -> Html {
        let field_type = match name {
            "email" => "email",
            "password" | "confirm_password" => "password",
            _ => "text",
        };
        let placeholder = name
            .split('_')
            .map(|s| [&str::to_uppercase(s), " "].concat())
            .collect::<String>()
            .trim()
            .to_string();
        html! {
            <InputField
                input_node_ref={input_node_ref}
                name={name.to_string()}
                field_type={field_type.to_string()}
                placeholder={placeholder}
            />
        }
    }
}
