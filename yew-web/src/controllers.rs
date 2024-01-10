use gloo_dialogs::alert;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use wasm_bindgen_futures::spawn_local;
use yew::UseReducerHandle;

use crate::{inv_api::*, state::*};
use types::*;

pub struct InvestmentController {
    state: UseReducerHandle<InvestmentState>,
}

#[derive(Serialize, Deserialize)]
pub struct TempThing {
    pub tb: String,
    pub id: Id,
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
            // this is a workaround for the fact that we can't serialize a Thing
            // directly. We need to serialize a struct that contains the Thing's
            // table name and id and then insert that into the Investment object
            // and then serialize the Investment object.
            // TODO: update surrealdb to allow serialization of a Thing after diguring out dependency issues
            let temp_id = match inv.id.clone() {
                Some(id) => serialize_thing(id),
                None => TempThing {
                    tb: "".to_string(),
                    id: Id::Number(0),
                },
            };
            let temp_id = serde_json::json!(temp_id);
            let mut inv = serde_json::json!(inv);

            if let Some(obj1) = inv.as_object_mut() {
                obj1.insert("id".to_string(), temp_id.clone());
            }
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
            // temp_id is a workaround for the fact that we can't serialize a Thing
            // directly. We need to serialize a struct that contains the Thing's
            // table name and id.
            // TODO: update surrealdb to allow serialization of a Thing after diguring out dependency issues
            let temp_id = serialize_thing(id.clone());
            let json_id = serde_json::json!(temp_id);
            let response = delete_investment(json_id.to_string()).await;

            match response {
                Ok(af) if af.id == id => investments.dispatch(InvestmentAction::Delete(id.clone())),
                Ok(_) => alert("Did not get a response"),
                Err(e) => alert(&e.to_string()),
            }
        });
    }
}

fn serialize_thing(thing: Thing) -> TempThing {
    TempThing {
        tb: thing.tb,
        id: thing.id,
    }
}
