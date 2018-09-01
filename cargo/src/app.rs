use station;
use reqwest;

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

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            stations_state: DataState::NoData,
        }
    }
}

pub fn app_reducer(state: &AppState, action: &Actions) -> AppState {
    let mut state = state.clone();

    match action {
        Actions::SetBuoyStations(new_stations) => state.stations_state = DataState::DataLoaded(new_stations.clone()),
    }

    state
}

pub fn fetch_buoy_stations_remote() -> station::BuoyStations {
    let mut res = reqwest::get("http://ndbc.noaa.gov/activestations.xml").unwrap();
    println!("Status: {}", res.status());
    // println!("Headers:\n{:?}", res.headers());

    // station::BuoyStations::from_raw_data(res.text().unwrap().as_ref())
    station::BuoyStations::default()
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