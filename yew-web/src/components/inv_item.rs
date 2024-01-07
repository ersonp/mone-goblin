use yew::{html, Callback, Component, Html, Properties};

use super::edit_inv_form::EditInvForm;
use types::Investment2;

#[derive(Properties, PartialEq, Clone)]
pub struct InvestmentItem {
    open_more: bool,
    open_edit: bool,
    show_delete_confirmation: bool,
    props: InvestmentItemProps,
}

#[derive(Properties, PartialEq, Clone)]
pub struct InvestmentItemProps {
    pub investment: Investment2,
    pub delete_investment: Callback<String>,
    pub edit_investment: Callback<Investment2>,
}

pub enum InvestmentItemState {
    ToggleExpandMore,
    ToggleExpandEdit,
    ToggleDeleteConfirmation,
    ConfirmDelete,
    CancelDelete,
}

impl Component for InvestmentItem {
    type Message = InvestmentItemState;
    type Properties = InvestmentItemProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            open_more: false,
            open_edit: false,
            show_delete_confirmation: false,
            props: InvestmentItemProps {
                investment: ctx.props().investment.clone(),
                delete_investment: ctx.props().delete_investment.clone(),
                edit_investment: ctx.props().edit_investment.clone(),
            },
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InvestmentItemState::ToggleExpandMore => {
                self.open_more = !self.open_more;
                self.open_edit = false;
                true
            }
            InvestmentItemState::ToggleExpandEdit => {
                self.open_edit = !self.open_edit;
                self.open_more = false;
                true
            }
            InvestmentItemState::ToggleDeleteConfirmation => {
                self.show_delete_confirmation = !self.show_delete_confirmation;
                true
            }
            InvestmentItemState::ConfirmDelete => {
                // Delete the item and hide the confirmation overlay
                let on_delete_investment = self.props.delete_investment.clone();
                let id = self.props.investment.id.clone();
                on_delete_investment.emit(id);
                self.show_delete_confirmation = false;
                true
            }
            InvestmentItemState::CancelDelete => {
                // Hide the confirmation overlay without deleting the item
                self.show_delete_confirmation = false;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let start_date = if let Some(date) = self.props.investment.start_date {
            date.date_naive().format("%d-%m-%Y").to_string()
        } else {
            String::new()
        };

        let end_date = if let Some(date) = self.props.investment.end_date {
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

        html! {
                <>
                    <tr class={format!("{} {}", {if self.open_more || self.open_edit { "bg-background-50" } else { "border-b dark:border-background-200 hover:bg-background-50" }}, "relative")}>
                        <td class="px-6 py-4 min-w-max whitespace-nowrap hidden sm:table-cell">
                            {start_date.clone()}
                            <dl class="lg:hidden">
                                <dt class="sr-only">{"End Date"}</dt>
                                <dd class="mt-1">{end_date.clone()}</dd>
                            </dl>
                        </td>
                        <td class="px-6 py-4 min-w-max whitespace-nowrap hidden lg:table-cell">{end_date.clone()}</td>
                        <th class="px-6 py-4 min-w-max font-medium text-text-950 ">
                            {&self.props.investment.clone().inv_name}
                            <dl class="font-normal text-text-500">
                                <dt class="lg:hidden sr-only">{"Name"}</dt>
                                <dd class="lg:hidden mt-1">{&self.props.investment.clone().name}</dd>
                                <dt class="sm:hidden sr-only">{"Start Date"}</dt>
                                <dd class="sm:hidden mt-1">{start_date.clone()}</dd>
                                <dt class="sm:hidden sr-only">{"End Date"}</dt>
                                <dd class="sm:hidden mt-1">{end_date.clone()}</dd>
                            </dl>
                        </th>
                        <td class="px-6 py-4 min-w-max hidden lg:table-cell">{&self.props.investment.clone().name}</td>
                        <td class="px-6 py-4 min-w-max hidden sm:table-cell">
                            {&self.props.investment.clone().inv_type}
                            <dl class="lg:hidden font-normal text-text-500">
                                <dt class="sr-only">{"Return Type"}</dt>
                                <dd class="mt-1">{&self.props.investment.clone().return_type}</dd>
                                <dt class="sr-only">{"Return Rate"}</dt>
                                <dd class="mt-1">{&self.props.investment.clone().return_rate}</dd>
                            </dl>
                        </td>
                        <td class="px-6 py-4 min-w-max hidden lg:table-cell">{&self.props.investment.clone().return_type}</td>
                        <td class="px-6 py-4 min-w-max hidden lg:table-cell">{&self.props.investment.clone().return_rate}</td>
                        <td class="px-6 py-4 min-w-max hidden lg:table-cell">{&self.props.investment.clone().inv_amount} </td>
                        <td class="px-6 py-4 min-w-max font-medium text-text-950">
                            {&self.props.investment.clone().return_amount}
                            <dl class="lg:hidden font-normal text-text-500">
                                <dt class="sr-only">{"Investment"}</dt>
                                <dd class="mt-1">{&self.props.investment.clone().inv_amount}</dd>
                                <dt class="sr-only sm:hidden">{"Investment Type"}</dt>
                                <dd class="mt-1 sm:hidden">{&self.props.investment.clone().inv_type}</dd>
                            </dl>
                        </td>
                        <td class="flex flex-col items-start px-6 py-4">
                            <a href="#" class="font-medium text-secondary-600 hover:underline">{"Renew"}</a>
                            <button onclick={ctx.link().callback(|_| InvestmentItemState::ToggleExpandEdit)} class="font-medium text-accent-600 hover:underline">
                                { "Edit"}
                            </button>
                            <button onclick={ctx.link().callback(|_| InvestmentItemState::ToggleDeleteConfirmation)} class="font-medium text-red-600 dark:text-red-500 hover:underline">
                                {"Remove"}
                            </button>
                            <button onclick={ctx.link().callback(|_| InvestmentItemState::ToggleExpandMore)}>
                                { if self.open_more { "Less" } else { "More" } }
                            </button>
                        </td>
                        <td class={if self.show_delete_confirmation { "absolute inset-0 flex items-center justify-center bg-white dark:bg-black bg-opacity-80 dark:bg-opacity-70" } else { "hidden" }}>
                            <div class="bg-background-50 p-4 rounded text-text-950">
                                <p class="mb-2">{"Are you sure you want to delete this Investment?"}</p>
                                <div class="flex justify-center">
                                    <button onclick={ctx.link().callback(|_| InvestmentItemState::ConfirmDelete)} class="bg-red-500 px-4 py-2 mr-1 rounded">{"Confirm"}</button>
                                    <button onclick={ctx.link().callback(|_| InvestmentItemState::CancelDelete)} class="bg-background-500 px-4 py-2 ml-1 rounded">{"Cancel"}</button>
                                </div>
                            </div>
                        </td>
                    </tr>
                    <tr class={format!("{} {}", {if self.open_more { "" } else { "hidden" }}, "overflow-hidden border-b dark:border-background-200 hover:bg-background-50")}>
                        <td colspan="100%">
                            <p class="p-4 text-text-950 text-base bg-background-50 rounded-b">
                                { "Expanded content" }
                            </p>
                        </td>
                    </tr>
                    // Render the edit form if the edit button is clicked
                    {if self.open_edit {
                        html! {
                            <tr class="overflow-hidden border-b dark:border-background-200 hover:bg-background-50">
                                <td colspan="100%">
                                    <p class="w-full p-4 text-text-950 text-base bg-background-50 rounded-b">
                                        <div class="w-full md:w-auto flex flex-col md:flex-row space-y-2 md:space-y-0 items-stretch md:items-center justify-end md:space-x-3 flex-shrink-0">
                                            <EditInvForm edit_investment={self.props.edit_investment.clone()} investment={self.props.investment.clone()}/>
                                        </div>
                                    </p>
                                </td>
                            </tr>
                        }
                    } else {
                        html! {}
                    }}

                </>

        }
    }
}
