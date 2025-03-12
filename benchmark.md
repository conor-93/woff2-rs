### Native
- Run `cargo test --package woff2 --release --example decoder benchmark -- --exact --nocapture`
- Output TTF files are written to same location as source (in `test_assets_dir`)
- Conversion latencies logged in `stdout`

### WASM (no output, just for benchmarking):
- Run `wasm-pack build --target web` (optional: rebuilds/packs WASM package)
- Run `python3 -m http.server 8080` from root
- Navigate to `localhost:8080` in the browser
- Open Developer Tools (Chrome), navigate to the Console tab
- Click 'Choose files' in the web app, and select one or more fonts
- Conversion latencies logged in `stdout`

### Native (Android)
**Full build/deploy**
```
cargo ndk -t arm64-v8a build --release
adb push target/aarch64-linux-android/release/woff2-rs-android-benchmark-native /data/local/tmp/
adb shell chmod +x /data/local/tmp/woff2-rs-android-benchmark-native
```

Copy asset files to device:
- Create an asset store:	`adb shell mkdir -p /data/local/tmp/test_assets/`
- Copy assets: 				`adb push test_assets/* /data/local/tmp/test_assets/`

**Run**
```
adb shell "cd /data/local/tmp && ./woff2-rs-android-benchmark-native"
```