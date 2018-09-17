use std::clone::Clone;
use std::sync::Arc;
use std::sync::Mutex;

pub trait StoreObserver<T> {
    fn new_state(&mut self, state: &T);
}

pub trait Reducer<T, U> {
    fn reduce(&self, state: T, action: U) -> T;
}

pub struct Store<T, U> {
    pub state: T,
    observers: Vec<Arc<Mutex<StoreObserver<T>>>>,
    reducer: Box<Reducer<T, U>>,
}

impl <T, U> Store<T, U> where T: Clone {

    pub fn create(initial_state: &T, reducer: Box<Reducer<T, U>>) -> Store<T, U> {
        Store {
            state: initial_state.clone(),
            observers: Vec::new(),
            reducer: reducer,
        }
    }

    pub fn subscribe(&mut self, new_observer: Arc<Mutex<StoreObserver<T>>>) {
        new_observer.lock().unwrap().new_state(&self.state);
        self.observers.push(new_observer);
    }

    pub fn unsubscribe(&mut self, observer: Arc<Mutex<StoreObserver<T>>>) {
        if let Some(position) = self.observers.iter().position(|x| {
            Arc::ptr_eq(&x, &observer)
        }) {
            self.observers.remove(position);
        }
    }

    pub fn dispatch(&mut self, action: U) {
        self.state = self.reducer.reduce(self.state.clone(), action);
        self.notify_observers();
    }

    fn notify_observers(&mut self) {
        for mut observer in self.observers.iter_mut() {
            observer.lock().unwrap().new_state(&self.state);
        }
    }
}