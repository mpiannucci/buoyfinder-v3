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
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view, typically from a nib.
        
        let station = BuoyStation(stationId_: "44097", stationName: "Block Island, RI", latitude: 41.0, longitude: -71.0)
        stationLabel.text = station.name
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }


}

