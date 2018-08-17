extern crate controller;

use controller::redux;
use std::cell::RefCell;
use std::sync::Arc;
use std::fmt::Debug;

pub enum Action {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub count: i64,
}

impl State {
    pub fn new() -> State {
        State {
            count: 0
        }
    }
}

pub fn reducer(state: &State, action: &Action) -> State {
    let mut state = state.clone();
    match action {
        Action::Increment => state.count += 1,
        Action::Decrement => state.count -= 1,
    }
    state
}

pub struct StatePrinter;

impl redux::StoreObserver<State> for StatePrinter {
    fn new_state(&mut self, state: &State) {
        println!("Got state: {}", state.count);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_redux() {
        let state = State::new();

        let observer = StatePrinter{};
        let observer = RefCell::new(observer);
        let observer = Arc::new(observer);

        println!("{}", Arc::strong_count(&observer));

        let mut store = redux::Store::create(&state, reducer);
        store.subscribe(observer.clone());

        println!("{}", Arc::strong_count(&observer));

        store.dispatch(&Action::Increment); // 1
        store.dispatch(&Action::Increment); // 2
        store.dispatch(&Action::Decrement); // 1

        store.unsubscribe(observer.clone());

        println!("{}", Arc::strong_count(&observer));

        store.dispatch(&Action::Decrement); // 0

    }
}