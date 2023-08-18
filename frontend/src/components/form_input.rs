use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_type: Option<String>,
    pub label: String,
    pub name: String,
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>,
}

#[function_component(FormInput)]
pub fn form_input_component(props: &Props) -> Html {
    let input_type: String = props
        .input_type
        .clone()
        .unwrap_or_else(|| "text".to_string());
    let val_errors: std::cell::Ref<'_, ValidationErrors> = props.errors.borrow();
    let errors: std::collections::HashMap<&str, &Vec<validator::ValidationError>> =
        val_errors.field_errors().clone();
    let empty_errors: Vec<validator::ValidationError> = vec![];
    let error: &Vec<validator::ValidationError> = match errors.get(&props.name.as_str()) {
        Some(error) => error,
        None => &empty_errors,
    };
    let error_message: String = match error.get(0) {
        Some(message) => message.to_string(),
        None => "".to_string(),
    };

    let handle_onchange: Callback<String> = props.handle_onchange.clone();
    let onchange: Callback<Event> = Callback::from(move |event: Event| {
        let target: web_sys::EventTarget = event.target().unwrap();
        let value: String = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });

    let handle_on_input_blur: Callback<(String, String)> = props.handle_on_input_blur.clone();
    let on_blur: Callback<FocusEvent> = {
        let cloned_input_name: String = props.name.clone();
        Callback::from(move |event: FocusEvent| {
            let input_name: String = cloned_input_name.clone();
            let target: web_sys::EventTarget = event.target().unwrap();
            let value: String = target.unchecked_into::<HtmlInputElement>().value();
            handle_on_input_blur.emit((input_name, value));
        })
    };

    html! {
    <div class="form-input">
      <label html={props.name.clone()} class="">
        {props.label.clone()}
      </label>
      <input
        type={input_type}
        placeholder=""
        class=""
        ref={props.input_ref.clone()}
        onchange={onchange}
        onblur={on_blur}
      />
    <span>
        {error_message}
    </span>
    </div>
    }
}
