//
//  Store.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 8/28/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import Foundation

class Store {
    
    public let raw: OpaquePointer
    
    public init() {
        raw = store_new()
    }
    
    deinit {
        store_free(raw)
    }
    
    private var fetch_buoy_stations_url: String{
        get {
            return String(cString: store_fetch_buoy_stations_url())
        }
    }
}
