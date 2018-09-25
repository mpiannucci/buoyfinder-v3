//
//  BuoyStationItemView.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 9/20/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import Foundation

class BuoyStationItemViewData {
    
    public let raw: OpaquePointer
    
    init(ptr: OpaquePointer) {
        raw = ptr
    }
    
    deinit {
        buoy_station_item_view_data_free(raw)
    }

    public var title: String {
        get {
            return String(cString: buoy_station_item_view_data_title(raw))
        }
    }
    
    public var subtitle: String {
        get {
            return String(cString: buoy_station_item_view_data_subtitle(raw))
        }
    }
    
    public var onClickId: String {
        get {
            return String(cString: buoy_station_item_view_data_on_click_id(raw))
        }
    }
    
    public var icon: BuoyStationIcon {
        get {
            return buoy_station_item_view_data_icon(raw)
        }
    }
    
    public var color: Color {
        get {
            return buoy_station_item_view_data_color(raw)
        }
    }

    public var location: Location {
        get {
            return Location(ptr: buoy_station_item_view_data_location(raw))
        }
    }
}
