use crate::data::station::{BuoyStation, BuoyStations, BuoyType};
use crate::app::redux;
use crate::app::{DataState, AppState};
use crate::data::location::Location;
use crate::app::color::Color;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum BuoyStationIcon {
    FixedStation,
    Buoy,
    Tides,
    Tsunami,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuoyStationItemViewData {
    pub title: String,
    pub subtitle: String,
    pub location: Location,
    pub on_click_id: String,
    pub icon: BuoyStationIcon,
    pub color: Color,
}

impl BuoyStationItemViewData {
    pub fn from_buoy_station(buoy: &BuoyStation) -> BuoyStationItemViewData {
        BuoyStationItemViewData {
            title: buoy.name(),
            subtitle: format!("{} • {} • {}", buoy.station_id, buoy.program, buoy.owner),
            location: buoy.location.clone(),
            on_click_id: buoy.station_id.clone(),
            icon: match buoy.buoy_type { 
                BuoyType::Fixed | BuoyType::OilRig => BuoyStationIcon::FixedStation,
                BuoyType::Buoy => BuoyStationIcon::Buoy,
                BuoyType::Dart => BuoyStationIcon::Tsunami,
                _ => BuoyStationIcon::Unknown,
            },
            color: match buoy.is_active() {
                true => Color { red: 0.0, green: 1.0, blue: 0.0 },
                false => Color { red: 1.0, green: 0.0, blue: 0.0 },
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    view: Box<ExploreView>,
    last_state: ExploreViewData,
}

impl ExploreViewModel {
    pub fn new(explore_view: Box<ExploreView>) -> ExploreViewModel {
        ExploreViewModel {
            view: explore_view,
            last_state: ExploreViewData::from_state(&DataState::NoData),
        }
    }
}

impl redux::StoreObserver<AppState> for ExploreViewModel {
    fn new_state(&mut self, new_state: &AppState) {
        let new_view_data = ExploreViewData::from_state(&new_state.stations_state);
        if new_view_data != self.last_state {
            self.view.new_view_data(&new_view_data);
        }
    }
}