extern crate buoyfinder_core;

use buoyfinder_core::app::redux::*;
use buoyfinder_core::app::vm::*;
use buoyfinder_core::app::state::app_state::*;
use buoyfinder_core::app::actions::*;
use buoyfinder_core::data::station;

use reqwest;

struct ExampleExploreView;

impl ExploreView for ExampleExploreView {
    fn new_view_data(&mut self, view_data: &ExploreViewData) {
        match view_data.is_loading {
            true => println!("Example Explore View got new state: loading stations"),
            _ => println!("Example Explore View got new state: {} stations", view_data.stations.len()),
        };
    }
}

fn main() {
    let mut store = Store::create(AppState::default());
    let explore_view = Box::new(ExampleExploreView{});
    let explore_vm = Box::new(ExploreViewModel::new(explore_view));
    let explore_vm_id = store.subscribe(explore_vm);
    
    store.dispatch(Actions::SetBuoyStationsLoading);

    let res = reqwest::blocking::get(&station::BuoyStations::active_stations_url()).unwrap();
    let raw_data = res.text().unwrap();
    store.dispatch(Actions::SetBuoyStations(station::BuoyStations::from_raw_data(raw_data.as_str())));
    
    store.unsubscribe(explore_vm_id);
}
