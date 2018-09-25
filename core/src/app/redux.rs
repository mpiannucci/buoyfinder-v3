use std::clone::Clone;

pub trait StoreObserver<T> {
    fn new_state(&mut self, state: &T);
}

pub trait Reducer<T, U> {
    fn reduce(&self, state: &T, action: &U) -> T;
}

pub struct Store<T, U> {
    pub state: T,
    observers: Vec<Box<StoreObserver<T>>>,
    observer_ids: Vec<i32>,
    observer_id_factory: i32,
    reducer: Box<Reducer<T, U>>,
}

impl <T, U> Store<T, U> where T: Clone {

    pub fn create(initial_state: &T, reducer: Box<Reducer<T, U>>) -> Store<T, U> {
        Store {
            state: initial_state.clone(),
            observers: Vec::new(),
            observer_ids: Vec::new(),
            observer_id_factory: 0,
            reducer: reducer,

        }
    }

    pub fn subscribe(&mut self, mut new_observer: Box<StoreObserver<T>>) -> i32 {
        new_observer.new_state(&self.state);
        self.observers.push(new_observer);
        let observer_id = self.observer_id_factory;
        self.observer_ids.push(observer_id);
        self.observer_id_factory += 1;
        observer_id
    }

    pub fn unsubscribe(&mut self, observer_id: i32) {
        if let Some(position) = self.observer_ids.iter().position(|x| x == &observer_id) {
            self.observers.remove(position);
        }
    }

    pub fn dispatch(&mut self, action: U) {
        self.state = self.reducer.reduce(&self.state, &action);
        self.notify_observers();
    }

    fn notify_observers(&mut self) {
        for mut observer in self.observers.iter_mut() {
            observer.new_state(&self.state);
        }
    }
}