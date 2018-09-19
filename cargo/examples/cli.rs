extern crate buoyfinder;

use std::sync::{Arc, Mutex};
use buoyfinder::redux;
use buoyfinder::app;
use buoyfinder::vm;

struct ExampleExploreView;

impl vm::ExploreView for ExampleExploreView {
    fn new_view_data(&mut self, view_data: &vm::ExploreViewData) {
        match view_data.is_loading {
            true => println!("Example Explore View got new state: loading stations"),
            _ => println!("Example Explore View got new state: {} stations", view_data.stations.len()),
        };
    }
}

fn main() {
    let app_reducer = Box::new(app::AppReducer{});
    let mut store = redux::Store::create(&app::AppState::default(), app_reducer);
    let explore_view = Box::new(ExampleExploreView{});
    let explore_vm = Arc::new(Mutex::new(vm::ExploreViewModel::new(explore_view)));
    store.subscribe(explore_vm.clone());
    
    store.dispatch(app::Actions::SetBuoyStationsLoading);
    let stations = app::fetch_buoy_stations_remote();
    store.dispatch(app::Actions::SetBuoyStations(stations));
    store.unsubscribe(explore_vm);
}