use gloo_dialogs::alert;
use surrealdb::sql::Thing;
use wasm_bindgen_futures::spawn_local;
use yew::UseReducerHandle;

use crate::{inv_api::*, state::*};
use types::*;

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

    pub fn create_investment(&self, inv: Investment) {
        let investments = self.state.clone();

        spawn_local(async move {
            let inv = serde_json::json!(inv);
            let response = create_investment(inv.to_string()).await;

            match response {
                Ok(investment) => investments.dispatch(InvestmentAction::Add(investment)),
                Err(e) => alert(&e.to_string()),
            }
        });
    }

    pub fn edit_investment(&self, inv: Investment) {
        let investments = self.state.clone();

        spawn_local(async move {
            let inv = serde_json::json!(inv);
            let response = edit_investment(inv.to_string()).await;

            match response {
                Ok(investment) => investments.dispatch(InvestmentAction::Edit(investment)),
                Err(e) => alert(&e.to_string()),
            }
        });
    }

    pub fn delete_investment(&self, id: Thing) {
        let investments = self.state.clone();

        spawn_local(async move {
            let json_id = serde_json::json!(id.clone());
            let response = delete_investment(json_id.to_string()).await;

            match response {
                Ok(af) if af.id == id => investments.dispatch(InvestmentAction::Delete(id.clone())),
                Ok(_) => alert("Did not get a response"),
                Err(e) => alert(&e.to_string()),
            }
        });
    }
}
