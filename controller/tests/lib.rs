extern crate controller;

use controller::redux;

pub enum Action {
    Increment,
    Decrement,
}

#[derive(Clone, PartialEq)]
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

impl <T: State> redux::StoreObserver<T> for StatePrinter {
    fn new_state(&mut self, state: &T) {
        println!("{}", state.count);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_redux() {
        let state = State::new();
        let store = redux::Store::create(&state, reducer);
    }
}