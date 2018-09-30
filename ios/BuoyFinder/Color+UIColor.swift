//
//  Color+UIColor.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 9/30/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import UIKit

extension Color {
    public var uiColor: UIColor {
        get {
            return UIColor(red: CGFloat(red), green: CGFloat(green), blue: CGFloat(blue), alpha: 1.0)
        }
    }
}
