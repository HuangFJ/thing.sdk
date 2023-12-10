# thing.sdk
## Prequisites
```shell
brew install swiftformat ktlint
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-linux-android x86_64-linux-android armv7-linux-androideabi i686-linux-android wasm32-unknown-unknown
cargo install --path uniffi-bindgen 
npm install -g wasm-pack
# specific NDK path
NDK_TOOLCHAINS_PATH=/Users/jon/Library/Android/sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/darwin-x86_64/bin
```

## Build
```shell
make
```

## iOS
It will generate the following files:
- `projects/ios/headers/thingFFI.h` - C header file
- `projects/ios/headers/thingFFI.modulemap` - C modulemap file
- `projects/ios/Sources/thing.swift` - Swift wrapper
- `projects/ios/thing.xcframework` - XCFramework

### Add generated headers and xcframework to the project
Open the Xcode project in the `projects/ios` folder. Add the `headers/thingFFI.*`, `Sources/thing.swift` and `thing.xcframework` files to the responding folders in the Xcode project.


## Android
It will generate the following files:
- `projects/android/app/src/main/java/uniffi/thing/thing.kt` - Kotlin wrapper
- `projects/android/app/src/main/jniLibs/x86_64/libuniffi_thing.so` - Android x86_64 shared library
- `projects/android/app/src/main/jniLibs/arm64-v8a/libuniffi_thing.so` - Android arm64-v8a shared library
- `projects/android/app/src/main/jniLibs/armeabi-v7a/libuniffi_thing.so` - Android armeabi-v7a shared library

Open the Android Studio project in the `projects/android` folder. Add `implementation "net.java.dev.jna:jna:5.13.0@aar"` to dependencies of the app build.gradle.
