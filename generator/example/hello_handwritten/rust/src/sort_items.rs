use crate::hello;

pub struct SortItems {
    listener: Box<hello::ItemsListener>,
}

impl hello::SortItemsInterface for SortItems {
    fn new(listener: Box<hello::ItemsListener>) -> Self {
        SortItems {
            listener: listener,
        }
    }

    fn sort(&mut self, order: &hello::SortOrder, items: &hello::ItemList) {
        // TODO: Actually sort
        self.listener.update(items)
    }

    fn listener_count(&self) -> i32 {
        1
    }
}