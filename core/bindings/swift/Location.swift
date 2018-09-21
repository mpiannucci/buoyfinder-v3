//
//  Location.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 9/20/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import Foundation
import CoreLocation

class Location {
    public let raw: OpaquePointer
    
    init(ptr: OpaquePointer) {
        raw = ptr
    }
    
    init(lat: Double, lon: Double, name: String) {
        raw = location_new(lat, lon, name)
    }
    
    convenience init(location: CLLocationCoordinate2D) {
        self.init(lat: location.latitude, lon: location.longitude, name: "")
    }
    
    deinit {
        location_free(raw)
    }
    
    public var latitude: Double {
        get {
            return location_latitude(raw)
        }
    }
    
    public var longitude: Double {
        get {
            return location_longitude(raw)
        }
    }

    public var altitude: Double {
        get {
            return location_altitude(raw)
        }
    }
    
    public var name: String {
        get {
            return String(cString: location_name(raw))
        }
    }
    
    public var coreLocation: CLLocationCoordinate2D {
        get {
            return CLLocationCoordinate2D(latitude: latitude, longitude: longitude)
        }
    }
}
