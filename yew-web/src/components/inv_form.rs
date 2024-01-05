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
    UpdateInvName(String),
    UpdateName(String),
    UpdateInvType(String),
    UpdateReturnType(String),
    UpdateInvAmount(i32),
    UpdateReturnAmount(i32),
    UpdateReturnRate(i32),
    UpdateStartDate(Option<DateTime<Utc>>),
    UpdateEndDate(Option<DateTime<Utc>>),
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
            Msg::UpdateInvName(inv_name) => {
                self.state.inv_name = inv_name;
                self.error_messages.remove("inv-name");
            }
            Msg::UpdateName(name) => {
                self.state.name = name;
                self.error_messages.remove("name");
            }
            Msg::UpdateInvType(inv_type) => {
                self.state.inv_type = inv_type;
                self.error_messages.remove("inv-type");
            }
            Msg::UpdateReturnType(return_type) => {
                self.state.return_type = return_type;
                self.error_messages.remove("return-type");
            }
            Msg::UpdateInvAmount(inv_amount) => {
                self.state.inv_amount = inv_amount;
                self.error_messages.remove("inv-amount");
            }
            Msg::UpdateReturnAmount(return_amount) => {
                self.state.return_amount = return_amount;
                self.error_messages.remove("return-amount");
            }
            Msg::UpdateReturnRate(return_rate) => {
                self.state.return_rate = return_rate;
                self.error_messages.remove("return-rate");
            }
            Msg::UpdateStartDate(start_date) => {
                self.state.start_date = start_date;
                self.error_messages.remove("start-date");
            }
            Msg::UpdateEndDate(end_date) => {
                self.state.end_date = end_date;
                self.error_messages.remove("end-date");
            }
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
        let label_style = "block mb-2 text-sm font-medium";
        let input_style = "border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400";
        html! {
            <form onsubmit={ctx.link().callback(|e: SubmitEvent| { e.prevent_default(); Msg::SaveForm })} class="mx-auto w-full">
                <div class="grid gap-6 mb-6 md:grid-cols-2 lg:grid-cols-3 text-text-950">
                    <div>
                        <label for="inv-name" class={label_style}>{"Investment Name"}</label>
                        <input
                            type="text"
                            value={self.state.inv_name.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                                Msg::UpdateInvName(input.value())
                            })}
                            id="inv-name"
                            class={input_style}
                        />
                        {
                            if let Some(error_message) = self.error_messages.get("inv-name") {
                                html! { <p class="error">{ error_message }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                    <label for="name" class={label_style}>{"Name"}</label>
                        <input
                            type="text"
                            value={self.state.name.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                                Msg::UpdateName(input.value())
                            })}
                            id="name"
                            class={input_style}
                        />
                        {
                            if let Some(error_message) = self.error_messages.get("name") {
                            html! { <p class="error">{ error_message }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                        <label for="inv-type" class={label_style}>{"Investment Type"}</label>
                        <select
                            value={self.state.inv_type.clone()}
                            onchange={ctx.link().callback(move |e: Event| {
                                let target = e.target().unwrap();
                                let select_element = target.dyn_into::<HtmlSelectElement>().unwrap();
                                let value = select_element.value();
                                Msg::UpdateInvType(value)
                            })}
                            id="inv-type"
                            class={input_style}
                        >
                            <option selected={self.state.inv_type.is_empty()} disabled=true value={""}>{""}</option>
                            <option value="FD">{"FD"}</option>
                            <option value="RD">{"RD"}</option>
                        </select>
                        {
                            if let Some(error_message) = self.error_messages.get("inv-type") {
                            html! { <p class="error">{ error_message }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                        <label for="return-type" class={label_style}>{"Return Type"}</label>
                        <select
                            value={self.state.return_type.clone()}
                            onchange={ctx.link().callback(move |e: Event| {
                                let target = e.target().unwrap();
                                let select_element = target.dyn_into::<HtmlSelectElement>().unwrap();
                                let value = select_element.value();
                                Msg::UpdateReturnType(value)
                            })}
                            id="return-type"
                            class={input_style}
                        >
                            <option selected={self.state.return_type.is_empty()} disabled=true value={""}>{""}</option>
                            <option value="Ordinary">{"Ordinary"}</option>
                            <option value="Culmulative">{"Culmulative"}</option>
                        </select>
                        {
                            if let Some(error_message) = self.error_messages.get("return-type") {
                            html! { <p class="error">{ error_message }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                        <label for="inv-amount" class="block mb-2 text-sm font-medium ">{"Invested Amount"}</label>
                        <input
                            type="number"
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                                Msg::UpdateInvAmount(input.value().parse().unwrap_or(0))
                            })}
                            value={self.state.inv_amount.to_string()}
                            id="inv-amount"
                            class={input_style}
                        />
                        {
                            if let Some(error_message) = self.error_messages.get("inv-amount") {
                            html! { <p class="error">{ error_message }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                        <label for="return-amount" class={label_style}>{"Return Amount"}</label>
                        <input
                            type="number"
                            value={self.state.return_amount.to_string()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                                Msg::UpdateReturnAmount(input.value().parse().unwrap_or(0))
                            })}
                            id="return-amount"
                            class={input_style}
                        />
                        {
                            if let Some(error_message) = self.error_messages.get("return-amount") {
                            html! { <p class="error">{ error_message }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                        <label for="return-rate" class={label_style}>{"Return Rate"}</label>
                        <input
                            type="number"
                            value={self.state.return_rate.to_string()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                                Msg::UpdateReturnRate(input.value().parse().unwrap_or(0))
                            })}
                            id="return-rate"
                            class={input_style}
                        />
                        {
                            if let Some(error_message) = self.error_messages.get("return-rate") {
                            html! { <p class="error">{ error_message }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                        <label for="start-date" class={label_style}>{"Start Date"}</label>
                        <input
                            type="date"
                            value={self.state.start_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                                let date = NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d")
                                .map(|date| date.and_hms_opt(0, 0, 0).map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc)))
                                .ok()
                                .flatten();
                                Msg::UpdateStartDate(date)
                            })}
                            id="start-date"
                            class={format!("{}{}", input_style, " dark:input-dark")}
                        />
                        {
                            if let Some(error_message) = self.error_messages.get("start-date") {
                            html! { <p class="error">{ error_message }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                        <label for="end-date" class={label_style}>{"End Date"}</label>
                        <input
                            type="date"
                            value={self.state.end_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                                let date = NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d")
                                .map(|date| date.and_hms_opt(0, 0, 0).map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc)))
                                .ok()
                                .flatten();
                                Msg::UpdateEndDate(date)
                            })}
                            id="end-date"
                            class={format!("{}{}", input_style, " dark:input-dark")}
                        />
                        {
                            if let Some(error_message) = self.error_messages.get("end-date") {
                            html! { <p class="error">{ error_message }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <button type="button" onclick={ctx.link().callback(|_| Msg::ResetForm)} class="inline-flex justify-center items-center px-5 py-2.5 text-sm font-medium text-center text-text-950 bg-background-50 hover:bg-background-100 rounded-lg border-2 border-primary-600 hover:border-primary-700 focus:ring-4 focus:ring-primary-200">{"Reset"}</button>
                    <button type="submit" class="inline-flex justify-center items-center px-5 py-2.5 text-sm font-medium text-center text-text-50 bg-primary-600 rounded-lg focus:ring-4 focus:ring-primary-200 hover:bg-primary-700">{"Save"}</button>
                </div>
            </form>
        }
    }
}

impl CreateInvForm {
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
