package com.mpiannucci.buoyfinder

import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import com.mpiannucci.buoyfinder.core.*
import kotlinx.android.synthetic.main.activity_main.*

class MainActivity : AppCompatActivity(), ExploreView {

    private lateinit var store: Store
    private lateinit var viewHandle: ExploreViewHandle

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        store = Store()
        viewHandle = ExploreViewHandle(this, store)

        val station = BuoyStation("44097", "Block Island, RI", 41.0, -71.0)
        station_tv.text = station.name
    }

    override fun newViewData(viewData: ExploreViewData) {
        println("Got new view data")
    }

    companion object {
        // Used to load the 'buoyfinder' library on application startup.
        init {
            System.loadLibrary("buoyfinder")
        }
    }
}
