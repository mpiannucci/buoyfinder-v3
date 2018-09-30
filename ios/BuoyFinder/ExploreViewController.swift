//
//  ExploreViewController.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 9/29/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import UIKit
import GoogleMaps

class ExploreViewController: UIViewController {
    
    @IBOutlet var mapView: GMSMapView!
    
    private var exploreViewHandle: ExploreViewHandle!
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        // Attach the view to the store
        exploreViewHandle = ExploreViewHandle(store: store, view: self)
        store.fetchBuoyStations()
    }
}

extension ExploreViewController: ExploreView {
    func newViewData(viewData: ExploreViewData) {
        guard !viewData.isLoading else {
            return
        }
        
        // TODO: Update the map view with all of the new buoys
        DispatchQueue.main.async {
            for stationIndex in 0..<viewData.stationCount {
                let station = viewData.station(at: stationIndex)
                let marker = GMSMarker()
                marker.icon = station.mapIcon
                marker.position = station.location.coreLocation
                marker.title = station.title
                marker.snippet = station.subtitle
                marker.map = self.mapView
            }
        }
    }
}

extension ExploreViewController: GMSMapViewDelegate {
    func mapView(_ mapView: GMSMapView, didTap marker: GMSMarker) -> Bool {
        return false
    }
    
    func mapView(_ mapView: GMSMapView, didTapInfoWindowOf marker: GMSMarker) {
        // TODO: Handle marker being tapped
        //delegate?.onBuoySelected(data)
    }
    
    func mapView(_ mapView: GMSMapView, didChange position: GMSCameraPosition) {
        // Handle camera position change
    }
}
