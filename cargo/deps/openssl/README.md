# OpenSSL for Android NDK

## Instructions

* Download this conveinence repo
```bash
git clone https://github.com/david-hoze/build-openssl-android
```

* Build the libs 
```bash
ANDROID_NDK_ROOT=$ANDROID_NDK_HOME sh build-openssl-android.sh 21 21
```

* Copy them to this directory
```bash
cp -r build-openssl-android/openssl-libs/* .
```
