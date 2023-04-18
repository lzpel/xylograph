# 方針

## 20230417

### 気づき

- bin/はsrc/下にないと認識されない
- cargo doc --open でドキュメント生成

### webgl
```shell
rustup target install wasm32-unknown-unknown
cargo install wasm-bindgen-cli
curl https://github.com/WebAssembly/binaryen/releases/download/version_112/binaryen-version_112-x86_64-windows.tar.gz | tar -zxv
echo "copy bin/*.exe to .cargo/bin"
```

```shell
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./target/ --target web ./target/wasm32-unknown-unknown/release/3d_scene.wasm
wasm-opt -Oz -c target/3d_scene_bg.wasm -o target/3d_scene_bg.wasm
echo "http://localhost:8000/test.html"
python -m http.server
```

調査
```shell
twiggy top -n 10 target/wasm32-unknown-unknown/release/hello-world.wasm
```

これで更に小さくなった
https://webassembly.github.io/binaryen/shrink.html
逆にwasm-snipは意味ない、wasm-bindgenに内包されているかも

### サブパッケージの動作

```shell
cargo run --package bevy_rhai --bin bevy_rhai
```

### 3d_scene.rsが最低限動いたfeature

```toml
[dependencies.bevy]
version = "0.10.1"
default-features = false
features = [
#"animation",
"bevy_asset",
#"bevy_audio",
#"bevy_gilrs",
"bevy_scene",
"bevy_winit",
"bevy_core_pipeline",
"bevy_pbr",
#"bevy_gltf",
"bevy_render",
#"bevy_sprite",
#"bevy_text",
#"bevy_ui",
#"png",
#"hdr",
#"ktx2",
#"zstd",
#"vorbis",
#"x11",
#"filesystem_watcher",
#"android_shared_stdcxx",
#"tonemapping_luts",
]
```

## 20230414

### ゲーム作りたい

- どんなゲーム？
  - web:マテリアルスナイパー
  - desktop:MPS法可視化
- 動画版Texを作りたい。
  - script:javascript
    - webではnative
    - desktopではdeno
  - desktop:luaから動画生成

### bevy でオセロでも作った方が百倍時間の有効活用

## 20230410

```shell
git clone git@github.com:glium/glium
cd glium
ls examples/ -1

cargo run --example tessellation
cargo run --example triangle
```