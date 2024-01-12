use std::collections::HashMap;

use yew::{html, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BaseFormComponent {
    pub error_messages: HashMap<String, String>,
}
// TODO: move other common elements from forms here
impl BaseFormComponent {
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
