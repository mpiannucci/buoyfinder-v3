//
//  ExploreViewData.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 8/28/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import Foundation

class ExploreViewData {
    
    private let raw: OpaquePointer
    
    init() {
        raw = explore_view_data_new()
    }
    
    init(from raw: OpaquePointer) {
        self.raw = raw
    }
    
    deinit {
        explore_view_data_free(raw)
    }

    var isLoading: Bool {
        get {
            return explore_view_data_is_loading(raw)
        }
    }
    
    var stationCount: Int {
        get {
            return Int(explore_view_data_station_count(raw))
        }
    }
    
    func station(at index: Int) -> BuoyStationItemViewData {
        let rawStation = explore_view_data_station_index(raw, Int64(index))
        return BuoyStationItemViewData(ptr: rawStation!)
    }
}
