use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, MouseEvent};
use yew::events::{Event, InputEvent};
use yew::{html, Callback, Component, Html, Properties};

use super::base_inv_form::BaseFormComponent;
use types::Investment;

#[derive(Properties, PartialEq, Clone)]
pub struct EditInvForm {
    form_changed: bool,
    show_edit_confirmation: bool,
    props: EditInvFormProps,
    base: BaseFormComponent,
}

#[derive(Properties, PartialEq, Clone)]
pub struct EditInvFormProps {
    pub edit_investment: Callback<Investment>,
    pub investment: Investment,
    pub on_edit: Callback<()>,
}

pub enum Form {
    Update(String, String),
    UpdateDate(String, Option<DateTime<Utc>>),
    Confirm,
    Cancel,
    Edit,
}

impl Component for EditInvForm {
    type Message = Form;
    type Properties = EditInvFormProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            form_changed: false,
            show_edit_confirmation: false,
            props: EditInvFormProps {
                edit_investment: ctx.props().edit_investment.clone(),
                investment: ctx.props().investment.clone(),
                on_edit: ctx.props().on_edit.clone(),
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
                    .update_field(&mut self.props.investment, &field, value);
                self.form_changed = true;
            }
            Form::UpdateDate(field, date) => {
                self.base
                    .update_date_field(&mut self.props.investment, &field, date);
                self.form_changed = true;
            }
            Form::Confirm => {
                if self.save_form() {
                    self.props.on_edit.emit(());
                }
            }
            Form::Cancel => {
                self.show_edit_confirmation = false;
            }
            Form::Edit => {
                self.show_edit_confirmation = true;
            }
        }
        true
    }
    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="mx-auto w-full relative">
                <form>
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
                        <button type="submit" disabled={!self.form_changed}
                            onclick={ctx.link().callback(|e: MouseEvent| {
                                // prevent the webpage from moving to top when the button is clicked
                                e.prevent_default();
                                Form::Edit
                            })}
                            class={format!("{} {}", {if self.form_changed { "bg-primary-600 hover:bg-primary-700" } else { "bg-background-500" }}, "inline-flex justify-center items-center px-5 py-2.5 mt-3 sm:mt-5 text-sm font-medium text-center text-text-50 rounded-lg focus:ring-4 focus:ring-primary-200")}>
                            {"Update"}
                        </button>
                    </div>
                </form>
                {if self.show_edit_confirmation {
                    html! {
                        <div class="absolute inset-0 flex items-center justify-center bg-white dark:bg-black bg-opacity-80 dark:bg-opacity-70">
                            <div class="bg-background-50 p-4 rounded text-text-950">
                                <p class="mb-2">{"Are you sure you want to edit this Investment?"}</p>
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

impl EditInvForm {
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
        let is_valid = self.base.validate_form(&mut self.props.investment);

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
