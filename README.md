# photo-filter
ブラウザで動くオリジナル画像フィルターです(wasm を使って何かしたかった)

pages : https://i-icc.github.io/photo-filter/

## wasm(Rust) side

### setup
```sh
# rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# wasm pack install
rustup target add wasm32-unknown-unknown
```

### build
```sh
cd photo_filter
wasm-pack build --target web
```

### rust run
```sh
cd photo_filter
cargo run
```

### mv package
```sh
cp -r photo_filter/pkg photo-filter-client/src/
```