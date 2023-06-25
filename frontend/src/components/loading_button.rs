use super::spinner::Spinner;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub loading: bool,
    pub btn_color: Option<String>,
    pub text_color: Option<String>,
    pub children: Children,
}

#[function_component(LoadingButton)]
pub fn loading_button_component(props: &Props) -> Html {
    let text_color = props
        .text_color
        .clone()
        .unwrap_or_else(|| "text-white".to_string());
    let btn_color = props
        .btn_color
        .clone()
        .unwrap_or_else(|| "bg-ct-yellow-600".to_string());

    let loading_class = if props.loading { "loading" } else { "" };

    html! {
    <button
      type="submit"
      class={format!(
        "loading-button {} {} {}",
         loading_class, btn_color, text_color
      )}
      style="padding: 12px 16px; border-radius: 4px; font-size: 14px; box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.2);"
    >
      if props.loading {
        <div class="flex items-center gap-3">
          <Spinner />
          <span class="loading-text">{"Loading..."}</span>
        </div>
      }else{
        <span class="button-text">{props.children.clone()}</span>
      }
    </button>
    }
}
