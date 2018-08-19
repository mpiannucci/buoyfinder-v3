use station;
use redux;
use std::sync::Weak;
use std::sync::Arc;
use std::cell::RefCell;
use app;

#[derive(Clone)]
pub struct ExploreViewData {
    pub stations: Vec<station::BuoyStation>
}

impl ExploreViewData {
    pub fn from_state(state: &app::DataState<station::BuoyStations>) -> ExploreViewData {
        match state {
            app::DataState::DataLoaded(stations) => ExploreViewData{stations: stations.stations.clone()},
            _ => ExploreViewData{stations: vec![]}
        }
    }
}

pub trait ExploreView {
    fn new_view_data(&mut self, view_data: &ExploreViewData);
}

pub struct ExploreViewModel {
    pub view: Arc<RefCell<ExploreView>>,
}

impl ExploreViewModel {
    pub fn new(explore_view: Arc<RefCell<ExploreView>>) -> ExploreViewModel {
        ExploreViewModel {
            view: explore_view,
        }
    }
}

impl redux::StoreObserver<app::AppState> for ExploreViewModel {
    fn new_state(&mut self, new_state: &app::AppState) {
        let new_view_data = ExploreViewData::from_state(&new_state.stations_state);
        self.view.borrow_mut().new_view_data(&new_view_data);
    }
}