use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use web_sys::{Event, KeyboardEvent};
use yew::prelude::*;

pub enum Message {
    Value(String),
    Clear,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    // pub value: String,
    pub on_change: Callback<Message>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

fn clear_target_on_escape(event: &Event) -> bool {
    if let Some(event) = event.dyn_ref::<KeyboardEvent>() {
        if event.key_code() == 27 {
            if let Some(event_target) = event.target() {
                let target: Result<HtmlInputElement, _> = event_target.dyn_into();

                if let Ok(target) = target {
                    target.set_value("");
                    return true;
                }
            }
        }
    }

    false
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { on_change } = props.clone();

    let on_change_key_event = on_change.clone();

    use_effect(move || {
        let document = gloo::utils::document();
        let listener = EventListener::new(&document, "keydown", move |event| {
            if clear_target_on_escape(event) {
                on_change_key_event.emit(Message::Clear);
            }
        });

        || drop(listener)
    });

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(Message::Value(get_value_from_input_event(input_event)));
    });

    html! {
        <input type="text" {oninput} />
    }
}
