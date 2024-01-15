use std::collections::HashMap;

use chrono::{DateTime, Utc};
use yew::{html, Callback, Event, Html, InputEvent, Properties};

use types::Investment;

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
        let field_id_string = field_id.to_string();
        html! {
            <div>
                <label for={field_id_string.clone()} class="block mb-2 text-sm font-medium">{self.kebab_to_title(field_id)}</label>
                <input
                    type={field_type.to_string()}
                    value={field_value.to_string()}
                    oninput={on_input}
                    id={field_id_string.clone()}
                    class="border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400"
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
        let field_id_string = field_id.to_string();
        html! {
            <div>
                <label for={field_id_string.clone()} class="block mb-2 text-sm font-medium">{self.kebab_to_title(field_id)}</label>
                <select
                    value={field_value.to_string()}
                    onchange={on_change}
                    id={field_id_string.clone()}
                    class="border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400"
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
        let field_id_string = field_id.to_string();
        html! {
            <div>
                <label for={field_id_string.clone()} class="block mb-2 text-sm font-medium">{self.kebab_to_title(field_id)}</label>
                <input
                    type="date"
                    value={field_value.to_string()}
                    oninput={on_input}
                    id={field_id_string.clone()}
                    class="border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400 dark:input-dark"
                />
                { self.error(field_id) }
            </div>
        }
    }
    pub fn update_field(&mut self, investment: &mut Investment, field: &str, value: String) {
        match field {
            "inv-name" => {
                investment.inv_name = value;
            }
            "name" => {
                investment.name = value;
            }
            "inv-type" => {
                investment.inv_type = value;
            }
            "return-type" => {
                investment.return_type = value;
            }
            "return-rate" => {
                investment.return_rate = value.parse().unwrap_or(0);
            }
            "inv-amount" => {
                investment.inv_amount = value.parse().unwrap_or(0);
                if investment.inv_amount < investment.return_amount {
                    self.error_messages.remove("return-amount");
                }
            }
            "return-amount" => {
                investment.return_amount = value.parse().unwrap_or(0);
                if investment.inv_amount < investment.return_amount {
                    self.error_messages.remove("inv-amount");
                }
            }
            _ => {}
        }
        self.error_messages.remove(field);
    }

    pub fn update_date_field(
        &mut self,
        investment: &mut Investment,
        field: &str,
        value: Option<DateTime<Utc>>,
    ) {
        match field {
            "start-date" => {
                investment.start_date = value;
            }
            "end-date" => {
                investment.end_date = value;
            }
            _ => {}
        }
        self.error_messages.remove(field);
    }

    pub fn validate_form(&mut self, investment: &mut Investment) -> bool {
        let mut is_valid = true;

        if investment.inv_name.is_empty() {
            self.error_messages.insert(
                "inv-name".to_string(),
                "Investment Name can not be blank".to_string(),
            );
            is_valid = false;
        }

        if investment.name.is_empty() {
            self.error_messages
                .insert("name".to_string(), "Name can not be blank".to_string());
            is_valid = false;
        }

        if investment.inv_type.is_empty() {
            self.error_messages.insert(
                "inv-type".to_string(),
                "Investment Type can not be blank".to_string(),
            );
            is_valid = false;
        }

        if investment.return_type.is_empty() {
            self.error_messages.insert(
                "return-type".to_string(),
                "Return Type can not be blank".to_string(),
            );
            is_valid = false;
        }

        if investment.inv_amount == 0 {
            self.error_messages.insert(
                "inv-amount".to_string(),
                "Investment Amount can not be blank".to_string(),
            );
            is_valid = false;
        }

        if investment.return_amount == 0 {
            self.error_messages.insert(
                "return-amount".to_string(),
                "Return Amount can not be blank".to_string(),
            );
            is_valid = false;
        }

        if investment.inv_amount > investment.return_amount {
            self.error_messages.insert(
                "inv-amount".to_string(),
                "Investment Amount can not be more than Return Amount".to_string(),
            );
            self.error_messages.insert(
                "return-amount".to_string(),
                "Return Amount can not be less than Investment Amount".to_string(),
            );
            is_valid = false;
        }

        if investment.return_rate == 0 {
            self.error_messages.insert(
                "return-rate".to_string(),
                "Return Rate can not be blank".to_string(),
            );
            is_valid = false;
        }

        if investment.start_date.is_none() {
            self.error_messages.insert(
                "start-date".to_string(),
                "Start Date can not be blank".to_string(),
            );
            is_valid = false;
        }

        if investment.end_date.is_none() {
            self.error_messages.insert(
                "end-date".to_string(),
                "End Date can not be blank".to_string(),
            );
            is_valid = false;
        }

        is_valid
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

// In shared_form_utils.rs
