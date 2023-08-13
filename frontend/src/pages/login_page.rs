use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::user_api::api_login_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::{self, Route};
use crate::store::{set_page_loading, set_user_auth_error, Store};

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]

struct LoginUserSchema {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginUserSchema>,
) -> Callback<String> {
    Callback::from(move |value: String| {
        let mut data: LoginUserSchema = cloned_form.deref().clone();
        match name {
            "email" => data.email = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form: UseStateHandle<LoginUserSchema> = use_state(|| LoginUserSchema::default());
    let validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> =
        use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let navigator: Navigator = use_navigator().unwrap();

    let email_input_ref: NodeRef = NodeRef::default();
    let password_input_ref: NodeRef = NodeRef::default();

    let validate_input_on_blur: Callback<(String, String)> = {
        let cloned_form: UseStateHandle<LoginUserSchema> = form.clone();
        let cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> =
            validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data: LoginUserSchema = cloned_form.deref().clone();
            match name.as_str() {
                "email" => data.email = value,
                "password" => data.password = value,
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

    let handle_email_input: Callback<String> = get_input_callback("email", form.clone());
    let handle_password_input: Callback<String> = get_input_callback("password", form.clone());

    let on_submit: Callback<SubmitEvent> = {
        let cloned_form: UseStateHandle<LoginUserSchema> = form.clone();
        let cloned_validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> =
            validation_errors.clone();
        let store_dispatch: Dispatch<Store> = dispatch.clone();
        let cloned_navigator: Navigator = navigator.clone();

        let cloned_email_input_ref: NodeRef = email_input_ref.clone();
        let cloned_password_input_ref: NodeRef = password_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let dispatch: Dispatch<Store> = store_dispatch.clone();
            let form: UseStateHandle<LoginUserSchema> = cloned_form.clone();
            let validation_errors: UseStateHandle<Rc<RefCell<ValidationErrors>>> =
                cloned_validation_errors.clone();
            let navigator: Navigator = cloned_navigator.clone();

            let email_input_ref: NodeRef = cloned_email_input_ref.clone();
            let password_input_ref: NodeRef = cloned_password_input_ref.clone();

            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data: LoginUserSchema = form.deref().clone();
                        set_page_loading(true, dispatch.clone());

                        let email_input: HtmlInputElement =
                            email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input: HtmlInputElement =
                            password_input_ref.cast::<HtmlInputElement>().unwrap();

                        email_input.set_value("");
                        password_input.set_value("");

                        let form_json: String = serde_json::to_string(&form_data).unwrap();
                        let res: Result<crate::api::types::UserLoginResponse, String> =
                            api_login_user(&form_json).await;
                        match res {
                            Ok(_) => {
                                set_page_loading(false, dispatch);
                                // set_auth_user(.unwrap(), dispatch);
                                navigator.push(&router::Route::ProfilePage);
                                log::info!("Successful login: {:?}", &form_json)
                            }
                            Err(e) => {
                                log::warn!("User login error: {}", e);
                                set_page_loading(false, dispatch.clone());
                                set_user_auth_error(
                                    "Invalid username or password".to_string(),
                                    dispatch,
                                );
                                log::warn!("User login error with form: {:?}", &form_json)
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
          {"Welcome Back"}
        </h1>
        <h2 class="">
          {"Login to have access"}
        </h2>
        {if !store.user_auth_error.is_empty() {
            html! {
                <div class="error-text">{store.user_auth_error.clone()}</div>
            }
        } else {
            html! {}
        }}
          <form
            onsubmit={on_submit}
            class=""
          >
            <FormInput label="Email" name="email" input_type="email" input_ref={email_input_ref} handle_onchange={handle_email_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}/>

            <div class="">
              <a href="#">
                {"Forgot Password?"}
              </a>
            </div>
            <LoadingButton
              loading={store.page_loading}
              text_color={Some("fluke-blue".to_string())}
            >
              {"Login"}
            </LoadingButton>
            <span class="block">
              {"Need an account?"} {" "}
              <Link<Route> to={Route::RegisterPage} classes="fluke-blue">{ "Sign Up Here" }</Link<Route>>
            </span>
          </form>
      </div>
    </section>
    }
}
