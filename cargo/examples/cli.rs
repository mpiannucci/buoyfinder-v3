extern crate buoyfinder;

use std::sync::{Arc, Mutex};
use buoyfinder::redux;
use buoyfinder::app;
use buoyfinder::vm;

struct ExampleExploreView;

impl vm::ExploreView for ExampleExploreView {
    fn new_view_data(&mut self, view_data: &vm::ExploreViewData) {
        println!("Example Explore View got new state: {} stations", view_data.stations.len());
    }
}

fn main() {
    let mut store = redux::Store::create(&app::AppState::default(), app::app_reducer);
    let explore_view = Box::new(ExampleExploreView{});
    let explore_vm = Arc::new(Mutex::new(vm::ExploreViewModel::new(explore_view)));
    store.subscribe(explore_vm.clone());
    
    let stations = app::fetch_buoy_stations_remote();
    store.dispatch(&app::Actions::SetBuoyStations(stations));
    store.unsubscribe(explore_vm);
}