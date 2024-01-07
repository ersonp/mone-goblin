use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::events::{Event, InputEvent};
use yew::{html, Callback, Component, Html, Properties, SubmitEvent};

use types::Investment2;

#[derive(Properties, PartialEq, Clone)]
pub struct EditInvForm {
    props: EditInvFormProps,
    error_messages: HashMap<String, String>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct EditInvFormProps {
    pub edit_investment: Callback<Investment2>,
    pub investment: Investment2,
}

pub enum Msg {
    ValidateAndSave(String, String),
    ValidateDateAndSave(String, Option<DateTime<Utc>>),
    SaveForm,
}

impl Component for EditInvForm {
    type Message = Msg;
    type Properties = EditInvFormProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            props: EditInvFormProps {
                edit_investment: ctx.props().edit_investment.clone(),
                investment: ctx.props().investment.clone(),
            },
            error_messages: HashMap::new(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ValidateAndSave(field, value) => match field.as_str() {
                "inv-name" => {
                    self.props.investment.inv_name = value;
                    self.error_messages.remove("inv-name");
                }
                "name" => {
                    self.props.investment.name = value;
                    self.error_messages.remove("name");
                }
                "inv-type" => {
                    self.props.investment.inv_type = value;
                    self.error_messages.remove("inv-type");
                }
                "return-type" => {
                    self.props.investment.return_type = value;
                    self.error_messages.remove("return-type");
                }
                "inv-amount" => {
                    self.props.investment.inv_amount = value.parse().unwrap_or(0);
                    self.error_messages.remove("inv-amount");
                }
                "return-amount" => {
                    self.props.investment.return_amount = value.parse().unwrap_or(0);
                    self.error_messages.remove("return-amount");
                }
                "return-rate" => {
                    self.props.investment.return_rate = value.parse().unwrap_or(0);
                    self.error_messages.remove("return-rate");
                }
                _ => {}
            },
            Msg::ValidateDateAndSave(field, date) => match field.as_str() {
                "start-date" => {
                    self.props.investment.start_date = date;
                    self.error_messages.remove("start-date");
                }
                "end-date" => {
                    self.props.investment.end_date = date;
                    self.error_messages.remove("end-date");
                }
                _ => {}
            },
            Msg::SaveForm => {
                if self.save_form() {
                    // self.reset_form();
                }
            }
        }
        true
    }
    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <form onsubmit={ctx.link().callback(|e: SubmitEvent| { e.prevent_default(); Msg::SaveForm })} class="mx-auto w-full">
                <div class="grid gap-6 mb-6 md:grid-cols-2 lg:grid-cols-3 text-text-950">
                    { self.date_field(ctx, "start-date", &self.props.investment.start_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()) }
                    { self.date_field(ctx, "end-date", &self.props.investment.end_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()) }
                    { self.input_field(ctx, "inv-name", "text", &self.props.investment.inv_name) }
                    { self.input_field(ctx, "name", "text", &self.props.investment.name) }
                    { self.select_field(ctx, "inv-type", &self.props.investment.inv_type,
                        html! {
                            <>
                                <option value="FD" selected={self.props.investment.inv_type == "FD"}>{"FD"}</option>
                                <option value="RD" selected={self.props.investment.inv_type == "RD"}>{"RD"}</option>
                            </>
                        }
                    ) }
                    { self.select_field(ctx, "return-type", &self.props.investment.return_type,
                        html! {
                            <>
                                <option value="Ordinary" selected={self.props.investment.return_type == "Ordinary"}>{"Ordinary"}</option>
                                <option value="Culmulative" selected={self.props.investment.return_type == "Culmulative"}>{"Culmulative"} </option>
                            </>
                        }
                    ) }
                    { self.input_field(ctx, "return-amount", "number", &self.props.investment.return_amount.to_string()) }
                    { self.input_field(ctx, "inv-amount", "number", &self.props.investment.inv_amount.to_string()) }
                    { self.input_field(ctx, "return-rate", "number", &self.props.investment.return_rate.to_string()) }
                    <button type="submit" class="inline-flex justify-center items-center px-5 py-2.5 mt-3 sm:mt-5 text-sm font-medium text-center text-text-50 bg-primary-600 rounded-lg focus:ring-4 focus:ring-primary-200 hover:bg-primary-700">{"Save"}</button>
                </div>
            </form>
        }
    }
}

impl EditInvForm {
    fn input_field(
        &self,
        ctx: &yew::Context<Self>,
        field_id: &str,
        field_type: &str,
        field_value: &str,
    ) -> Html {
        let label_style = "block mb-2 text-sm font-medium";
        let input_style = "border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400";
        let field_id_string = field_id.to_string();
        html! {
            <div>
                <label for={field_id_string.clone()} class={label_style}>{kebab_to_title(field_id)}</label>
                <input
                    type={field_type.to_string()}
                    value={field_value.to_string()}
                    oninput={ctx.link().callback(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                        Msg::ValidateAndSave(field_id_string.clone(), input.value())
                    })}
                    id={field_id_string.clone()}
                    class={input_style}
                />
                { self.error(field_id) }
            </div>
        }
    }

    fn select_field(
        &self,
        ctx: &yew::Context<Self>,
        field_id: &str,
        field_value: &str,
        options: Html,
    ) -> Html {
        let label_style = "block mb-2 text-sm font-medium";
        let input_style = "border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400";
        let field_id_string = field_id.to_string();
        html! {
            <div>
                <label for={field_id_string.clone()} class={label_style}>{kebab_to_title(field_id)}</label>
                <select
                    value={field_value.to_string()}
                    onchange={ctx.link().callback(move |e: Event| {
                        let target = e.target().unwrap();
                        let select_element = target.dyn_into::<HtmlSelectElement>().unwrap();
                        let value = select_element.value();
                        Msg::ValidateAndSave({field_id_string.clone()},value)
                    })}
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

    fn date_field(&self, ctx: &yew::Context<Self>, field_id: &str, field_value: &str) -> Html {
        let label_style: &str = "block mb-2 text-sm font-medium";
        let input_style = "border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400";
        let field_id_string = field_id.to_string();
        html! {
            <div>
                <label for={field_id_string.clone()} class={label_style}>{kebab_to_title(field_id)}</label>
                <input
                    type="date"
                    value={field_value.to_string()}
                    oninput={ctx.link().callback(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                        let date = NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d")
                        .map(|date| date.and_hms_opt(0, 0, 0).map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc)))
                        .ok()
                        .flatten();
                        Msg::ValidateDateAndSave({field_id_string.clone()},date)
                    })}
                    id={field_id_string.clone()}
                    class={format!("{}{}", input_style, " dark:input-dark")}
                />
                { self.error(field_id) }
            </div>
        }
    }

    fn error(&self, field_id: &str) -> Html {
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

    fn validate_form(&mut self) -> bool {
        let mut is_valid = true;

        if self.props.investment.inv_name.is_empty() {
            self.error_messages.insert(
                "inv-name".to_string(),
                "Investment Name can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.name.is_empty() {
            self.error_messages
                .insert("name".to_string(), "Name can not be blank".to_string());
            is_valid = false;
        }

        if self.props.investment.inv_type.is_empty() {
            self.error_messages.insert(
                "inv-type".to_string(),
                "Investment Type can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.return_type.is_empty() {
            self.error_messages.insert(
                "return-type".to_string(),
                "Return Type can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.inv_amount == 0 {
            self.error_messages.insert(
                "inv-amount".to_string(),
                "Investment Amount can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.return_amount == 0 {
            self.error_messages.insert(
                "return-amount".to_string(),
                "Return Amount can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.return_rate == 0 {
            self.error_messages.insert(
                "return-rate".to_string(),
                "Return Rate can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.start_date.is_none() {
            self.error_messages.insert(
                "start-date".to_string(),
                "Start Date can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.end_date.is_none() {
            self.error_messages.insert(
                "end-date".to_string(),
                "End Date can not be blank".to_string(),
            );
            is_valid = false;
        }

        is_valid
    }

    fn save_form(&mut self) -> bool {
        // Validate form fields
        let is_valid = self.validate_form();

        if is_valid {
            self.props
                .edit_investment
                .emit(self.props.investment.clone());
            true
        } else {
            // If the form is not valid, return false
            false
        }
    }
}

fn kebab_to_title(s: &str) -> String {
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
