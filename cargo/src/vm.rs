use station::{BuoyStation, BuoyStations};
use redux;
use app::{DataState, AppState};
use location::Location;
use palette::Color;

#[derive(Clone, Debug)]
pub struct BuoyStationItemViewData {
    pub title: String,
    pub subtitle: String,
    pub location: Location,
    pub on_click_id: String,
}

impl BuoyStationItemViewData {
    pub fn from_buoy_station(buoy: &BuoyStation) -> BuoyStationItemViewData {
        BuoyStationItemViewData {
            title: buoy.name(),
            subtitle: format!("{} • {} • {}", buoy.station_id, buoy.program, buoy.owner),
            location: buoy.location.clone(),
            on_click_id: buoy.station_id.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExploreViewData {
    pub is_loading: bool,
    pub stations: Vec<BuoyStationItemViewData>,
}

impl ExploreViewData {
    pub fn from_state(state: &DataState<BuoyStations>) -> ExploreViewData {
        match state {
            DataState::DataLoading => ExploreViewData{is_loading: true, stations: vec![]},
            DataState::DataLoaded(stations) => ExploreViewData{
                    is_loading: false,
                    stations: stations.stations.iter().map(|b| BuoyStationItemViewData::from_buoy_station(b)).collect(),
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