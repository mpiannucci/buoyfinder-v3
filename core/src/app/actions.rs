use data::station::BuoyStations;

#[derive(Clone)]
pub enum Actions {
    SetBuoyStationsLoading,
    SetBuoyStations(BuoyStations),
    SetBuoyStationLoadError,
}