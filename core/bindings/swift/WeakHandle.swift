//
//  File.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 8/29/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import Foundation

class WeakHandle<T: AnyObject> {
    
    weak var object: T?
    
    init(_ obj: T) {
        object = obj
    }
}
