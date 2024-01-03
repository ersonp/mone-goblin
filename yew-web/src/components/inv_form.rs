use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use gloo_dialogs::alert;
use std::cell::RefCell;
use std::rc::Rc;
use types::Investment2;
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::NodeRef;
use yew::{function_component, html, Callback, Html, Properties, SubmitEvent};

#[derive(Properties, PartialEq)]
pub struct InvestmentFormProps {
    pub create_investment: Callback<Investment2>,
}

struct FormRefs {
    inv_name: NodeRef,
    name: NodeRef,
    inv_type: NodeRef,
    return_type: NodeRef,
    inv_amount: NodeRef,
    return_amount: NodeRef,
    return_rate: NodeRef,
    start_date: NodeRef,
    end_date: NodeRef,
    form: NodeRef,
}

impl FormRefs {
    fn new() -> Self {
        Self {
            inv_name: NodeRef::default(),
            name: NodeRef::default(),
            inv_type: NodeRef::default(),
            return_type: NodeRef::default(),
            inv_amount: NodeRef::default(),
            return_amount: NodeRef::default(),
            return_rate: NodeRef::default(),
            start_date: NodeRef::default(),
            end_date: NodeRef::default(),
            form: NodeRef::default(),
        }
    }

    fn get_inv_name(&self) -> String {
        match validate_input(&self.inv_name, "Investment Name can not be blank") {
            Some(value) => value,
            None => "".to_string(),
        }
    }

    fn get_name(&self) -> String {
        match validate_input(&self.name, "Name can not be blank") {
            Some(value) => value,
            None => "".to_string(),
        }
    }

    fn get_inv_type(&self) -> String {
        match validate_input(&self.inv_type, "Investment type can not be blank") {
            Some(value) => value,
            None => "".to_string(),
        }
    }

    fn get_return_type(&self) -> String {
        match validate_input(&self.return_type, "Return type can not be blank") {
            Some(value) => value,
            None => "".to_string(),
        }
    }

    fn get_inv_amount(&self) -> i32 {
        match validate_input(&self.inv_amount, "Investment Amount can not be blank") {
            Some(value) => value.parse::<i32>().unwrap_or_default(),
            None => 0,
        }
    }

    fn get_return_amount(&self) -> i32 {
        match validate_input(&self.return_amount, "Return Amount can not be blank") {
            Some(value) => value.parse::<i32>().unwrap_or_default(),
            None => 0,
        }
    }

    fn get_return_rate(&self) -> i32 {
        match validate_input(&self.return_rate, "Return Rate can not be blank") {
            Some(value) => value.parse::<i32>().unwrap_or_default(),
            None => 0,
        }
    }

    fn get_date(&self, date: &NodeRef, error_message: &str) -> Option<DateTime<Utc>> {
        match validate_input(date, error_message) {
            Some(value) => {
                let naive_date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap();
                let naive_datetime = match NaiveTime::from_hms_opt(0, 0, 0) {
                    Some(time) => NaiveDateTime::new(naive_date, time),
                    None => {
                        // Handle invalid time here
                        return None;
                    }
                };
                Some(DateTime::<Utc>::from_utc(naive_datetime, Utc))
            }
            None => None,
        }
    }

    fn get_start_date(&self) -> Option<DateTime<Utc>> {
        self.get_date(&self.start_date, "Start Date can not be blank")
    }

    fn get_end_date(&self) -> Option<DateTime<Utc>> {
        self.get_date(&self.end_date, "End Date can not be blank")
    }
}

fn validate_input(input: &NodeRef, error_message: &str) -> Option<String> {
    if let Some(input) = input.cast::<HtmlInputElement>() {
        let value = input.value();
        if value.is_empty() {
            alert(error_message);
            return None;
        }
        return Some(value);
    }
    None
}

#[function_component(InvestmentForm)]
pub fn investment_form(props: &InvestmentFormProps) -> Html {
    let form_refs = Rc::new(RefCell::new(FormRefs::new()));

    let handle_submit = {
        let form_refs = Rc::clone(&form_refs);
        let form_clone = form_refs.borrow().form.clone();
        let on_create_investment = props.create_investment.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let investment = Investment2 {
                id: "".to_string(),
                name: form_refs.borrow().get_name(),
                inv_name: form_refs.borrow().get_inv_name(),
                inv_type: form_refs.borrow().get_inv_type(),
                return_type: form_refs.borrow().get_return_type(),
                inv_amount: form_refs.borrow().get_inv_amount(),
                return_amount: form_refs.borrow().get_return_amount(),
                return_rate: form_refs.borrow().get_return_rate(),
                start_date: form_refs.borrow().get_start_date(),
                end_date: form_refs.borrow().get_end_date(),
                created_at: None,
                updated_at: None,
            };

            on_create_investment.emit(investment);
            // Reset the form
            if let Some(form_element) = form_clone.cast::<HtmlFormElement>() {
                form_element.reset();
            }
        })
    };

    let label_style = "block mb-2 text-sm font-medium";
    let input_style = "border border-background-300 text-text-950 text-sm rounded-lg block w-full p-2.5 bg-background-50 placeholder-text-400";

    html! {
        <form ref={form_refs.borrow().form.clone()} onsubmit={handle_submit} class="mx-auto w-full">
            <div class="grid gap-6 mb-6 md:grid-cols-2 text-text-950">
                <div>
                    <label for="inv-name" class={label_style}>{"Investment Name"}</label>
                    <input ref={form_refs.borrow().inv_name.clone()} type="text" id="inv-name" class={input_style} placeholder="BCCB" required=true/>
                </div>
                <div>
                    <label for="name" class={label_style}>{"Name"}</label>
                    <input ref={form_refs.borrow().name.clone()} type="text" id="name" class={input_style} placeholder="Erson" required=true/>
                </div>
                <div>
                    <label for="investment-type" class={label_style}>{"Select an option"}</label>
                    <select
                        ref={form_refs.borrow().inv_type.clone()}
                        class={input_style}
                        id="investment-type" required=true>
                        <option selected=true disabled=true value="">{"Investment type"}</option>
                        <option value="FD">{"FD"}</option>
                        <option value="RD">{"RD"}</option>
                        // Add more options as needed
                    </select>
                </div>
                <div>
                    <label for="return-rate-type" class={label_style}>{"Select an option"}</label>
                    <select
                        ref={form_refs.borrow().return_type.clone()}
                        class={input_style}
                        id="return-rate-type" required=true>
                        <option selected=true disabled=true value="">{"Return type"}</option>
                        <option value="Ordinary">{"Ordinary"}</option>
                        <option value="Culmulative">{"Culmulative"}</option>
                        // Add more options as needed
                    </select>
                </div>
                <div>
                    <label for="inv-amount" class="block mb-2 text-sm font-medium ">{"Invested Amount"}</label>
                    <input ref={form_refs.borrow().inv_amount.clone()} type="number" id="inv-amount" aria-describedby="helper-text-explanation" class={input_style} placeholder="10000" required=true/>
                </div>
                <div>
                    <label for="return-amount" class={label_style}>{"Return Amount"}</label>
                    <input ref={form_refs.borrow().return_amount.clone()} ype="number" id="return-amount" aria-describedby="helper-text-explanation" class={input_style} placeholder="11000" required=true/>
                </div>
                <div>
                <label for="return-rate" class={label_style}>{"Return Rate"}</label>
                    <input ref={form_refs.borrow().return_rate.clone()} type="number" id="return-rate" aria-describedby="helper-text-explanation" class={input_style} placeholder="8" required=true/>
                </div>
                <div>
                    <label for="start-date" class={label_style}>{"Start Date"}</label>
                    <input ref={form_refs.borrow().start_date.clone()} type="date" id="start-date" class={format!("{}{}", input_style, " dark:input-dark")} placeholder="7000" required=true/>
                </div>
                <div>
                    <label for="end-date" class={label_style}>{"End Date"}</label>
                    <input ref={form_refs.borrow().end_date.clone()} type="date" id="end-date" class={format!("{}{}", input_style, " dark:input-dark")} required=true/>
                </div>
                <button type="submit" class="inline-flex justify-center items-center px-5 py-2.5 mt-4 sm:mt-6 text-sm font-medium text-center text-text-50 bg-primary-700 rounded-lg focus:ring-4 focus:ring-primary-200 hover:bg-primary-600">
                    {"Add Investment"}
                </button>
            </div>
        </form>
    }
}
