package com.mpiannucci.buoyfinder.core

enum class BuoyStationIcon {
    FixedStation,
    Buoy,
    Tides,
    Tsunami,
    Unknown;

    companion object {
        fun fromInt(intValue: Int): BuoyStationIcon {
            return when (intValue) {
                0 -> FixedStation
                1 -> Buoy
                2 -> Tides
                3 -> Tsunami
                else -> Unknown
            }
        }
    }
}