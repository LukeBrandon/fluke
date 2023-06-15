use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    // pub input_value: String,
    // pub on_cautious_change: Callback<ChangeData>,
    pub field_type: String,
    pub name: String,
    pub input_node_ref: NodeRef,
    pub placeholder: String,  // Add this line
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        // input_value,
        // on_cautious_change,
        field_type,
        name,
        input_node_ref,
        placeholder,  // Add this line
    } = props;

    html! {
        <label for={name.clone()}>
                <input
                    /* onchange={on_cautious_change} */
                    type={field_type.clone()}
                    /* value={input_value.clone()} */
                    name={name.clone()}
                    ref={input_node_ref.clone()}
                    placeholder={placeholder.clone()}  // Add this line
                />
        </label>
    }
}