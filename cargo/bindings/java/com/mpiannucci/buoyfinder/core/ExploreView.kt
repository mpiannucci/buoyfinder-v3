package com.mpiannucci.buoyfinder.core

interface ExploreView {
    fun newViewData(viewData: ExploreViewData)
}

class ExploreViewHandle(val view: ExploreView, store: Store) {

    private val store_ptr: Long = store.rawStore
    private val ptr = bind(store_ptr)

    fun finalize() {
        unbind(ptr, store_ptr)
    }

    private external fun bind(store_ptr: Long): Long
    private external fun unbind(ptr: Long, store_ptr: Long)
}