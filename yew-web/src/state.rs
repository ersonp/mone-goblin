use std::collections::VecDeque;
use std::rc::Rc;

use surrealdb::sql::Thing;
use yew::Reducible;

use types::Investment;

/// reducer's Action
pub enum InvestmentAction {
    Set(VecDeque<Investment>),
    Add(Investment),
    Edit(Investment),
    Delete(Thing),
}

/// reducer's State
pub struct InvestmentState {
    pub investments: VecDeque<Investment>,
}

/// Implementation by default when starting the application
impl Default for InvestmentState {
    fn default() -> Self {
        Self {
            investments: VecDeque::from([]),
        }
    }
}

/// Implementation of Reducible (required for the reducer)
impl Reducible for InvestmentState {
    /// Reducer Action Type
    type Action = InvestmentAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_investments = match action {
            InvestmentAction::Set(investments) => investments,
            InvestmentAction::Add(investment) => {
                let mut investments = self.investments.clone();
                investments.push_front(investment);
                investments
            }
            InvestmentAction::Edit(edited_inv) => {
                let mut investments = self.investments.clone();
                if let Some(investment) = investments
                    .iter_mut()
                    .find(|investment| investment.id == edited_inv.id)
                {
                    *investment = edited_inv.clone();
                }
                investments
            }
            InvestmentAction::Delete(id) => {
                let mut investments = self.investments.clone();
                investments.retain(|investment| investment.id != Some(id.clone()));
                investments
            }
        };

        Self {
            investments: next_investments,
        }
        .into()
    }
}
