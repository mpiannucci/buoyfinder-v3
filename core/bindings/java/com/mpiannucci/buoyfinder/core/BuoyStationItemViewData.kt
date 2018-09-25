package com.mpiannucci.buoyfinder.core

class BuoyStationItemViewData {

    val ptr: Long

    constructor(rawPtr: Long) {
        ptr = rawPtr
    }

    fun finalize() {
        free(ptr)
    }

    val title: String
        get() = title(ptr)

    val subtitle: String
        get() = subtitle(ptr)

    val onClickId: String
        get() = onClickId(ptr)

    val icon: BuoyStationIcon
        get() = BuoyStationIcon.fromInt(icon(ptr))

    val color: Color
        get() = color(ptr)

    val location: Location 
        get() = Location(location(ptr))

    private external fun free(ptr: Long)
    private external fun title(ptr: Long): String
    private external fun subtitle(ptr: Long): String
    private external fun onClickId(ptr: Long): String
    private external fun icon(ptr: Long): Int
    private external fun color(ptr: Long): Color
    private external fun location(ptr: Long): Long
}