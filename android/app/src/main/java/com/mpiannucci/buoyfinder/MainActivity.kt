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

        station_tv.text = "No stations loaded"

        store = Store()
        viewHandle = ExploreViewHandle(this, store)

        store.fetchBuoyStations()
    }

    override fun newViewData(viewData: ExploreViewData) {
        station_tv.text = viewData.stationCount.toString() + " stations loaded"
    }

    companion object {
        // Used to load the 'buoyfinder' library on application startup.
        init {
            System.loadLibrary("buoyfinder")
        }
    }
}
