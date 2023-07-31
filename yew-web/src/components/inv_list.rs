use std::collections::VecDeque;
use types::Investment;
use yew::{function_component, html, Callback, Html, Properties};

use super::inv_item::InvestmentItem;

#[derive(Properties, PartialEq)]
pub struct InvestmentListProps {
    pub investments: VecDeque<Investment>,
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
        <ul>
            {investments}
        </ul>
    }
}
