# BuoyFinder

Core BuoyFinder logic and functionality, written in Rust

## Installing toolchains

**Assumes `rustup` and the android NDK are installed**

* Install `cbindgen`
```bash
cargo install cbindgen
```

* Generate C ABI Bindings
```bash
bash ./gen_bindings.sh
```

* Install `ios` targets
```bash
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
```

* Install `android` targets
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

* Install `cargo-lipo`
```bash
cargo install cargo-lipo
```

* Generate the standalone NDK
```bash
bash create-ndk-standalone.sh
```

* Install the android cargo configuration
```bash
cp cargo-config.toml ./.cargo/config
```

## Building

* Build ios universal binary
```bash
cargo lipo --release
```

* Build android libraries
```bash
./build_android
```

## Linking 

* Link to the fat static library in Xcode
```
Link libresolv.d
Link libbuoyfinder_core.a
Add ./target/universal/release to LIBRARY_SEARCH_PATHS
```

* Link to the shared libraries through the jni. Copy them over for use
```bash
cp target/armv7-linux-androideabi/release/libbuoyfinder_core.so ../android/src/main/jniLibs/armeabi-v7a/libbuoyfinder_core.so
cp target/aarch64-linux-android/release/libbuoyfinder_core.so ../android/src/main/jniLibs/arm64-v8a/libbuoyfinder_core.so
cp target/i686-linux-android/release/libbuoyfinder_core.so ../android/src/main/jniLibs/x86/libbuoyfinder_core.so
```