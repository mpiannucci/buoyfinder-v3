package com.mpiannucci.buoyfinder.core

class Store {

    private val ptr: Long

    init {
        ptr = new()
    }

    fun finalize() {
        free(ptr)
    }

    val rawStore: Long
        get() = ptr

    external fun fetchBuoyStations()

    private external fun new(): Long
    private external fun free(ptr: Long)
}