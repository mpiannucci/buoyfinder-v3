package com.mpiannucci.buoyfinder

import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import com.mpiannucci.buoyfinder.core.BuoyStation
import kotlinx.android.synthetic.main.activity_main.*

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val station = BuoyStation("44097", "Block Island, RI", 41.0, -71.0)
        station_tv.text = station.name
    }

    companion object {
        // Used to load the 'buoyfinder' library on application startup.
        init {
            System.loadLibrary("buoyfinder")
        }
    }
}
