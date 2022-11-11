Vywrs
---

Vywrs is a complete rewrite of [vyw](https://github.com/fudanchii/vyw), a files and images browser which only need nginx as backend.

Vywrs is written in rust, and supposed to be compiled into wasm format, this is possible thanks to [yew](https://yew.rs).

Vywrs config is identical to vyw, in addition vywrs now has list view mode to complement thumbnail view.

how to build
---

- Ensure you have rustup and stable Rust instalation (> 1.39) 
- Install wasm32-unknown-unknown build target with rustup
- Follow ["Getting Started with Trunk"](https://yew.rs/docs/getting-started/project-setup/using-trunk) in yew.rs docs.
- Run `trunk serve` to run vywrs in development mode
- Run `trunk build --release` and serve `dist/` folder in HTTP server to run vywrs in release mode 

build result will be available under dist folder.

server configuration
---
vywrs make use nginx json format for file listing, e.g.
```json
[
{ "name":"bower_components", "type":"directory", "mtime":"Wed, 05 Sep 2018 09:10:11 GMT" },
{ "name":"css", "type":"directory", "mtime":"Fri, 31 Aug 2018 00:07:35 GMT" },
{ "name":"img", "type":"directory", "mtime":"Wed, 26 Sep 2018 10:50:09 GMT" },
{ "name":"js", "type":"directory", "mtime":"Wed, 05 Sep 2018 06:24:15 GMT" },
{ "name":"index.html", "type":"file", "mtime":"Fri, 07 Sep 2018 02:42:57 GMT", "size":730 }
]
```

Another server may be used if it can be configured to serve listing in the previous format.

For nginx, configuration may look like this:
```
location = /index.json {
  index '*';
  autoindex on;
  autoindex_format json;
  rewrite ^ /$arg_p/ break;
  add_header Access-Control-Allow-Origin "*";
}
```

The above configuration may be used as the following:
```
http://localhost/index.json?p=<PATHNAME>
```


3rd party libraries
---

This app invoke and re-bind some methods from [GLightbox](https://github.com/biati-digital/glightbox), which licensed under [MIT](https://github.com/biati-digital/glightbox/blob/master/license.md).
