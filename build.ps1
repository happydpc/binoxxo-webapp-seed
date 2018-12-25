cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/binoxxo_webapp_seed.wasm --no-modules --out-dir ./dist
sass ./assets/main.scss ./dist/main.css
cp index.html ./dist
cp assets/favicon.png ./dist