use yew::prelude::{html, Callback, Component, Html, Properties};

use types::Investment;

use super::create_inv_form::CreateInvForm;

#[derive(Properties, PartialEq, Clone)]
pub struct ExpandableHeader {
    open: bool,
    pub props: ExpandableHeaderProps,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ExpandableHeaderProps {
    pub create_investment: Callback<Investment>,
}

pub enum ExpandableHeaderState {
    Toggle,
}

impl Component for ExpandableHeader {
    type Message = ExpandableHeaderState;
    type Properties = ExpandableHeaderProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            open: false,
            props: ExpandableHeaderProps {
                create_investment: ctx.props().create_investment.clone(),
            },
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExpandableHeaderState::Toggle => {
                self.open = !self.open;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="w-full mx-auto">
                <div class="bg-background-50 rounded">
                    <div class="">
                        <button  class="flex items-center justify-between w-full p-3 font-medium rtl:text-left" onclick={ctx.link().callback(|_| ExpandableHeaderState::Toggle)}>
                            <span class="flex items-center text-text-950"> {"Total: 5,00,000"}</span>
                                <svg class="w-7 text-text-950" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M2 18H12V20H2V18ZM2 11H22V13H2V11ZM2 4H22V6H2V4ZM18 \
                                18V15H20V18H23V20H20V23H18V20H15V18H18Z" />
                            </svg>
                        </button>
                        <div class={if self.open { "max-h-[1500px] overflow-hidden transition-all duration-100 ease-in-out" } else { "max-h-0 overflow-hidden transition-all duration-100 ease-in-out" }}>
                            <p class="w-full p-4 text-text-950 text-base bg-background-50 rounded-b">
                                <div class="w-full md:w-auto flex flex-col md:flex-row space-y-2 md:space-y-0 items-stretch md:items-center justify-end md:space-x-3 flex-shrink-0">
                                    <CreateInvForm create_investment={self.props.create_investment.clone()} />
                                </div>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
