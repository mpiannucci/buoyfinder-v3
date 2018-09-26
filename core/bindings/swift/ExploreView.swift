//
//  ExploreView.swift
//  BuoyFinder
//
//  Created by Matthew Iannucci on 8/29/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//

import Foundation

protocol ExploreView: class {
    func newViewData(viewData: ExploreViewData)
}

class ExploreViewHandle {
    
    weak var view: ExploreView?
    
    private var view_id: Int32 = 0
    private weak var store: Store?
    
    init(store: Store?, view: ExploreView?) {
        self.store = store
        self.view = view
        
        let weakSelf = UnsafeMutableRawPointer(Unmanaged.passRetained(WeakHandle(self)).toOpaque())
        let raw_view = explore_view(view: weakSelf, new_view_data: handleNewExploreViewData)
        view_id = explore_view_bind(raw_view, store?.raw)
    }
    
    deinit {
        explore_view_unbind(view_id, store?.raw ?? nil)
    }
}

private func handleNewExploreViewData(ptr: UnsafeMutableRawPointer?, viewData: OpaquePointer?) {
    autoreleasepool {
        guard let ptr = ptr else {
            return
        }
        let handle = Unmanaged<WeakHandle<ExploreViewHandle>>.fromOpaque(ptr).takeUnretainedValue()
        DispatchQueue.main.async {
            guard let exploreViewHandle = handle.object, let viewData = viewData else {
                return
            }
            
            let newViewData = ExploreViewData(from: viewData)
            exploreViewHandle.view?.newViewData(viewData: newViewData)
        }
    }
}
