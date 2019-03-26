use std::clone::Clone;

pub trait StoreObserver<T> {
    fn new_state(&mut self, state: &T);
}

pub trait Reducer: Default + Clone {
    type Action;
    fn reduce(&mut self, action: Self::Action);
}

pub struct Store<T> where T: Reducer {
    pub state: T,
    observers: Vec<Box<StoreObserver<T>>>,
    observer_ids: Vec<i32>,
    observer_id_factory: i32,
}

impl <T> Store<T> where T: Reducer {

    pub fn create(initial_state: T) -> Store<T> {
        Store {
            state: initial_state,
            observers: Vec::new(),
            observer_ids: Vec::new(),
            observer_id_factory: 0,
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

    pub fn dispatch(&mut self, action: T::Action) {
        self.state.reduce(action);
        self.notify_observers();
    }

    fn notify_observers(&mut self) {
        for mut observer in self.observers.iter_mut() {
            observer.new_state(&self.state);
        }
    }
}