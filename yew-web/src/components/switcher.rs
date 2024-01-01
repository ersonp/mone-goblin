use web_sys::{wasm_bindgen::JsCast, window, Element};
use yew::{function_component, html, use_state, Callback, Html};

#[function_component(DarkModeContent)]
pub fn dark_mode_content() -> Html {
    let dark_mode = use_state(|| false);

    let toggle_dark_mode = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            let dark_mode_enabled = !*dark_mode;
            dark_mode.set(dark_mode_enabled);

            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    let root_element = document.document_element().unwrap();
                    let root_element = root_element.dyn_into::<Element>().unwrap();
                    if dark_mode_enabled {
                        root_element.class_list().add_1("dark").unwrap();
                    } else {
                        root_element.class_list().remove_1("dark").unwrap();
                    }
                }
            }
        })
    };

    html! {
        <div>
            <button onclick={toggle_dark_mode}>
                {if *dark_mode { "Switch to Light Mode" } else { "Switch to Dark Mode" }}
            </button>
            {if *dark_mode {
                html! { <div class="text-white">{"This is dark mode content."}</div> }
            } else {
                html! { <div class="text-black">{"This is light mode content."}</div> }
            }}
        </div>
    }
}
