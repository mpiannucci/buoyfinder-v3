package com.mpiannucci.buoyfinder.core

class ExploreViewData {

    val ptr: Long

    constructor() {
        ptr = new()
    }

    constructor(rawPtr: Long) {
        ptr = rawPtr
    }

    fun finalize() {
        free(ptr)
    }

    val stationCount: Long
        get() = stationCount(ptr)

    fun station(index: Long): BuoyStation {
        val rawStation = stationAtIndex(ptr, index)
        return BuoyStation(rawStation)
    }

    private external fun new(): Long
    private external fun free(ptr: Long)
    private external fun stationCount(ptr: Long): Long
    private external fun stationAtIndex(ptr: Long, index: Long): Long
}