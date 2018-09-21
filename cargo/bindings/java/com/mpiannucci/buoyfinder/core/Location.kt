package com.mpiannucci.buoyfinder.core

class Location {

    private val ptr: Long

    constructor(lat: Double, lon: Double, name: String) {
        ptr = new(lat, lon, name)
    }

    constructor(rawPtr: Long) {
        ptr = rawPtr
    }

    fun finalize() {
        free(ptr)
    }

    val latitude: Double
        get() = latitude(ptr)

    val longitude: Double
        get() = longitude(ptr)

    val altitude: Double
        get() = altitude(ptr)

    val name: String
        get() = name(ptr)

    private external fun new(lat: Double, lon: Double, name: String): Long
    private external fun free(ptr: Long)
    private external fun latitude(ptr: Long): Double
    private external fun longitude(ptr: Long): Double
    private external fun altitude(ptr: Long): Double
    private external fun name(ptr: Long): String
}