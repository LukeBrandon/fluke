use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use lazy_static::lazy_static;
use regex;

use crate::api::user_api::api_register_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::{self, Route};
use crate::store::{set_page_loading, Store};

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors, ValidationError};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

lazy_static! {
    static ref RE_SPECIAL_CHAR: regex::Regex = regex::Regex::new("^.*?[@$!%*?&].*$").unwrap();
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut has_whitespace: bool = false;
    let mut has_upper: bool = false;
    let mut has_lower: bool = false;
    let mut has_digit: bool = false;

    for c in password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }
    if !has_whitespace && has_upper && has_lower && has_digit && password.len() >= 8 {
        Ok(())
    } else { 
        return Err(ValidationError::new("Password Validation Failed"));
    }
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct RegisterUserSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    first_name: String,
    last_name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    email: String,
    #[validate(
        custom(
            function = "validate_password",
            message = "Must Contain At Least One Upper Case, Lower Case and Number. Dont use spaces."
        ),
        regex(
            path = "RE_SPECIAL_CHAR",
            message = "Must Contain At Least One Special Character"
        ),
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    password: String,
    #[validate(
        length(min = 1, message = "Please confirm your password"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    password_confirm: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<RegisterUserSchema>,
) -> Callback<String> {
    Callback::from(move |value: String| {
        let mut data: RegisterUserSchema = cloned_form.deref().clone();
        match name {
            "first_name" => data.first_name = value,
            "last_name" => data.last_name = value,
            "email" => data.email = value,
            "password" => data.password = value,
            "password_confirm" => data.password_confirm = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form: UseStateHandle<RegisterUserSchema> = use_state(|| RegisterUserSchema::default());
    let validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator: Navigator = use_navigator().unwrap();

    let first_name_input_ref: NodeRef = NodeRef::default();
    let last_name_input_ref: NodeRef = NodeRef::default();
    let email_input_ref: NodeRef = NodeRef::default();
    let password_input_ref: NodeRef = NodeRef::default();
    let password_confirm_input_ref: NodeRef = NodeRef::default();

    let validate_input_on_blur: Callback<(String, String)> = {
        let cloned_form: UseStateHandle<RegisterUserSchema> = form.clone();
        let cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data: RegisterUserSchema = cloned_form.deref().clone();
            match name.as_str() {
                "first_name" => data.first_name = value,
                "last_name" => data.last_name = value, // Corrected the assignment here
                "email" => data.email = value,
                "password" => data.password = value,
                "password_confirm" => data.password_confirm = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name, error.clone());
                        }
                    }
                }
            }
        })
    };

    let handle_first_name_input: Callback<String> = get_input_callback("first_name", form.clone());
    let handle_last_name_input: Callback<String> = get_input_callback("last_name", form.clone());
    let handle_email_input: Callback<String> = get_input_callback("email", form.clone());
    let handle_password_input: Callback<String> = get_input_callback("password", form.clone());
    let handle_password_confirm_input: Callback<String> = get_input_callback("password_confirm", form.clone());

    let on_submit: Callback<SubmitEvent> = {
        let cloned_form: UseStateHandle<RegisterUserSchema> = form.clone();
        let cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> = validation_errors.clone();
        let cloned_navigator: Navigator = navigator.clone();
        let cloned_dispatch: Dispatch<Store> = dispatch.clone();

        let cloned_first_name_input_ref: NodeRef = first_name_input_ref.clone();
        let cloned_last_name_input_ref: NodeRef = last_name_input_ref.clone();
        let cloned_email_input_ref: NodeRef = email_input_ref.clone();
        let cloned_password_input_ref: NodeRef = password_input_ref.clone();
        let cloned_password_confirm_input_ref: NodeRef = password_confirm_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let form: UseStateHandle<RegisterUserSchema> = cloned_form.clone();
            let validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> = cloned_validation_errors.clone();
            let navigator: Navigator = cloned_navigator.clone();
            let dispatch: Dispatch<Store> = cloned_dispatch.clone();

            let first_name_input_ref: NodeRef = cloned_first_name_input_ref.clone();
            let last_name_input_ref: NodeRef = cloned_last_name_input_ref.clone();
            let email_input_ref: NodeRef = cloned_email_input_ref.clone();
            let password_input_ref: NodeRef = cloned_password_input_ref.clone();
            let password_confirm_input_ref: NodeRef = cloned_password_confirm_input_ref.clone();

            event.prevent_default();
            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data: RegisterUserSchema = form.deref().clone();
                        let form_json: String = serde_json::to_string(&form_data).unwrap();
                        set_page_loading(true, dispatch.clone());

                        let first_name_input: HtmlInputElement = first_name_input_ref.cast::<HtmlInputElement>().unwrap();
                        let last_name_input: HtmlInputElement = last_name_input_ref.cast::<HtmlInputElement>().unwrap();
                        let email_input: HtmlInputElement = email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input: HtmlInputElement = password_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_confirm_input: HtmlInputElement = password_confirm_input_ref
                            .cast::<HtmlInputElement>()
                            .unwrap();

                        first_name_input.set_value("");
                        last_name_input.set_value("");
                        email_input.set_value("");
                        password_input.set_value("");
                        password_confirm_input.set_value("");

                        let res: Result<crate::api::types::User, String> = api_register_user(&form_json).await;
                        match res {
                            Ok(_) => {
                                set_page_loading(false, dispatch.clone());

                                navigator.push(&router::Route::LoginPage);
                            }
                            Err(_) => {
                                set_page_loading(false, dispatch.clone());
                            }
                        };
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    html! {
        <section class="section">
            <div class="container">
                <h1 class="">
                    {" Welcome to Fluke!"}
                </h1>
                <h2 class="">
                    {"Sign Up To Get Started!"}
                </h2>
                <form
                    onsubmit={on_submit}
                    class=""
                >
                    <FormInput
                        label="First Name"
                        name="first_name"
                        input_ref={first_name_input_ref}
                        handle_onchange={handle_first_name_input}
                        errors={&*validation_errors}
                        handle_on_input_blur={validate_input_on_blur.clone()}
                    />
                    <FormInput
                        label="Last Name"
                        name="last_name"
                        input_ref={last_name_input_ref}
                        handle_onchange={handle_last_name_input}
                        errors={&*validation_errors}
                        handle_on_input_blur={validate_input_on_blur.clone()}
                    />
                    <FormInput
                        label="Email"
                        name="email"
                        input_type="email"
                        input_ref={email_input_ref}
                        handle_onchange={handle_email_input}
                        errors={&*validation_errors}
                        handle_on_input_blur={validate_input_on_blur.clone()}
                    />
                    <FormInput
                        label="Password"
                        name="password"
                        input_type="password"
                        input_ref={password_input_ref}
                        handle_onchange={handle_password_input}
                        errors={&*validation_errors}
                        handle_on_input_blur={validate_input_on_blur.clone()}
                    />
                    <FormInput
                        label="Confirm Password"
                        name="password_confirm"
                        input_type="password"
                        input_ref={password_confirm_input_ref}
                        handle_onchange={handle_password_confirm_input}
                        errors={&*validation_errors}
                        handle_on_input_blur={validate_input_on_blur.clone()}
                    />
                    <span class="block">
                        {"Already have an account?"} {" "}
                        <Link<Route> to={Route::LoginPage} classes="fluke-blue">{"Login Here"}</Link<Route>>
                    </span>
                    <LoadingButton
                        loading={store.page_loading}
                        text_color={Some("fluke-blue".to_string())}
                    >
                        {" Sign Up"}
                    </LoadingButton>
                </form>
            </div>
        </section>
    }
}