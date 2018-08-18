use station;

#[derive(Clone)]
pub enum Actions {
    SetBuoyStations(station::BuoyStations),
}

#[derive(Clone)]
pub enum DataState<T> {
    NoData,
    DataLoading,
    DataLoaded(T),
}

#[derive(Clone)]
pub struct AppState {
    pub stations_state: DataState<station::BuoyStations>,
}

impl AppState {
    pub fn default() -> AppState {
        AppState {
            stations_state: DataState::NoData,
        }
    }
}

pub fn app_reducer(state: &AppState, action: &Actions) -> AppState {
    let mut state = state.clone();

    // TODO: Handle everything

    state
}