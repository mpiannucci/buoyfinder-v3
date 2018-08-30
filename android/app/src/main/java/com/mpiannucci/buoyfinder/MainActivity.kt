package com.mpiannucci.buoyfinder

import android.support.v7.app.AppCompatActivity
import android.os.Bundle

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
    }

    companion object {
        // Used to load the 'buoyfinder' library on application startup.
        init {
            System.loadLibrary("buoyfinder")
        }
    }
}
