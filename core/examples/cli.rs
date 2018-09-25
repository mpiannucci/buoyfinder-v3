extern crate buoyfinder;

use buoyfinder::app::redux;
use buoyfinder::app;
use buoyfinder::app::vm;

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
    let mut store = redux::Store::create(&app::state::app_state::AppState::default(), app_reducer);
    let explore_view = Box::new(ExampleExploreView{});
    let explore_vm = Box::new(vm::ExploreViewModel::new(explore_view));
    let explore_vm_id = store.subscribe(explore_vm);
    
    store.dispatch(app::actions::Actions::SetBuoyStationsLoading);
    let stations = app::fetch_buoy_stations_remote();
    store.dispatch(app::actions::Actions::SetBuoyStations(stations));
    store.unsubscribe(explore_vm_id);
}