//
//  BuoyStation.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 8/28/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import Foundation

class BuoyStation {
    
    private let raw: OpaquePointer
    
    init(_ raw: OpaquePointer) {
        self.raw = raw
    }
    
    init(stationId_: String, stationName: String, latitude: Double, longitude: Double) {
        raw = buoy_station_new(stationId_, stationName, latitude, longitude)
    }
    
    deinit {
        buoy_station_free(raw)
    }
    
    var stationId: String {
        get {
            return String(cString: buoy_station_id(raw))
        }
    }
    
    var name: String {
        get {
            return String(cString: buoy_station_name(raw))
        }
    }
    
    var isActive: Bool {
        get {
            return buoy_station_active(raw)
        }
    }
    
    
}
