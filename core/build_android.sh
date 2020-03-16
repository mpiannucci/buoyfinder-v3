# ARM
TARGET_CC=$PWD/NDK/arm/bin/arm-linux-androideabi-clang TARGET_AR=$PWD/NDK/arm/bin/arm-linux-androideabi-ar cargo build --target armv7-linux-androideabi --release

# ARM64
TARGET_CC=$PWD/NDK/arm64/bin/aarch64-linux-android-clang TARGET_AR=$PWD/NDK/arm64/bin/aarch64-linux-android-ar cargo build --target aarch64-linux-android --release

# x86
TARGET_CC=$PWD/NDK/x86/bin/i686-linux-android-clang TARGET_AR=$PWD/NDK/x86/bin/i686-linux-android-ar cargo build --target i686-linux-android --release

cp target/armv7-linux-androideabi/release/libbuoyfinder_core.so ../android/app/src/main/jniLibs/armeabi-v7a/
cp target/aarch64-linux-android/release/libbuoyfinder_core.so ../android/app/src/main/jniLibs/arm64-v8a/
cp target/i686-linux-android/release/libbuoyfinder_core.so ../android/app/src/main/jniLibs/x86/