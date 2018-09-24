# BuoyFinder

A cross platform app for viewing NOAA buoys and their data

## Background

This is an *extremely* young **WIP** for workign with rust as a shared backend between iOS, Android, and UWP. The main idea is that views are implemented in every platform but the backend code stays the same. 

The end goal is to create a prototype app with some kind of code generator that can generate the "glue" code that plugs all of the rust, kotlin, swift, and C# together. To do this though, first a handwritten basic prototype should be created to work on creating generated code from as a base test. The current status of this repo is there is a sigle view plugged into rust, listening for updates via a view model and a (very rough and simple) redux backend. This is my first real foray into Rust so there is definitely some *bad* code in there, but the idea works.

#### 9/24/2018

A very simple prototype is ready to try and reimplement with generated code. 

## References 

https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html

https://github.com/kennytm/rust-ios-android

https://github.com/bignerdranch/rust-ios-app-part-4

https://github.com/fluffyemily/cross-platform-rust

https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
