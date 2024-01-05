use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::events::{Event, InputEvent};
use yew::{html, Callback, Component, Html, Properties, SubmitEvent};

use types::Investment2;

#[derive(Properties, PartialEq, Clone)]
pub struct CreateInvForm {
    state: Investment2,
    props: CreateInvFormProps,
    error_messages: HashMap<String, String>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CreateInvFormProps {
    pub create_investment: Callback<Investment2>,
}

pub enum Msg {
    SaveAndValidate(String, String),
    SaveAndValidateDate(String, Option<DateTime<Utc>>),
    ResetForm,
    SaveForm,
}

impl Component for CreateInvForm {
    type Message = Msg;
    type Properties = CreateInvFormProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            state: Investment2 {
                id: "".to_string(),
                inv_name: "".to_string(),
                name: "".to_string(),
                inv_type: "".to_string(),
                return_type: "".to_string(),
                inv_amount: 0,
                return_amount: 0,
                return_rate: 0,
                start_date: None,
                end_date: None,
                created_at: None,
                updated_at: None,
            },
            props: CreateInvFormProps {
                create_investment: ctx.props().create_investment.clone(),
            },
            error_messages: HashMap::new(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SaveAndValidate(field, value) => match field.as_str() {
                "inv-name" => {
                    self.state.inv_name = value;
                    self.error_messages.remove("inv-name");
                }
                "name" => {
                    self.state.name = value;
                    self.error_messages.remove("name");
                }
                "inv-type" => {
                    self.state.inv_type = value;
                    self.error_messages.remove("inv-type");
                }
                "return-type" => {
                    self.state.return_type = value;
                    self.error_messages.remove("return-type");
                }
                "inv-amount" => {
                    self.state.inv_amount = value.parse().unwrap_or(0);
                    self.error_messages.remove("inv-amount");
                }
                "return-amount" => {
                    self.state.return_amount = value.parse().unwrap_or(0);
                    self.error_messages.remove("return-amount");
                }
                "return-rate" => {
                    self.state.return_rate = value.parse().unwrap_or(0);
                    self.error_messages.remove("return-rate");
                }
                _ => {}
            },
            Msg::SaveAndValidateDate(field, date) => match field.as_str() {
                "start-date" => {
                    self.state.start_date = date;
                    self.error_messages.remove("start-date");
                }
                "end-date" => {
                    self.state.end_date = date;
                    self.error_messages.remove("end-date");
                }
                _ => {}
            },
            Msg::ResetForm => {
                self.reset_form();
            }
            Msg::SaveForm => {
                if self.save_form() {
                    self.reset_form();
                }
            }
        }
        true
    }
    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <form onsubmit={ctx.link().callback(|e: SubmitEvent| { e.prevent_default(); Msg::SaveForm })} class="mx-auto w-full">
                <div class="grid gap-6 mb-6 md:grid-cols-2 lg:grid-cols-3 text-text-950">
                    { self.date_field(ctx, "start-date", &self.state.start_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()) }
                    { self.date_field(ctx, "end-date", &self.state.end_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()) }
                    { self.input_field(ctx, "inv-name", "text", &self.state.inv_name) }
                    { self.input_field(ctx, "name", "text", &self.state.name) }
                    { self.select_field(ctx, "inv-type", &self.state.inv_type,
                        html! {
                            <>
                                <option value="FD">{"FD"}</option>
                                <option value="RD">{"RD"}</option>
                            </>
                        }
                    ) }
                    { self.select_field(ctx, "return-type", &self.state.return_type,
                        html! {
                            <>
                                <option value="Ordinary">{"Ordinary"}</option>
                                <option value="Culmulative">{"Culmulative"}</option>
                            </>
                        }
                    ) }
                    { self.input_field(ctx, "inv-amount", "number", &self.state.inv_amount.to_string()) }
                    { self.input_field(ctx, "return-amount", "number", &self.state.return_amount.to_string()) }
                    { self.input_field(ctx, "return-rate", "number", &self.state.return_rate.to_string()) }
                    <button type="button" onclick={ctx.link().callback(|_| Msg::ResetForm)} class="inline-flex justify-center items-center px-5 py-2.5 mt-4 sm:mt-6 text-sm font-medium text-center text-text-950 bg-background-50 hover:bg-background-100 rounded-lg ring-2 ring-primary-600 ring-inset focus:ring-4 focus:ring-primary-200">{"Reset"}</button>
                    <button type="submit" class="inline-flex justify-center items-center px-5 py-2.5 mt-4 sm:mt-6 text-sm font-medium text-center text-text-50 bg-primary-600 rounded-lg focus:ring-4 focus:ring-primary-200 hover:bg-primary-700">{"Save"}</button>
                </div>
            </form>
        }
    }
}

impl CreateInvForm {
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
                        Msg::SaveAndValidate(field_id_string.clone(), input.value())
                    })}
                    id={field_id_string.clone()}
                    class={input_style}
                />
                {
                    if let Some(error_message) = self.error_messages.get(field_id) {
                        html! { <p class="error">{ error_message }</p> }
                    } else {
                        html! {}
                    }
                }
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
                        Msg::SaveAndValidate({field_id_string.clone()},value)
                    })}
                    id={field_id_string.clone()}
                    class={input_style}
                >
                    <option selected={field_value.is_empty()} disabled=true value={""}>{""}</option>
                    { options }
                </select>
                {
                    if let Some(error_message) = self.error_messages.get(field_id) {
                    html! { <p class="error">{ error_message }</p> }
                    } else {
                        html! {}
                    }
                }
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
                        Msg::SaveAndValidateDate({field_id_string.clone()},date)
                    })}
                    id={field_id_string.clone()}
                    class={format!("{}{}", input_style, " dark:input-dark")}
                />
                {
                    if let Some(error_message) = self.error_messages.get(field_id) {
                        html! { <p class="error">{ error_message }</p> }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }

    fn validate_form(&mut self) -> bool {
        // Perform validation
        if self.state.inv_name.is_empty() {
            self.error_messages.insert(
                "inv-name".to_string(),
                "Investment Name can not be blank".to_string(),
            );
            false
        } else if self.state.name.is_empty() {
            self.error_messages
                .insert("name".to_string(), "Name can not be blank".to_string());
            false
        } else if self.state.inv_type.is_empty() {
            self.error_messages.insert(
                "inv-type".to_string(),
                "Investment type can not be blank".to_string(),
            );
            false
        } else if self.state.return_type.is_empty() {
            self.error_messages.insert(
                "return-type".to_string(),
                "Return type can not be blank".to_string(),
            );
            false
        } else if self.state.inv_amount == 0 {
            self.error_messages.insert(
                "inv-amount".to_string(),
                "Invested Amount can not be blank".to_string(),
            );
            false
        } else if self.state.return_amount == 0 {
            self.error_messages.insert(
                "return-amount".to_string(),
                "Return Amount can not be blank".to_string(),
            );
            false
        } else if self.state.return_rate == 0 {
            self.error_messages.insert(
                "return-rate".to_string(),
                "Return Rate can not be blank".to_string(),
            );
            log::info!("self.state.start_date {:?}", self.state.start_date);
            false
        } else if self.state.start_date.is_none() {
            self.error_messages.insert(
                "start-date".to_string(),
                "Start Date can not be blank".to_string(),
            );
            false
        } else if self.state.end_date.is_none() {
            self.error_messages.insert(
                "end-date".to_string(),
                "End Date can not be blank".to_string(),
            );
            false
        } else {
            true
        }
    }

    fn save_form(&mut self) -> bool {
        // Validate form fields
        let is_valid = self.validate_form();

        if is_valid {
            self.props.create_investment.emit(self.state.clone());
            true
        } else {
            // If the form is not valid, return false
            false
        }
    }

    fn reset_form(&mut self) {
        self.state.inv_name = "".to_string();
        self.state.name = "".to_string();
        self.state.inv_type = "".to_string();
        self.state.return_type = "".to_string();
        self.state.inv_amount = 0;
        self.state.return_amount = 0;
        self.state.return_rate = 0;
        self.state.start_date = None;
        self.state.end_date = None;
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
