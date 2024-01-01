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
            <button onclick={toggle_dark_mode} class="transition-colors duration-1000">
                {if *dark_mode {
                    html! {
                        <div class="text-white">
                            <svg class="animate-icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" color="white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle fill="white" cx="12" cy="12" r="5" />
                                <line x1="12" y1="1" x2="12" y2="3" />
                                <line x1="12" y1="21" x2="12" y2="23" />
                                <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
                                <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
                                <line x1="1" y1="12" x2="3" y2="12" />
                                <line x1="21" y1="12" x2="23" y2="12" />
                                <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
                                <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
                            </svg>
                        </div>
                    }
                } else {
                    html! {
                        <div class="text-white flex items-center">
                            <svg class="animate-icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="transform: rotate(40deg)">
                                <mask id="mask" >
                                    <rect x="0" y="4" width="100%" height="100%" fill="white" />
                                    <circle cx="12" cy="4" r="7" fill="black" />
                                </mask>
                                <circle fill="black" cx="12" cy="12" r="10" mask="url(#mask)" />
                            </svg>
                        </div>
                    }
                }}
            </button>
        </div>
    }
}
