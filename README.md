# snake-rs

<div align="center">
    <img src="static/snake-gtk.gif" alt="snake-gtk">
</div>

run in gtk

```shell
cargo run --bin gtk --features=gtk-deps
```

run in web

```shell
cargo +nightly web start --target wasm32-unknown-unknown --bin web --features web-deps
```
