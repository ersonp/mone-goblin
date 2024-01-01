use std::collections::VecDeque;
use types::Investment2;
use yew::{function_component, html, Callback, Html, Properties};

use super::inv_item::InvestmentItem;
use crate::components::accordion::Accordion;

#[derive(Properties, PartialEq)]
pub struct InvestmentListProps {
    pub investments: VecDeque<Investment2>,
    pub createinvestment: Callback<Investment2>,
    pub deleteinvestment: Callback<String>,
    pub toggleinvestment: Callback<String>,
}

#[function_component(InvestmentList)]
pub fn investment_list(
    InvestmentListProps {
        investments,
        createinvestment,
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
                    <Accordion open={true} createinvestment={createinvestment.clone()}/>
                </div>
                <div class="overflow-x-auto">
                    <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                        <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                            <tr>
                                <th scope="col" class="px-6 py-3">{"Start Date"}</th>
                                <th scope="col" class="px-6 py-3">{"End Date"}</th>
                                <th scope="col" class="px-6 py-3">{"Investment Name"}</th>
                                <th scope="col" class="px-6 py-3">{"Name"}</th>
                                <th scope="col" class="px-6 py-3">{"Investment Type"}</th>
                                <th scope="col" class="px-6 py-3">{"Return Rate"}</th>
                                <th scope="col" class="px-6 py-3">{"Return Type"}</th>
                                <th scope="col" class="px-6 py-3">{"Investment"}</th>
                                <th scope="col" class="px-6 py-3">{"Return"}</th>
                                <th scope="col" class="px-6 py-3">
                                    <span >{"More"}</span>
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
