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
            <label class="text-xl font-semibold" for="new-investment">{"Add Item"}</label>

            <hr class="mb-4 border-t-2" />

            <div class="flex justify-center items-center gap-4 flex-col my-2 pl-4 py-1">
                <input
                    ref={bank_name}
                    class="rounded-md focus:outline-none focus:ring focus:ring-blue-400 text-xl px-4 py-2 bg-slate-700"
                    id="bank-name"
                    type="text"
                    placeholder="Bank Name" />
                <input
                    ref={inv_name}
                    class="rounded-md focus:outline-none focus:ring focus:ring-blue-400 text-xl px-4 py-2 bg-slate-700"
                    id="name"
                    type="text"
                    placeholder="Name" />
                <select
                    ref={inv_type}
                    class="rounded-md focus:outline-none focus:ring focus:ring-blue-400 text-xl px-4 py-2 bg-slate-700"
                    id="investment-type" placeholder="Name">
                    <option selected=true disabled=true value="">{"Investment type"}</option>
                    <option value="FD">{"FD"}</option>
                    <option value="RD">{"RD"}</option>
                    // Add more options as needed
                </select>

                <button onclick={handle_click} title="Add Todo" class="bg-sky-600 hover:bg-sky-400 rounded-md text-xl px-4 py-2">
                    <svg class="w-7" fill="currentColor" viewBox="0 0 24 24">
                        <path d="M2 18H12V20H2V18ZM2 11H22V13H2V11ZM2 4H22V6H2V4ZM18 \
                        18V15H20V18H23V20H20V23H18V20H15V18H18Z" />
                    </svg>
                </button>
            </div>
        </div>
    }
}
