//
//  ViewController.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 8/22/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    @IBOutlet weak var stationLabel: UILabel!
    
    var exploreViewHandle: ExploreViewHandle!
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view, typically from a nib.
        
        stationLabel.text = "No Stations loaded"
        
        // Attach the view to the store
        exploreViewHandle = ExploreViewHandle(store: store, view: self)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}

extension ViewController: ExploreView {
    func newViewData(viewData: ExploreViewData) {
        if viewData.isLoading {
            stationLabel.text = "Loading stations..."
        } else {
            stationLabel.text = "\(viewData.stationCount) stations loaded"
        }
    }
}
