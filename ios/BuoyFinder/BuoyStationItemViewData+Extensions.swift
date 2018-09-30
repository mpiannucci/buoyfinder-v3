//
//  BuoyStationItemViewData+Extensions.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 9/29/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import Foundation
import GoogleMaps

let redMarkerIcon = GMSMarker.markerImage(with: .red)
let blueMarkerIcon = GMSMarker.markerImage(with: .blue)
let greenMarkerIcon = GMSMarker.markerImage(with: .green)

extension BuoyStationItemViewData {
    public var mapIcon: UIImage {
        get {
            if color.red > color.green {
                return redMarkerIcon
            } else {
                return greenMarkerIcon
            }
        }
    }
}
