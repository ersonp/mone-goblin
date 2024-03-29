use std::collections::VecDeque;

use surrealdb::sql::Thing;
use uuid::Uuid;
use yew::{function_component, html, Callback, Html, Properties};

use super::inv_item::InvestmentItem;
use crate::components::exp_table_header::ExpandableHeader;
use types::Investment;

#[derive(Properties, PartialEq)]
pub struct InvestmentListProps {
    pub investments: VecDeque<Investment>,
    pub create_investment: Callback<Investment>,
    pub delete_investment: Callback<Thing>,
    pub edit_investment: Callback<Investment>,
    pub renew_investment: Callback<(Investment, Investment)>,
}

#[function_component(InvestmentList)]
pub fn investment_list(
    InvestmentListProps {
        investments,
        create_investment,
        delete_investment,
        edit_investment,
        renew_investment,
    }: &InvestmentListProps,
) -> Html {
    let investments = investments
    .iter()
    .map(|investment| {
        // Generate a unique key for each investment everytime so that the DOM can be updated correctly
        let uuid = Uuid::new_v4();
        let display_string = match investment.id.clone() {
            Some(thing) => format!("{}", thing),
            None => "No Thing available".to_string(),
        };
        let key = format!("{}-{}", display_string, uuid);
        html!(<InvestmentItem key={key} investment={investment.clone()} create_investment={create_investment.clone()} delete_investment={delete_investment} edit_investment={edit_investment} renew_investment={renew_investment} />)
    })
    .collect::<Html>();

    html! {
        <section class="p-3 sm:p-5">
            <div class="mx-auto px-4 lg:px-12">
                <div class="backdrop-blur-sm bg-white/50 dark:bg-black/70 relative shadow-md dark:shadow-white-md rounded-lg overflow-hidden">
                    <div class="flex flex-col md:flex-row items-center justify-between space-y-3 md:space-y-0 md:space-x-4 p-4">
                        <ExpandableHeader create_investment={create_investment.clone()}/>
                    </div>
                    <div class="overflow-x-auto">
                        <table class="w-full text-sm text-left text-text-600">
                            <thead class="text-xs uppercase bg-background-200">
                                <tr>
                                    <th scope="col" class="px-6 py-3 hidden sm:table-cell">{"Start Date"}</th>
                                    <th scope="col" class="px-6 py-3 hidden lg:table-cell">{"End Date"}</th>
                                    <th scope="col" class="px-6 py-3">{"Investment Name"}</th>
                                    <th scope="col" class="px-6 py-3 hidden lg:table-cell">{"Name"}</th>
                                    <th scope="col" class="px-6 py-3 hidden sm:table-cell">{"Investment Type"}</th>
                                    <th scope="col" class="px-6 py-3 hidden lg:table-cell">{"Return Type"}</th>
                                    <th scope="col" class="px-6 py-3 hidden lg:table-cell">{"Return Rate"}</th>
                                    <th scope="col" class="px-6 py-3 hidden lg:table-cell">{"Investment"}</th>
                                    <th scope="col" class="px-6 py-3">{"Return"}</th>
                                    <th scope="col" class="px-6 py-3">
                                        <span >{"Actions"}</span>
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
