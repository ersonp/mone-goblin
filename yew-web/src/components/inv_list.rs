use std::collections::VecDeque;
use types::Investment2;
use yew::{function_component, html, Callback, Html, Properties};

use super::inv_item::InvestmentItem;
use crate::components::accordion::Accordion;

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
                    <Accordion />
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
