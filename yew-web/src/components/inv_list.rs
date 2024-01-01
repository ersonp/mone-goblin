use std::collections::VecDeque;
use types::Investment2;
use yew::{function_component, html, Callback, Html, Properties};

use super::inv_item::InvestmentItem;

#[derive(Properties, PartialEq)]
pub struct InvestmentListProps {
    pub investments: VecDeque<Investment2>,
    pub deleteinvestment: Callback<String>,
    pub toggleinvestment: Callback<String>,
}

#[function_component(InvestmentList)]
pub fn investment_list(
    InvestmentListProps {
        investments,
        deleteinvestment,
        toggleinvestment,
    }: &InvestmentListProps,
) -> Html {
    let investments = investments
        .iter()
        .map(|investment| html!(<InvestmentItem investment={investment.clone()} deleteinvestment={deleteinvestment} toggleinvestment={toggleinvestment} />))
        .collect::<Html>();

    html! {

        <section class="bg-gray-50 dark:bg-gray-900 p-3 sm:p-5">
        <div class="mx-auto max-w-screen-xl px-4 lg:px-12">
            <div class="bg-white dark:bg-gray-800 relative shadow-md sm:rounded-lg overflow-hidden">
                <div class="flex flex-col md:flex-row items-center justify-between space-y-3 md:space-y-0 md:space-x-4 p-4">
                    <div class="w-full md:w-1/2">
                    </div>
                    <div class="w-full md:w-auto flex flex-col md:flex-row space-y-2 md:space-y-0 items-stretch md:items-center justify-end md:space-x-3 flex-shrink-0">
                        <button type="button" class="flex items-center justify-center text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-primary-600 dark:hover:bg-primary-700 focus:outline-none dark:focus:ring-primary-800">
                            {"Add product"}
                        </button>
                    </div>
                </div>
                <div class="overflow-x-auto">
                    <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                        <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                            <tr>
                                <th scope="col" class="px-6 py-3">{"Product name"}</th>
                                <th scope="col" class="px-6 py-3">{"Category"}</th>
                                <th scope="col" class="px-6 py-3">{"Brand"}</th>
                                <th scope="col" class="px-6 py-3">{"Description"}</th>
                                <th scope="col" class="px-6 py-3">{"Price"}</th>
                                <th scope="col" class="px-6 py-3">
                                    <span class="sr-only">{"Actions"}</span>
                                </th>
                            </tr>
                        </thead>
                        {investments}
                    </table>
                </div>
            <nav class="flex flex-col md:flex-row justify-between items-start md:items-center space-y-3 md:space-y-0 p-4" aria-label="Table navigation">
            </nav>
        </div>
    </div>
    </section>
    }
}
