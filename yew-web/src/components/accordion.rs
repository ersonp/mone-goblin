use inv_form::InvestmentForm;
use types::Investment2;
use yew::prelude::*;

use super::inv_form;

#[derive(Properties, PartialEq, Clone)]
pub struct Accordion {
    pub open: bool,
    pub creat_investment: Callback<Investment2>,
}

pub enum Form {
    Toggle,
}

impl Component for Accordion {
    type Message = Form;
    type Properties = Accordion;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            open: false,
            creat_investment: ctx.props().creat_investment.clone(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Form::Toggle => {
                self.open = !self.open;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="w-full mx-auto">
                <div class="bg-background-50 rounded">
                    <div class="transition-all duration-500 ease-in-out">
                        <button  class="flex items-center justify-between w-full p-3 font-medium rtl:text-left" onclick={ctx.link().callback(|_| Form::Toggle)}>
                            <span class="flex items-center text-text-950"> {"Total: 5,00,000"}</span>
                                <svg class="w-7 text-text-950" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M2 18H12V20H2V18ZM2 11H22V13H2V11ZM2 4H22V6H2V4ZM18 \
                                18V15H20V18H23V20H20V23H18V20H15V18H18Z" />
                            </svg>
                        </button>
                        <div class={if self.open { "max-h-[1000px] transition-all duration-500 ease-in-out overflow-hidden" } else { "max-h-0 transition-all duration-500 ease-in-out overflow-hidden" }}>
                            <p class="w-full p-4 text-text-950 text-base bg-background-50 rounded-b">
                                <div class="w-full md:w-auto flex flex-col md:flex-row space-y-2 md:space-y-0 items-stretch md:items-center justify-end md:space-x-3 flex-shrink-0">
                                    <InvestmentForm creat_investment={self.creat_investment.clone()} />
                                </div>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
