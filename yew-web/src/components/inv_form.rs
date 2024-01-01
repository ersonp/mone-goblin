use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use gloo_dialogs::alert;
use types::Investment2;
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::NodeRef;
use yew::{function_component, html, use_node_ref, Callback, Html, Properties, SubmitEvent};

#[derive(Properties, PartialEq)]
pub struct InvestmentFormProps {
    pub createinvestment: Callback<Investment2>,
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
    let inv_name = use_node_ref();
    let name = use_node_ref();
    let inv_type = use_node_ref();
    let return_type = use_node_ref();
    let inv_amount = use_node_ref();
    let return_amount = use_node_ref();
    let return_rate = use_node_ref();
    let start_date = use_node_ref();
    let end_date = use_node_ref();
    let form = NodeRef::default();
    let form_clone = form.clone();
    let handle_submit = {
        let inv_name = inv_name.clone();
        let name = name.clone();
        let inv_type = inv_type.clone();
        let return_type = return_type.clone();
        let inv_amount = inv_amount.clone();
        let return_amount = return_amount.clone();
        let return_rate = return_rate.clone();
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let on_create_investment = props.createinvestment.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let input_inv = match validate_input(&inv_name, "Investment Name can not be blank") {
                Some(value) => value,
                None => return,
            };
            let input_name = match validate_input(&name, "Name can not be blank") {
                Some(value) => value,
                None => return,
            };

            let input_type = match validate_input(&inv_type, "Investment type can not be blank") {
                Some(value) => value,
                None => return,
            };

            let input_return_type =
                match validate_input(&return_type, "Return type can not be blank") {
                    Some(value) => value,
                    None => return,
                };

            let input_inv_amount =
                match validate_input(&inv_amount, "Investment Amount can not be blank") {
                    Some(value) => value.parse::<i32>().unwrap_or_default(),
                    None => return,
                };
            let input_return_amount =
                match validate_input(&return_amount, "Return Amount can not be blank") {
                    Some(value) => value.parse::<i32>().unwrap_or_default(),
                    None => return,
                };
            let input_return_rate =
                match validate_input(&return_rate, "Return Rate can not be blank") {
                    Some(value) => value.parse::<i32>().unwrap_or_default(),
                    None => return,
                };
            let input_start_date = match validate_input(&start_date, "Start Date can not be blank")
            {
                Some(value) => {
                    let naive_date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap();
                    let naive_datetime = match NaiveTime::from_hms_opt(0, 0, 0) {
                        Some(time) => NaiveDateTime::new(naive_date, time),
                        None => {
                            // Handle invalid time here
                            return;
                        }
                    };
                    DateTime::<Utc>::from_utc(naive_datetime, Utc)
                }
                None => return,
            };
            let input_end_date = match validate_input(&end_date, "End Date can not be blank") {
                Some(value) => {
                    let naive_date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap();
                    let naive_datetime = match NaiveTime::from_hms_opt(0, 0, 0) {
                        Some(time) => NaiveDateTime::new(naive_date, time),
                        None => {
                            // Handle invalid time here
                            return;
                        }
                    };
                    DateTime::<Utc>::from_utc(naive_datetime, Utc)
                }
                None => return,
            };

            let investment = Investment2 {
                id: "".to_string(),
                name: input_name,
                inv_name: input_inv,
                inv_amount: input_inv_amount,
                return_amount: input_return_amount,
                inv_type: input_type,
                return_rate: input_return_rate,
                return_type: input_return_type,
                start_date: Some(input_start_date),
                end_date: Some(input_end_date),
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
        <form ref={form.clone()} onsubmit={handle_submit} class="mx-auto w-full">
            <div class="grid gap-6 mb-6 md:grid-cols-2">
                <div>
                    <label for="inv_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Investment Name"}</label>
                    <input ref={inv_name} type="text" id="inv_name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="BCCB" required=true/>
                </div>
                <div>
                    <label for="inv_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Name"}</label>
                    <input ref={name} type="text" id="name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Erson" required=true/>
                </div>
                <div>
                    <label for="countries" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Select an option"}</label>
                    <select
                        ref={inv_type}
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
                        ref={return_type}
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
                    <input ref={inv_amount} type="number" id="inv_amount" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="10000" required=true/>
                </div>
                <div>
                    <label for="return_amount" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Return Amount"}</label>
                    <input ref={return_amount} ype="number" id="return_amount" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="11000" required=true/>
                </div>
                <div>
                <label for="return_rate" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Return Rate"}</label>
                    <input ref={return_rate} type="number" id="return_rate" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="8" required=true/>
                </div>
                <div>
                    <label for="start_date" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Start Date"}</label>
                    <input ref={start_date} type="date" id="start_date" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="7000" required=true/>
                </div>
                <div>
                    <label for="end_date" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"End Date"}</label>
                    <input ref={end_date} type="date" id="end_date" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="7000" required=true/>
                </div>
                <button type="submit" class="inline-flex justify-center items-center px-5 py-2.5 mt-4 sm:mt-6 text-sm font-medium text-center text-white bg-primary-700 rounded-lg focus:ring-4 focus:ring-primary-200 dark:focus:ring-primary-900 hover:bg-primary-800">
                    {"Add Investment"}
                </button>

            </div>
        </form>
    }
}
