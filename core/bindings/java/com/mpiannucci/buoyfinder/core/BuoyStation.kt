package com.mpiannucci.buoyfinder.core

class BuoyStation {

    private val ptr: Long

    constructor(stationId: String, name: String, lat: Double, lon: Double) {
        ptr = new(stationId, name, lat, lon)
    }

    constructor(rawPtr: Long) {
        ptr = rawPtr
    }

    fun finalize() {
        free(ptr)
    }

    public val stationId: String
        get() = stationId(ptr)

    public val name: String
        get() = name(ptr)

    private external fun new(stationId: String, name: String, lat: Double, lon: Double): Long
    private external fun free(ptr: Long)
    private external fun stationId(ptr: Long): String
    private external fun name(ptr: Long): String
}