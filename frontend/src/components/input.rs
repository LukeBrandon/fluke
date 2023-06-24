use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub field_type: String,
    pub name: String,
    pub placeholder: String, 
    pub input_node_ref: NodeRef,
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        field_type,
        name,
        placeholder,
        input_node_ref
    } = props;

    html! {
        <label for={name.clone()}>
                <input
                    type={field_type.clone()}
                    name={name.clone()}
                    ref={input_node_ref.clone()}
                    placeholder={placeholder.clone()}
                />
        </label>
    }
}
