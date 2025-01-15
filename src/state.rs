use crate::models::{CustomStrategy, Duel};
use implicit_clone::unsync::IArray;
use std::rc::Rc;
use yew::prelude::*;

pub type StateContext = UseReducerHandle<State>;
pub type StateContextProvider = ContextProvider<StateContext>;

#[derive(Debug, PartialEq)]
pub struct State {
    pub duel: Option<Duel>,
    pub custom_strategies: IArray<CustomStrategy>,
}

#[allow(clippy::derivable_impls)]
impl Default for State {
    fn default() -> Self {
        Self {
            duel: None,
            custom_strategies: IArray::default(),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Duel(Duel),
    DuelTurn,
    DuelClose,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Duel(duel) => Rc::new(Self {
                duel: Some(duel),
                custom_strategies: self.custom_strategies.clone(),
            }),
            Action::DuelTurn => Rc::new(Self {
                duel: self.duel.as_ref().cloned().map(|d| d.next()),
                custom_strategies: self.custom_strategies.clone(),
            }),
            Action::DuelClose => Rc::new(Self {
                duel: None,
                custom_strategies: self.custom_strategies.clone(),
            }),
        }
    }
}

#[hook]
pub fn use_state_context() -> StateContext {
    use_context::<StateContext>().unwrap()
}
