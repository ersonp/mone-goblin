use gloo_dialogs::alert;
use types::Investment2;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect, use_node_ref, Callback, Html, MouseEvent, Properties,
};

#[derive(Properties, PartialEq)]
pub struct InvestmentFormProps {
    pub createinvestment: Callback<Investment2>,
}

#[function_component(InvestmentForm)]
pub fn investment_form(props: &InvestmentFormProps) -> Html {
    let bank_name = use_node_ref();
    let inv_name = use_node_ref();
    let inv_type = use_node_ref();

    let handle_click = {
        let bank_name = bank_name.clone();
        let inv_name = inv_name.clone();
        let inv_type = inv_type.clone();
        let on_create_investment = props.createinvestment.clone();

        Callback::from(move |_e: MouseEvent| {
            let mut input_bank: String = String::new();
            let mut input_name: String = String::new();
            let mut input_type: String = String::new();
            if let Some(input) = bank_name.cast::<HtmlInputElement>() {
                input_bank = input.value();
                if input_bank.is_empty() {
                    alert("Bank Name can not be blank");
                    return;
                }
                // Reset the input
                input.set_value("");
            }
            if let Some(input) = inv_name.cast::<HtmlInputElement>() {
                input_name = input.value();
                if input_name.is_empty() {
                    alert("Name can not be blank");
                    return;
                }
                // Reset the input
                input.set_value("");
            }
            if let Some(input) = inv_type.cast::<HtmlInputElement>() {
                input_type = input.value();
                if input_type.is_empty() {
                    alert("Investment type can not be blank");
                    return;
                }
                // Reset the input
                input.set_value("");
            }

            let investment = Investment2 {
                id: "".to_string(),
                name: input_name,
                inv_name: input_bank,
                inv_amount: 0,
                return_amount: 0,
                inv_type: input_type,
                return_rate: 0,
                return_rate_type: "test".to_string(),
                start_date: None,
                end_date: None,
                created_at: None,
                updated_at: None,
            };

            on_create_investment.emit(investment);
        })
    };

    {
        let bank_name = bank_name.clone();
        let inv_name = inv_name.clone();
        let inv_type = inv_type.clone();
        use_effect(move || {
            if let Some(input) = bank_name.cast::<HtmlInputElement>() {
                input.focus().unwrap();
            }
            if let Some(input) = inv_name.cast::<HtmlInputElement>() {
                input.focus().unwrap();
            }
            if let Some(input) = inv_type.cast::<HtmlInputElement>() {
                input.focus().unwrap();
            }
        });
    }

    html! {
        <div class="mx-auto w-full">
            <div class="grid gap-6 mb-6 md:grid-cols-2">
                <div>
                    <label for="bank_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Bank Name"}</label>
                    <input ref={bank_name} type="text" id="bank_name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="BCCB" required=true/>
                </div>
                <div>
                    <label for="inv_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Name"}</label>
                    <input ref={inv_name} type="text" id="name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Erson" required=true/>
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
                        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        id="return-rate-type" >
                        <option selected=true disabled=true value="">{"Return Rate type"}</option>
                        <option value="Ordinary">{"Ordinary"}</option>
                        <option value="Culmulative">{"Culmulative"}</option>
                        // Add more options as needed
                    </select>
                </div>
                <div>
                    <label for="inv_amount" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Return Rate"}</label>
                    <input type="number" id="inv_amount" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="90210" required=true/>
                </div>
                <div>
                    <label for="return_amount" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Return Amount"}</label>
                    <input type="number" id="return_amount" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="90210" required=true/>
                </div>
                <div>
                <label for="return_rate" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Return Rate"}</label>
                    <input type="number" id="return_rate" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="90210" required=true/>
                </div>
                <div>
                    <label for="start_date" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Start Date"}</label>
                    <input type="date" id="start_date" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="7000" required=true/>
                </div>
                <div>
                    <label for="end_date" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"End Date"}</label>
                    <input type="date" id="end_date" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="7000" required=true/>
                </div>
                <button onclick={handle_click} type="submit" class="inline-flex items-center px-5 py-2.5 mt-4 sm:mt-6 text-sm font-medium text-center text-white bg-primary-700 rounded-lg focus:ring-4 focus:ring-primary-200 dark:focus:ring-primary-900 hover:bg-primary-800">
                    {"Add Investment"}
                </button>

            </div>
        </div>
    }
}
