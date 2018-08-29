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
    
    deinit {
        explore_view_data_free(raw)
    }
    
    var stationCount: Int {
        get {
            return Int(explore_view_data_station_count(raw))
        }
    }
    
    func station(at index: Int) -> BuoyStation {
        let rawStation = explore_view_data_station_index(raw, Int64(index))
        return BuoyStation(rawStation!)
    }
    
    var stations: [BuoyStation] {
        get {
            var stations: [BuoyStation] = []
            for i in 0..<stationCount {
                let buoy = station(at: i)
                stations.append(buoy)
            }
            return stations
        }
    }
}
