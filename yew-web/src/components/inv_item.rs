use types::Investment2;
use yew::{html, Callback, Component, Event, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct InvestmentItem {
    pub open: bool,
    pub investment: Investment2,
    pub delete_investment: Callback<String>,
    pub toggle_investment: Callback<String>,
}

pub enum InvestmentItemState {
    Toggle,
}

impl InvestmentItem {
    fn handle_click(&self) -> Callback<MouseEvent> {
        let on_delete_investment = self.delete_investment.clone();
        let id = self.investment.id.clone();
        Callback::from(move |_e: MouseEvent| on_delete_investment.emit(id.clone()))
    }
}

impl Component for InvestmentItem {
    type Message = InvestmentItemState;
    type Properties = InvestmentItem;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            open: false,
            investment: ctx.props().investment.clone(),
            delete_investment: ctx.props().delete_investment.clone(),
            toggle_investment: ctx.props().toggle_investment.clone(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InvestmentItemState::Toggle => {
                self.open = !self.open;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let start_date = if let Some(date) = self.investment.start_date {
            date.date_naive().format("%d-%m-%Y").to_string()
        } else {
            String::new()
        };

        let end_date = if let Some(date) = self.investment.end_date {
            date.date_naive().format("%d-%m-%Y").to_string()
        } else {
            String::new()
        };
        // let date = investment
        //     .created_at
        //     .unwrap_or_default()
        //     .date_naive()
        //     .format("%d-%m-%Y");

        // let time_and_date = &format!(
        //     "{:02}:{:02} • {}",
        //     self.investment.created_at.unwrap_or_default().hour(),
        //     self.investment.created_at.unwrap_or_default().minute(),
        //     date
        // );

        // let completed_investment = match self.investment.completed {
        //     true => Some("text-gray-600 line-through"),
        //     false => None,
        // };

        let handle_toggle = {
            let on_toggle_investment = self.toggle_investment.clone();

            // let id = match self.investment.id.clone() {
            //     Some(id) => id.tb,
            //     None => "".to_string(),
            // };
            // move |_e: Event| on_toggle_investment.emit(id.clone())
            move |_e: Event| on_toggle_investment.emit(self.investment.id.clone())
        };
        html! {
                <>
                    <tr class="border-b dark:border-background-200 hover:bg-background-50">
                        <td class="px-6 py-4 min-w-max whitespace-nowrap hidden sm:table-cell">
                            {start_date.clone()}
                            <dl class="lg:hidden">
                                <dt class="sr-only">{"End Date"}</dt>
                                <dd class="mt-1">{end_date.clone()}</dd>
                            </dl>
                        </td>
                        <td class="px-6 py-4 min-w-max whitespace-nowrap hidden lg:table-cell">{end_date.clone()}</td>
                        <th class="px-6 py-4 min-w-max font-medium text-text-950 ">
                            {&self.investment.clone().inv_name}
                            <dl class="font-normal text-text-500">
                                <dt class="lg:hidden sr-only">{"Name"}</dt>
                                <dd class="lg:hidden mt-1">{&self.investment.clone().name}</dd>
                                <dt class="sm:hidden sr-only">{"Start Date"}</dt>
                                <dd class="sm:hidden mt-1">{start_date.clone()}</dd>
                                <dt class="sm:hidden sr-only">{"End Date"}</dt>
                                <dd class="sm:hidden mt-1">{end_date.clone()}</dd>
                            </dl>
                        </th>
                        <td class="px-6 py-4 min-w-max hidden lg:table-cell">{&self.investment.clone().name}</td>
                        <td class="px-6 py-4 min-w-max hidden sm:table-cell">
                            {&self.investment.clone().inv_type}
                            <dl class="lg:hidden font-normal text-text-500">
                                <dt class="sr-only">{"Return Type"}</dt>
                                <dd class="mt-1">{&self.investment.clone().return_type}</dd>
                                <dt class="sr-only">{"Return Rate"}</dt>
                                <dd class="mt-1">{&self.investment.clone().return_rate}</dd>
                            </dl>
                        </td>
                        <td class="px-6 py-4 min-w-max hidden lg:table-cell">{&self.investment.clone().return_type}</td>
                        <td class="px-6 py-4 min-w-max hidden lg:table-cell">{&self.investment.clone().return_rate}</td>
                        <td class="px-6 py-4 min-w-max hidden lg:table-cell">{&self.investment.clone().inv_amount} </td>
                        <td class="px-6 py-4 min-w-max font-medium text-text-950">
                            {&self.investment.clone().return_amount}
                            <dl class="lg:hidden font-normal text-text-500">
                                <dt class="sr-only">{"Investment"}</dt>
                                <dd class="mt-1">{&self.investment.clone().inv_amount}</dd>
                                <dt class="sr-only sm:hidden">{"Investment Type"}</dt>
                                <dd class="mt-1 sm:hidden">{&self.investment.clone().inv_type}</dd>
                            </dl>
                        </td>
                        <td class="flex flex-col items-start px-6 py-4">
                            <a href="#" class="font-medium text-secondary-600 hover:underline">{"Renew"}</a>
                            <a href="#" class="font-medium text-accent-600 hover:underline">{"Edit"}</a>
                            <a onclick={self.handle_click()} class="font-medium text-red-600 hover:underline">{"Remove"}</a>
                            <button onclick={ctx.link().callback(|_| InvestmentItemState::Toggle)}>
                                { if self.open { "Collapse" } else { "Expand" } }
                            </button>
                        </td>
                        { if self.open {
                            html! {
                                <div>
                                    { "Expanded content" }
                                </div>
                            }
                        } else {
                            Html::default()
                        }}
                    </tr>
                </>

        }
    }
}
