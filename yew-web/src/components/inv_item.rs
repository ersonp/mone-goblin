use surrealdb::sql::Thing;
use yew::{html, Callback, Component, Html, Properties};

use super::edit_inv_form::EditInvForm;
use super::renew_inv_form::RenewInvForm;
use types::Investment;

#[derive(Properties, PartialEq, Clone)]
pub struct InvestmentItem {
    open_more: bool,
    open_edit: bool,
    open_renew: bool,
    show_delete_confirmation: bool,
    props: InvestmentItemProps,
}

#[derive(Properties, PartialEq, Clone)]
pub struct InvestmentItemProps {
    pub investment: Investment,
    pub create_investment: Callback<Investment>,
    pub delete_investment: Callback<Thing>,
    pub edit_investment: Callback<Investment>,
}

pub enum InvestmentItemState {
    ToggleExpandMore,
    ToggleExpandEdit,
    ToggleExpandRenew,
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
            open_renew: false,
            show_delete_confirmation: false,
            props: InvestmentItemProps {
                investment: ctx.props().investment.clone(),
                create_investment: ctx.props().create_investment.clone(),
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
                self.open_renew = false;
            }
            InvestmentItemState::ToggleExpandEdit => {
                self.open_edit = !self.open_edit;
                self.open_more = false;
                self.open_renew = false;
            }
            InvestmentItemState::ToggleExpandRenew => {
                self.open_renew = !self.open_renew;
                self.open_more = false;
                self.open_edit = false;
            }
            InvestmentItemState::ToggleDeleteConfirmation => {
                self.show_delete_confirmation = !self.show_delete_confirmation;
            }
            InvestmentItemState::ConfirmDelete => {
                // Delete the item and hide the confirmation overlay
                let on_delete_investment = self.props.delete_investment.clone();
                let id = match self.props.investment.id.clone() {
                    Some(thing) => thing,
                    None => {
                        // Handle the None case here. For example, you can return an error or a default Thing.
                        return Default::default();
                    }
                };
                on_delete_investment.emit(id);
                self.show_delete_confirmation = false;
            }
            InvestmentItemState::CancelDelete => {
                // Hide the confirmation overlay without deleting the item
                self.show_delete_confirmation = false;
            }
        }
        true
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

        let arrow_down = html! {
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4">
                <path fill-rule="evenodd" d="M8 15A7 7 0 1 0 8 1a7 7 0 0 0 0 14Zm.75-10.25a.75.75 0 0 0-1.5 0v4.69L6.03 8.22a.75.75 0 0 0-1.06 1.06l2.5 2.5a.75.75 0 0 0 1.06 0l2.5-2.5a.75.75 0 1 0-1.06-1.06L8.75 9.44V4.75Z" clip-rule="evenodd" />
            </svg>
        };
        let arrow_up = html! {
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4">
                <path fill-rule="evenodd" d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1Zm-.75 10.25a.75.75 0 0 0 1.5 0V6.56l1.22 1.22a.75.75 0 1 0 1.06-1.06l-2.5-2.5a.75.75 0 0 0-1.06 0l-2.5 2.5a.75.75 0 0 0 1.06 1.06l1.22-1.22v4.69Z" clip-rule="evenodd" />
            </svg>
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
                    <tr class={format!("{} {}", {if self.open_more || self.open_edit || self.open_renew { "bg-background-50" } else { "border-b dark:border-background-200 hover:bg-background-50" }}, "relative")}>
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
                        <td class="flex flex-col items-start px-6 py-4 whitespace-nowrap">
                            <button onclick={ctx.link().callback(|_| InvestmentItemState::ToggleDeleteConfirmation)} class="font-medium text-red-600 dark:text-red-500 hover:underline w-full">
                                <div class="flex items-center justify-between w-full rtl:text-left">
                                    {"Delete"}
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4">
                                        <path fill-rule="evenodd" d="M8 15A7 7 0 1 0 8 1a7 7 0 0 0 0 14Zm2.78-4.22a.75.75 0 0 1-1.06 0L8 9.06l-1.72 1.72a.75.75 0 1 1-1.06-1.06L6.94 8 5.22 6.28a.75.75 0 0 1 1.06-1.06L8 6.94l1.72-1.72a.75.75 0 1 1 1.06 1.06L9.06 8l1.72 1.72a.75.75 0 0 1 0 1.06Z" clip-rule="evenodd" />
                                    </svg>
                                </div>
                            </button>
                            <button onclick={ctx.link().callback(|_| InvestmentItemState::ToggleExpandEdit)} class="font-medium text-accent-600 hover:underline w-full">
                                <div class="flex items-center justify-between w-full rtl:text-left">
                                    {"Edit"}{ if self.open_edit { arrow_up.clone() } else { arrow_down.clone() } }
                                </div>
                            </button>
                            <button onclick={ctx.link().callback(|_| InvestmentItemState::ToggleExpandRenew)}  class="font-medium text-secondary-600 hover:underline w-full">
                                <div class="flex items-center justify-between w-full rtl:text-left">
                                    {"Renew"}{ if self.open_renew { arrow_up.clone() } else { arrow_down.clone() } }
                                </div>
                            </button>
                            <button onclick={ctx.link().callback(|_| InvestmentItemState::ToggleExpandMore)} class="w-full">
                                <div class="flex items-center justify-between w-full rtl:text-left">
                                    {"More"}{ if self.open_more { arrow_up.clone() } else { arrow_down.clone() } }
                                </div>
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
                    // Render the expanded content if the item is expanded
                    {if self.open_edit {
                        html! {
                            <tr class="overflow-hidden border-b dark:border-background-200 hover:bg-background-50">
                                <td colspan="100%">
                                    <p class="w-full p-4 text-text-950 text-base bg-background-50 rounded-b">
                                        <div class="w-full md:w-auto flex flex-col md:flex-row space-y-2 md:space-y-0 items-stretch md:items-center justify-end md:space-x-3 flex-shrink-0">
                                            <EditInvForm edit_investment={self.props.edit_investment.clone()} investment={self.props.investment.clone()} on_edit={ctx.link().callback(|_| InvestmentItemState::ToggleExpandEdit)}/>
                                        </div>
                                    </p>
                                </td>
                            </tr>
                        }
                    } else if self.open_more {
                        html! {
                            <tr class="overflow-hidden border-b dark:border-background-200 hover:bg-background-50">
                                <td colspan="100%">
                                    <p class="p-4 text-text-950 text-base bg-background-50 rounded-b">
                                        {self.props.investment.inv_status.as_ref().map_or("No status", |s| &s.status)}
                                    </p>
                                </td>
                            </tr>
                        }
                    }  else if self.open_renew {
                        html! {
                            <tr class="overflow-hidden border-b dark:border-background-200 hover:bg-background-50">
                                <td colspan="100%">
                                    <p class="w-full p-4 text-text-950 text-base bg-background-50 rounded-b">
                                        <div class="w-full md:w-auto flex flex-col md:flex-row space-y-2 md:space-y-0 items-stretch md:items-center justify-end md:space-x-3 flex-shrink-0">
                                            <RenewInvForm create_investment={self.props.create_investment.clone()} edit_investment={self.props.edit_investment.clone()} old_investment={self.props.investment.clone()} on_renew={ctx.link().callback(|_| InvestmentItemState::ToggleExpandRenew)}/>
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
