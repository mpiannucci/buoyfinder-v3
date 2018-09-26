package com.mpiannucci.buoyfinder.core

interface ExploreView {
    fun newViewData(viewData: ExploreViewData)
}

class ExploreViewHandle(val view: ExploreView, store: Store) {

    private val storePtr: Long
    private val view_id: Int

    init {
        storePtr = store.rawStore
        view_id = bind(view, storePtr)
    }

    fun finalize() {
        unbind(view_id, storePtr)
    }

    private external fun bind(view: ExploreView, store_ptr: Long): Int
    private external fun unbind(view_id: Int, store_ptr: Long)
}