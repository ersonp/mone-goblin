use std::collections::HashMap;

use yew::{html, Callback, Event, Html, InputEvent, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BaseFormComponent {
    pub error_messages: HashMap<String, String>,
}

impl BaseFormComponent {
    pub fn input_field(
        &self,
        field_id: &str,
        field_type: &str,
        field_value: &str,
        on_input: Callback<InputEvent>,
    ) -> Html {
        let label_style = "block mb-2 text-sm font-medium";
        let input_style = "border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400";
        let field_id_string = field_id.to_string();
        html! {
            <div>
                <label for={field_id_string.clone()} class={label_style}>{self.kebab_to_title(field_id)}</label>
                <input
                    type={field_type.to_string()}
                    value={field_value.to_string()}
                    oninput={on_input}
                    id={field_id_string.clone()}
                    class={input_style}
                />
                { self.error(field_id) }
            </div>
        }
    }

    pub fn select_field(
        &self,
        field_id: &str,
        field_value: &str,
        options: Html,
        on_change: Callback<Event>,
    ) -> Html {
        let label_style = "block mb-2 text-sm font-medium";
        let input_style = "border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400";
        let field_id_string = field_id.to_string();
        html! {
            <div>
                <label for={field_id_string.clone()} class={label_style}>{self.kebab_to_title(field_id)}</label>
                <select
                    value={field_value.to_string()}
                    onchange={on_change}
                    id={field_id_string.clone()}
                    class={input_style}
                >
                    <option selected={field_value.is_empty()} disabled=true value={""}>{""}</option>
                    { options }
                </select>
                { self.error(field_id) }
            </div>
        }
    }

    pub fn date_field(
        &self,
        field_id: &str,
        field_value: &str,
        on_input: Callback<InputEvent>,
    ) -> Html {
        let label_style: &str = "block mb-2 text-sm font-medium";
        let input_style = "border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400";
        let field_id_string = field_id.to_string();
        html! {
            <div>
                <label for={field_id_string.clone()} class={label_style}>{self.kebab_to_title(field_id)}</label>
                <input
                    type="date"
                    value={field_value.to_string()}
                    oninput={on_input}
                    id={field_id_string.clone()}
                    class={format!("{}{}", input_style, " dark:input-dark")}
                />
                { self.error(field_id) }
            </div>
        }
    }

    pub fn error(&self, field_id: &str) -> Html {
        html! {
            <>
            {
                if let Some(error_message) = self.error_messages.get(field_id) {
                    html! { <p class="error mt-2 text-sm text-red-600 dark:text-red-500">{error_message}</p>}
                } else {
                    html! {}
                }
            }
            </>
        }
    }

    pub fn kebab_to_title(&self, s: &str) -> String {
        s.split('-')
            .map(|part| {
                let mut c = part.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
