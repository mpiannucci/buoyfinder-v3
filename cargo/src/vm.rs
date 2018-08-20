use station;
use redux;
use app::{DataState, AppState};

#[derive(Clone)]
pub struct ExploreViewData {
    pub stations: Vec<station::BuoyStation>
}

impl ExploreViewData {
    pub fn from_state(state: &DataState<station::BuoyStations>) -> ExploreViewData {
        match state {
            DataState::DataLoaded(stations) => ExploreViewData{stations: stations.stations.clone()},
            _ => ExploreViewData{stations: vec![]}
        }
    }
}

pub trait ExploreView {
    fn new_view_data(&mut self, view_data: &ExploreViewData);
}

pub struct ExploreViewModel {
    pub view: Box<ExploreView>,
}

impl ExploreViewModel {
    pub fn new(explore_view: Box<ExploreView>) -> ExploreViewModel {
        ExploreViewModel {
            view: explore_view,
        }
    }
}

impl redux::StoreObserver<AppState> for ExploreViewModel {
    fn new_state(&mut self, new_state: &AppState) {
        let new_view_data = ExploreViewData::from_state(&new_state.stations_state);
        self.view.new_view_data(&new_view_data);
    }
}