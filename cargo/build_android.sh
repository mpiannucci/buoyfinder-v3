# ARM
CC=$PWD/NDK/arm/bin/arm-linux-androideabi-gcc AR=$PWD/NDK/arm/bin/arm-linux-androideabi-ar OPENSSL_DIR=$PWD/deps/openssl/armeabi-v7a cargo build --target armv7-linux-androideabi

# ARM64
CC=$PWD/NDK/arm64/bin/aarch64-linux-android-gcc AR=$PWD/NDK/arm64/bin/aarch64-linux-android-ar OPENSSL_DIR=$PWD/deps/openssl/arm64-v8a cargo build --target aarch64-linux-android

# x86
CC=$PWD/NDK/x86/bin/i686-linux-android-gcc AR=$PWD/NDK/x86/bin/i686-linux-android-ar OPENSSL_DIR=$PWD/deps/openssl/x86 cargo build --target i686-linux-android

