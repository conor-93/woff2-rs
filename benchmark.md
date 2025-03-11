### Usage / Benchmarking

Native
- Check `benchmark.rs#main` -> `test_assets_dir` (top of file) to see which folder WOFF2 fonts will be read from
    - Directories are recursively searched (so, `"test_assets"` should get all existing fonts)
- Through the IDE, run `cargo test --package woff2 --example decoder benchmark -- --exact` from root
- Output TTF files are written to same location as source (in `test_assets_dir`)
- Conversion latencies logged in `stdout`

WASM (no output, just for benchmarking):
- Run `wasm-pack build --target web` (optional: rebuilds/packs WASM package)
- Run `python3 -m http.server 8080` from root
- Navigate to `localhost:8080` in the browser
- Open Developer Tools (Chrome), navigate to the Console tab
- Click 'Choose files' in the web app, and select one or more fonts
- Conversion latencies logged in `stdout`