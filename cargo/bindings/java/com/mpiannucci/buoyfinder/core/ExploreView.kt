package com.mpiannucci.buoyfinder.core

interface ExploreView {
    fun newViewData(viewData: ExploreViewData)
}

class ExploreViewHandle(val view: ExploreView, store: Store) {

    private val storePtr: Long
    private val ptr: Long

    init {
        storePtr = store.rawStore
        ptr = bind(view, storePtr)
    }

    fun finalize() {
        unbind(ptr, storePtr)
    }

    private external fun bind(view: ExploreView, store_ptr: Long): Long
    private external fun unbind(ptr: Long, store_ptr: Long)
}