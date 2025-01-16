use crate::{
    ext::IMapExt,
    id::Id,
    models::{CustomStrategy, Duel},
};
use gloo::storage::{LocalStorage, Storage};
use implicit_clone::unsync::{IArray, IMap};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

const STATE_KEY: &str = "pdstate_v1";

pub type StateContext = UseReducerHandle<State>;
pub type StateContextProvider = ContextProvider<StateContext>;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct State {
    pub duels: IMap<Id, Duel>,
    pub custom_strategies: IArray<CustomStrategy>,
}

impl State {
    fn new() -> Self {
        LocalStorage::get(STATE_KEY)
            .ok()
            .unwrap_or_else(Self::default)
    }
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)] // remove when more actions
#[allow(clippy::large_enum_variant)] // TODO: is this an issue?
pub enum Action {
    DuelNew(Id, Duel),
    DuelNext(Id),
    DuelClose(Id),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::DuelNew(id, duel) => Rc::new(Self {
                duels: self.duels.clone().insert(id, duel),
                custom_strategies: self.custom_strategies.clone(),
            }),
            Action::DuelNext(id) => Rc::new(Self {
                duels: self.duels.clone().transform(&id, |d| d.next()),
                custom_strategies: self.custom_strategies.clone(),
            }),
            Action::DuelClose(id) => Rc::new(Self {
                duels: self.duels.clone().remove(&id),
                custom_strategies: self.custom_strategies.clone(),
            }),
        }
    }
}

#[hook]
pub fn use_state_new() -> StateContext {
    let state_ctx = use_reducer(State::new);

    use_effect_with(state_ctx.clone(), |state_ctx| {
        let state: &State = state_ctx;
        LocalStorage::set(STATE_KEY, state).ok();
    });

    state_ctx
}

#[hook]
pub fn use_state_context() -> StateContext {
    use_context::<StateContext>().unwrap()
}
