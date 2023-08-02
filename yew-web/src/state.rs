use std::collections::VecDeque;
use std::rc::Rc;
use types::Investment2;
use yew::Reducible;

/// reducer's Action
pub enum InvestmentAction {
    Set(VecDeque<Investment2>),
    Add(Investment2),
    Edit(String),
    Delete(String),
}

/// reducer's State
pub struct InvestmentState {
    pub investments: VecDeque<Investment2>,
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
            InvestmentAction::Edit(id) => {
                let mut investments = self.investments.clone();
                // not sure how i "fixed" this
                // converted id: String to Option<Thing> with thing(&id).ok()
                let investment = investments
                    .iter_mut()
                    .find(|investment| investment.id == id);
                investments
            }
            InvestmentAction::Delete(id) => {
                let mut investments = self.investments.clone();
                investments.retain(|investment| investment.id != id);
                investments
            }
        };

        Self {
            investments: next_investments,
        }
        .into()
    }
}
