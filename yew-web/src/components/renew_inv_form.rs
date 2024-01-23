use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, MouseEvent};
use yew::events::{Event, InputEvent};
use yew::{html, Callback, Component, Html, Properties};

use super::base_inv_form::BaseFormComponent;
use types::{InvStatus, Investment};

#[derive(Properties, PartialEq, Clone)]
pub struct RenewInvForm {
    form_changed: bool,
    show_renew_confirmation: bool,
    props: RenewInvFormProps,
    base: BaseFormComponent,
    renew_investment: Investment,
}

#[derive(Properties, PartialEq, Clone)]
pub struct RenewInvFormProps {
    pub renew_investment: Callback<(Investment, Investment)>,
    pub old_investment: Investment,
    pub on_renew: Callback<()>,
}

pub enum Form {
    Update(String, String),
    UpdateDate(String, Option<DateTime<Utc>>),
    Confirm,
    Cancel,
    Renew,
}

impl Component for RenewInvForm {
    type Message = Form;
    type Properties = RenewInvFormProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            form_changed: false,
            show_renew_confirmation: false,
            renew_investment: Investment {
                id: None,
                inv_name: ctx.props().old_investment.inv_name.clone(),
                name: ctx.props().old_investment.name.clone(),
                inv_type: ctx.props().old_investment.inv_type.clone(),
                return_type: ctx.props().old_investment.return_type.clone(),
                inv_amount: ctx.props().old_investment.return_amount,
                return_amount: 0,
                return_rate: 0,
                inv_status: None,
                start_date: ctx.props().old_investment.end_date,
                end_date: None,
                created_at: None,
                updated_at: None,
            },
            props: RenewInvFormProps {
                renew_investment: ctx.props().renew_investment.clone(),
                old_investment: ctx.props().old_investment.clone(),
                on_renew: ctx.props().on_renew.clone(),
            },
            base: BaseFormComponent {
                error_messages: HashMap::new(),
            },
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Form::Update(field, value) => {
                self.base
                    .update_field(&mut self.renew_investment, &field, value);
                self.form_changed = true;
            }
            Form::UpdateDate(field, date) => {
                self.base
                    .update_date_field(&mut self.renew_investment, &field, date);
                self.form_changed = true;
            }
            Form::Confirm => {
                if self.save_form() {
                    self.props.on_renew.emit(());
                }
            }
            Form::Cancel => {
                self.show_renew_confirmation = false;
            }
            Form::Renew => {
                self.show_renew_confirmation = true;
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="mx-auto w-full relative">
                <form>
                    <div class="grid gap-6 mb-6 md:grid-cols-2 lg:grid-cols-3 text-text-950">
                        { self.date_field(ctx, "start-date", &self.renew_investment.start_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()) }
                        { self.date_field(ctx, "end-date", &self.renew_investment.end_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()) }
                        { self.input_field(ctx, "inv-name", "text", &self.renew_investment.inv_name) }
                        { self.input_field(ctx, "name", "text", &self.renew_investment.name) }
                        { self.select_field(ctx, "inv-type", &self.renew_investment.inv_type,
                            html! {
                                <>
                                    <option value="FD" selected={self.renew_investment.inv_type == "FD"}>{"FD"}</option>
                                    <option value="RD" selected={self.renew_investment.inv_type == "RD"}>{"RD"}</option>
                                </>
                            }
                        ) }
                        { self.select_field(ctx, "return-type", &self.renew_investment.return_type,
                            html! {
                                <>
                                    <option value="Ordinary" selected={self.renew_investment.return_type == "Ordinary"}>{"Ordinary"}</option>
                                    <option value="Culmulative" selected={self.renew_investment.return_type == "Culmulative"}>{"Culmulative"} </option>
                                </>
                            }
                        ) }
                        { self.input_field(ctx, "return-amount", "number", &self.renew_investment.return_amount.to_string()) }
                        { self.input_field(ctx, "inv-amount", "number", &self.renew_investment.inv_amount.to_string()) }
                        { self.input_field(ctx, "return-rate", "number", &self.renew_investment.return_rate.to_string()) }
                        <button type="submit" disabled={!self.form_changed}
                            onclick={ctx.link().callback(|e: MouseEvent| {
                                // prevent the webpage from moving to top when the button is clicked
                                e.prevent_default();
                                Form::Renew
                            })}
                            class={format!("{} {}", {if self.form_changed { "bg-primary-600 hover:bg-primary-700" } else { "bg-background-500" }}, "inline-flex justify-center items-center px-5 py-2.5 mt-3 sm:mt-5 text-sm font-medium text-center text-text-50 rounded-lg focus:ring-4 focus:ring-primary-200")}>
                            {"Renew"}
                        </button>
                    </div>
                </form>
                {if self.show_renew_confirmation {
                    html! {
                        <div class="absolute inset-0 flex items-center justify-center bg-white dark:bg-black bg-opacity-80 dark:bg-opacity-70">
                            <div class="bg-background-50 p-4 rounded text-text-950">
                                <p class="mb-2">{"Are you sure you want to renew this Investment?"}</p>
                                <div class="flex justify-center">
                                    <button onclick={ctx.link().callback(|_| Form::Confirm)} class="bg-red-500 px-4 py-2 mr-1 rounded">{"Confirm"}</button>
                                    <button onclick={ctx.link().callback(|_| Form::Cancel)} class="bg-background-500 px-4 py-2 ml-1 rounded">{"Cancel"}</button>
                                </div>
                            </div>
                        </div>
                    }
                } else { html! {} } }
            </div>
        }
    }
}

impl RenewInvForm {
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
        let is_valid = self.base.validate_form(&mut self.renew_investment);

        if is_valid {
            // add inv_status to renewed investment with the id of the old investment
            // and status as "renewed"
            self.renew_investment.inv_status = Some(InvStatus {
                id: self.props.old_investment.id.clone(),
                status: "renewed".to_string(),
            });

            // update the old investment with status as "closed"
            let mut old_investment = self.props.old_investment.clone();
            if let Some(old_status) = &old_investment.inv_status {
                old_investment.inv_status = Some(InvStatus {
                    id: old_status.id.clone(), // keep the id same as before
                    status: "closed".to_string(),
                });
            } else {
                // Handle the case where inv_status is None
                old_investment.inv_status = Some(InvStatus {
                    id: None,
                    status: "closed".to_string(),
                });
            }

            self.props.renew_investment.emit((
                self.props.old_investment.clone(),
                self.renew_investment.clone(),
            ));

            true
        } else {
            // If the form is not valid, return false
            false
        }
    }
}
