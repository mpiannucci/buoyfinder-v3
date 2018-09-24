use app::state::data_state::DataState;
use data::station::BuoyStations;

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