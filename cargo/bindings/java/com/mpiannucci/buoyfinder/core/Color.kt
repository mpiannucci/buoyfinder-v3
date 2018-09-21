package com.mpiannucci.buoyfinder.core

class Color {

    private val ptr: Long

    constructor(rawPtr: Long) {
        ptr = rawPtr
    }

    fun finalize() {
        free(ptr)
    }

    val red: Double
        get() = red(ptr)

    val green: Double
        get() = green(ptr)

    val blue: Double
        get() = blue(ptr)

    private external fun free(ptr: Long)
    private external fun red(ptr: Long): Double
    private external fun green(ptr: Long): Double
    private external fun blue(ptr: Long): Double
}