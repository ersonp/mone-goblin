use gloo_dialogs::alert;
use wasm_bindgen_futures::spawn_local;
use yew::UseReducerHandle;

use crate::{inv_api::*, state::*};

pub struct InvestmentController {
    state: UseReducerHandle<InvestmentState>,
}

impl InvestmentController {
    pub fn new(state: UseReducerHandle<InvestmentState>) -> InvestmentController {
        InvestmentController { state }
    }

    pub fn init_investments(&self) {
        let investments = self.state.clone();

        spawn_local(async move {
            let fetched_investments = fetch_investments().await;

            match fetched_investments {
                Ok(ft) => investments.dispatch(InvestmentAction::Set(ft)),
                Err(e) => alert(&e.to_string()),
            }
        });
    }

    pub fn create_investment(&self, title: String) {
        let investments = self.state.clone();

        spawn_local(async move {
            let response = create_investment(&title).await;

            match response {
                Ok(investment) => investments.dispatch(InvestmentAction::Add(investment)),
                Err(e) => alert(&e.to_string()),
            }
        });
    }

    pub fn toggle_investment(&self, id: String) {
        let investments = self.state.clone();

        spawn_local(async move {
            let response = toggle_investment(id.clone()).await;

            match response {
                Ok(af) if af.rows_affected == 1 => {
                    investments.dispatch(InvestmentAction::Toggle(id.clone()))
                }
                Ok(_) => alert("Did not get a response"),
                Err(e) => alert(&e.to_string()),
            }
        });
    }

    pub fn delete_investment(&self, id: String) {
        let investments = self.state.clone();

        spawn_local(async move {
            let response = delete_investment(id.clone()).await;

            match response {
                Ok(af) if af.rows_affected == 1 => {
                    investments.dispatch(InvestmentAction::Delete(id.clone()))
                }
                Ok(_) => alert("Did not get a response"),
                Err(e) => alert(&e.to_string()),
            }
        });
    }
}
