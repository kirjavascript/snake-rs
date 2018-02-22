<div align="center">
    <img src="media/snake-gtk.gif" alt="snake-gtk">
</div>

run in gtk

```shell
cargo run --bin gtk --features=gtk-deps
```

run in web

cargo +nightly web start --target-webasm --bin web --features web-deps
