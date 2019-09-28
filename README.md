# snake-rs - [play online](http://snake.kirjava.xyz)

build for gtk

```shell
cargo run --bin gtk --features=gtk-deps
```

build for web

```shell
cargo +nightly web start --target wasm32-unknown-unknown --bin web --features web-deps
```

build for nintendo switch

install [devKitPro](https://devkitpro.org/wiki/Getting_Started#Setup)
and follow the rusty-horizon [setup guide](https://github.com/rusty-horizon/setup-guide)

```
pacman -S switch-sdl2 switch-sdl2_ttf
cd switch && sh build.sh
```

or download the [nro](switch/snake-rs.nro)
