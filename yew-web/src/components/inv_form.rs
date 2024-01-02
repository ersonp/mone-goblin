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
    pub createinvestment: Callback<Investment2>,
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
        let on_create_investment = props.createinvestment.clone();

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

    html! {
        <form ref={form_refs.borrow().form.clone()} onsubmit={handle_submit} class="mx-auto w-full">
            <div class="grid gap-6 mb-6 md:grid-cols-2">
                <div>
                    <label for="inv_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Investment Name"}</label>
                    <input ref={form_refs.borrow().inv_name.clone()} type="text" id="inv_name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="BCCB" required=true/>
                </div>
                <div>
                    <label for="inv_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Name"}</label>
                    <input ref={form_refs.borrow().name.clone()} type="text" id="name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Erson" required=true/>
                </div>
                <div>
                    <label for="countries" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Select an option"}</label>
                    <select
                        ref={form_refs.borrow().inv_type.clone()}
                        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        id="investment-type">
                        <option selected=true disabled=true value="">{"Investment type"}</option>
                        <option value="FD">{"FD"}</option>
                        <option value="RD">{"RD"}</option>
                        // Add more options as needed
                    </select>
                </div>
                <div>
                    <label for="countries" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Select an option"}</label>
                    <select
                        ref={form_refs.borrow().return_type.clone()}
                        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        id="return-rate-type" >
                        <option selected=true disabled=true value="">{"Return type"}</option>
                        <option value="Ordinary">{"Ordinary"}</option>
                        <option value="Culmulative">{"Culmulative"}</option>
                        // Add more options as needed
                    </select>
                </div>
                <div>
                    <label for="inv_amount" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Invested Amount"}</label>
                    <input ref={form_refs.borrow().inv_amount.clone()} type="number" id="inv_amount" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="10000" required=true/>
                </div>
                <div>
                    <label for="return_amount" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Return Amount"}</label>
                    <input ref={form_refs.borrow().return_amount.clone()} ype="number" id="return_amount" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="11000" required=true/>
                </div>
                <div>
                <label for="return_rate" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Return Rate"}</label>
                    <input ref={form_refs.borrow().return_rate.clone()} type="number" id="return_rate" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="8" required=true/>
                </div>
                <div>
                    <label for="start_date" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Start Date"}</label>
                    <input ref={form_refs.borrow().start_date.clone()} type="date" id="start_date" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="7000" required=true/>
                </div>
                <div>
                    <label for="end_date" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"End Date"}</label>
                    <input ref={form_refs.borrow().end_date.clone()} type="date" id="end_date" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="7000" required=true/>
                </div>
                <button type="submit" class="inline-flex justify-center items-center px-5 py-2.5 mt-4 sm:mt-6 text-sm font-medium text-center text-white bg-primary-700 rounded-lg focus:ring-4 focus:ring-primary-200 dark:focus:ring-primary-900 hover:bg-primary-800">
                    {"Add Investment"}
                </button>
            </div>
        </form>
    }
}
