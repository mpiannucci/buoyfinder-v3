pub mod redux;
pub mod color;
pub mod actions;
pub mod state;
pub mod vm;

use crate::data::station;
use reqwest;
use crate::app::redux::Reducer;
use crate::app::state::data_state::DataState;
use crate::app::state::app_state::AppState;
use crate::app::actions::Actions;

pub struct AppReducer;

impl Reducer<AppState, Actions> for AppReducer {
    fn reduce(&self, state: &AppState, action: &Actions) -> AppState {
        let mut new_state = state.clone();

        match action {
            Actions::SetBuoyStationsLoading => new_state.stations_state = DataState::DataLoading,
            Actions::SetBuoyStations(stations) => new_state.stations_state = DataState::DataLoaded(stations.clone()),
            Actions::SetBuoyStationLoadError => new_state.stations_state = DataState::DataError,
        };

        new_state
    }
}

pub fn fetch_buoy_stations_remote() -> station::BuoyStations {
    let mut res = reqwest::get("https://ndbc.noaa.gov/activestations.xml").unwrap();
    station::BuoyStations::from_raw_data(res.text().unwrap().as_ref())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fetch_buoys_remote() {
        let stations = fetch_buoy_stations_remote();
        assert!(stations.station_count > 0);
    }
}