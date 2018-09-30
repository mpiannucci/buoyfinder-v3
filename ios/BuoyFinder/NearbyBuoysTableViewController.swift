//
//  NearbyBuoysViewController.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 9/30/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import UIKit
import Pulley

class NearbyBuoysTableViewController: UITableViewController {
    
    override func viewDidLoad() {
        super.viewDidLoad()
    }
    
    @IBAction func openFavoritesView(_ sender: Any) {
    }
    
    @IBAction func openSearchView(_ sender: Any) {
    }
    
    @IBAction func openSettingsView(_ sender: Any) {
    }
}

extension NearbyBuoysTableViewController: PulleyDrawerViewControllerDelegate {
    func collapsedDrawerHeight(bottomSafeArea: CGFloat) -> CGFloat {
        return 100.0
    }
    
    func partialRevealDrawerHeight(bottomSafeArea: CGFloat) -> CGFloat {
        return 360.0
    }
    
    func supportedDrawerPositions() -> [PulleyPosition] {
        return PulleyPosition.all
    }
}
