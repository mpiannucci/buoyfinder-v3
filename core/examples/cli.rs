extern crate buoyfinder;

use buoyfinder::app::redux::*;
use buoyfinder::app::http::*;
use buoyfinder::app::vm::*;
use buoyfinder::app::state::app_state::*;
use buoyfinder::app::actions::*;
use buoyfinder::data::station;

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

struct ExampleHttpClient;

impl HttpClient for ExampleHttpClient {
    fn fetch<T>(&mut self, url: &str, completion: &mut T) where T: HttpListener {
        let mut res = reqwest::get(&station::BuoyStations::active_stations_url()).unwrap();
        if res.status().is_success() {
            let raw_data = res.text().unwrap();
            completion.on_completed(HttpResponse{
                error_code: None,
                data: raw_data,
                url: url.to_owned(),
            });
        }
        
    }
}

struct ActionCreator<F> where F: FnMut(HttpResponse) {
    creator: F,
}

impl <F> HttpListener for ActionCreator<F> where F: FnMut(HttpResponse) {
    fn on_completed(&mut self, response: HttpResponse) {
        (self.creator)(response);
    }
}

fn main() {
    let mut store = Store::create(AppState::default());
    let explore_view = Box::new(ExampleExploreView{});
    let explore_vm = Box::new(ExploreViewModel::new(explore_view));
    let explore_vm_id = store.subscribe(explore_vm);
    
    store.dispatch(Actions::SetBuoyStationsLoading);

    let mut http_client = ExampleHttpClient{};
    let mut stations_creator = ActionCreator{
        creator: |response| {
            let stations = station::BuoyStations::from_raw_data(response.data.as_ref());
            store.dispatch(Actions::SetBuoyStations(stations));
        }
    };
    http_client.fetch(&station::BuoyStations::active_stations_url(), &mut stations_creator);
    
    store.unsubscribe(explore_vm_id);
}