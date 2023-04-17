# 方針

## 20230417

サブパッケージの動作

run --package bevy_rhai --bin bevy_rhai

3d_scene.rsが最低限動いたfeature

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