use station;
use reqwest;
use redux::Reducer;

#[derive(Clone)]
pub enum Actions {
    SetBuoyStations(station::BuoyStations),
}

#[derive(Clone, Debug)]
pub enum DataState<T> {
    NoData,
    DataLoading,
    DataLoaded(T),
}

#[derive(Clone, Debug)]
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

pub struct AppReducer;

impl Reducer<AppState, Actions> for AppReducer {
    fn reduce(&self, state: AppState, action: Actions) -> AppState {
        // let mut new_state = state.clone();

        // match action {
        //     Actions::SetBuoyStations(stations) => new_state.stations_state = DataState::DataLoaded(stations),
        // };

        // new_state
        state
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