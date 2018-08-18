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
    stations_state: DataState<station::BuoyStations>,
}
