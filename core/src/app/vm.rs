use data::station::{BuoyStation, BuoyStations, BuoyType};
use app::redux;
use app::{DataState, AppState};
use data::location::Location;
use std::str::FromStr;
use std::num::ParseIntError;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            red: r, 
            green: g,
            blue: b,
        }
    }
}

impl From<i64> for Color {
    fn from(value: i64) -> Self {
        Color {
            red: (((value >> 16) & 0xff) as f64) / 255.0,
            green: (((value >> 8) & 0xff) as f64) / 255.0, 
            blue: ((value & 0xff) as f64) / 255.0,
        }
    }
}

impl From<i32> for Color {
    fn from(value: i32) -> Self {
        Color::from(value as i64)
    }
}

impl FromStr for Color {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, stripped_color) = s.trim().split_at(1);
        let color_int = stripped_color.parse::<i64>()?;
        Ok(Color::from(color_int))
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub enum BuoyStationIcon {
    FixedStation,
    Buoy,
    Tides,
    Tsunami,
    Unknown,
}

#[derive(Clone, Debug)]
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