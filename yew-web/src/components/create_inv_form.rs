use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::events::{Event, InputEvent};
use yew::{html, Callback, Component, Html, Properties, SubmitEvent};

use super::base_inv_form::BaseFormComponent;
use types::Investment;

#[derive(Properties, PartialEq, Clone)]
pub struct CreateInvForm {
    state: Investment,
    props: CreateInvFormProps,
    base: BaseFormComponent,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CreateInvFormProps {
    pub create_investment: Callback<Investment>,
}

pub enum Form {
    Update(String, String),
    UpdateDate(String, Option<DateTime<Utc>>),
    Reset,
    Save,
}

impl Component for CreateInvForm {
    type Message = Form;
    type Properties = CreateInvFormProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            state: Investment {
                id: None,
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
            base: BaseFormComponent {
                error_messages: HashMap::new(),
            },
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Form::Update(field, value) => {
                self.base.update_field(&mut self.state, &field, value);
            }
            Form::UpdateDate(field, date) => {
                self.base.update_date_field(&mut self.state, &field, date);
            }
            Form::Reset => {
                self.reset_form();
            }
            Form::Save => {
                if self.save_form() {
                    self.reset_form();
                }
            }
        }
        true
    }
    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <form onsubmit={ctx.link().callback(|e: SubmitEvent| { e.prevent_default(); Form::Save })} class="mx-auto w-full">
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
                    { self.input_field(ctx, "return-amount", "number", &self.state.return_amount.to_string()) }
                    { self.input_field(ctx, "inv-amount", "number", &self.state.inv_amount.to_string()) }
                    { self.input_field(ctx, "return-rate", "number", &self.state.return_rate.to_string()) }
                    <button type="button" onclick={ctx.link().callback(|_| Form::Reset)} class="inline-flex justify-center items-center px-5 py-2.5 mt-3 sm:mt-5 text-sm font-medium text-center text-text-950 bg-background-50 hover:bg-background-100 rounded-lg ring-2 ring-primary-600 ring-inset focus:ring-4 focus:ring-primary-200">{"Reset"}</button>
                    <button type="submit" class="inline-flex justify-center items-center px-5 py-2.5 mt-3 sm:mt-5 text-sm font-medium text-center text-text-50 bg-primary-600 rounded-lg focus:ring-4 focus:ring-primary-200 hover:bg-primary-700">{"Save"}</button>
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
        let field_id_str = field_id.to_string();
        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            Form::Update(field_id_str.clone(), input.value())
        });
        self.base
            .input_field(field_id, field_type, field_value, on_input)
    }

    fn select_field(
        &self,
        ctx: &yew::Context<Self>,
        field_id: &str,
        field_value: &str,
        options: Html,
    ) -> Html {
        let field_id_str = field_id.to_string();
        let on_change = ctx.link().callback(move |e: Event| {
            let target = e.target().unwrap();
            let select_element = target.dyn_into::<HtmlSelectElement>().unwrap();
            let value = select_element.value();
            Form::Update(field_id_str.clone(), value)
        });
        self.base
            .select_field(field_id, field_value, options, on_change)
    }

    fn date_field(&self, ctx: &yew::Context<Self>, field_id: &str, field_value: &str) -> Html {
        let field_id_str = field_id.to_string();
        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let date = NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d")
                .map(|date| {
                    date.and_hms_opt(0, 0, 0)
                        .map(|datetime| Utc.from_utc_datetime(&datetime))
                })
                .ok()
                .flatten();
            Form::UpdateDate(field_id_str.clone(), date)
        });

        self.base.date_field(field_id, field_value, on_input)
    }

    fn save_form(&mut self) -> bool {
        // Validate form fields
        let is_valid = self.base.validate_form(&mut self.state);

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
