use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub field_type: String,
    pub name: String,
    pub placeholder: String, // Add this line
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        field_type,
        name,
        placeholder,
    } = props;

    html! {
        <label for={name.clone()}>
                <input
                    type={field_type.clone()}
                    name={name.clone()}
                    ref={NodeRef::default()}
                    placeholder={placeholder.clone()}
                />
        </label>
    }
}
