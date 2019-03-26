use crate::app::state::data_state::DataState;
use crate::data::station::BuoyStations;
use crate::app::redux::Reducer;
use crate::app::actions::Actions;

#[derive(Clone, Debug)]
pub struct AppState {
    pub stations_state: DataState<BuoyStations>,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            stations_state: DataState::NoData,
        }
    }
}

impl Reducer for AppState {
    type Action = Actions;

    fn reduce(&mut self, action: Self::Action) {
        match action {
            Actions::SetBuoyStationsLoading => self.stations_state = DataState::DataLoading,
            Actions::SetBuoyStations(stations) => self.stations_state = DataState::DataLoaded(stations.clone()),
            Actions::SetBuoyStationLoadError => self.stations_state = DataState::DataError,
        };
    }
}