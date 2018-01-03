Vywrs
---

Vywrs is a complete rewrite of [vyw](https://github.com/fudanchii/vyw), a files and images browser which only need nginx as backend.

Vywrs written in rust, and supposed to be compiled into wasm format, this is possible thanks to great projects as: yew, stdweb, and cargo-web, to name a few.

Vywrs config is identical to vyw, in addition vywrs now has list view mode in addition to thumbnail view.

demo site
---

https://vywrs.fudanchii.net

how to build
---

- install rustup, and install rust nightly toolchain
- install wasm32-unknown-unknown target with rustup
- install cargo-web with cargo
- install ruby, and install sass gem (needed to compile scss stylesheet)
  make sure sass/scss is available in your PATH
- run make

build result will be available under dist folder. You can serve this directory with nginx.
