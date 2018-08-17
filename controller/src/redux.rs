use std::clone::Clone;
use std::sync::Arc;
use std::cell::RefCell;

pub trait StoreObserver<T> {
    fn new_state(&mut self, state: &T);
}

type Reducer<T, U> = fn(&T, &U) -> T;

pub struct Store<T: Clone, U> {
    state: T,
    observers: Vec<Arc<RefCell<StoreObserver<T>>>>,
    reducer: Reducer<T, U>,
}

impl <T: Clone, U> Store<T, U> {

    pub fn create(initial_state: &T, reducer: Reducer<T, U>) -> Store<T, U> {
        Store {
            state: initial_state.clone(),
            observers: Vec::new(),
            reducer: reducer,
        }
    }

    pub fn subscribe(&mut self, new_observer: Arc<RefCell<StoreObserver<T>>>) {
        self.observers.push(new_observer);
    }

    pub fn unsubscribe(&mut self, observer: Arc<RefCell<StoreObserver<T>>>) {
        if let Some(position) = self.observers.iter().position(|x| {
            Arc::ptr_eq(&x, &observer)
        }) {
            self.observers.remove(position);
        }
    }

    pub fn dispatch(&mut self, action: &U) {
        self.state = (self.reducer)(&self.state, action);
        self.notify_observers();
    }

    fn notify_observers(&mut self) {
        for mut observer in self.observers.iter_mut() {
            observer.borrow_mut().new_state(&self.state);
        }
    }
}