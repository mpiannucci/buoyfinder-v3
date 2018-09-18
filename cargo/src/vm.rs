use station::{BuoyStation, BuoyStations};
use redux;
use app::{DataState, AppState};

pub struct BuoyStationItem {
    title: String,
    subtitle: String
}

#[derive(Clone, Debug)]
pub struct ExploreViewData {
    pub is_loading: bool,
    pub stations: Vec<BuoyStation>,
}

impl ExploreViewData {
    pub fn from_state(state: &DataState<BuoyStations>) -> ExploreViewData {
        match state {
            DataState::DataLoading => ExploreViewData{is_loading: true, stations: vec![]},
            DataState::DataLoaded(stations) => ExploreViewData{
                    is_loading: false,
                    stations: stations.stations.clone(),
                },
            _ => ExploreViewData{is_loading: false, stations: vec![]}
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