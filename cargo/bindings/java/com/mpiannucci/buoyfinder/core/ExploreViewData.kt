package com.mpiannucci.buoyfinder.core

class ExploreViewData {

    val ptr: Long

    constructor(rawPtr: Long) {
        ptr = rawPtr
    }

    fun finalize() {
        free(ptr)
    }

    val isLoading: Boolean
        get() = isLoading(ptr)

    val stationCount: Long
        get() = stationCount(ptr)

    fun station(index: Long): BuoyStationItemViewData {
        val rawStation = stationAtIndex(ptr, index)
        return BuoyStationItemViewData(rawStation)
    }

    private external fun new(): Long
    private external fun free(ptr: Long)
    private external fun isLoading(ptr: Long): Boolean
    private external fun stationCount(ptr: Long): Long
    private external fun stationAtIndex(ptr: Long, index: Long): Long
}