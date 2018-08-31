package com.mpiannucci.buoyfinder.core

class Store {

    private val ptr: Long

    init {
        ptr = new()
    }

    fun finalize() {
        free(ptr)
    }

    public external fun fetch_buoy_stations()

    private external fun new(): Long
    private external fun free(ptr: Long)
}