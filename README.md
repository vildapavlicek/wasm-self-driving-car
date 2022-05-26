<div align="center">


  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>

  <h1><code>create-wasm-app</code></h1>

  <strong>An <code>npm init</code> template for kick starting a project that uses NPM packages containing Rust-generated WebAssembly and bundles them with Webpack.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/create-wasm-app"><img src="https://img.shields.io/travis/rustwasm/create-wasm-app.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="#usage">Usage</a>

    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About


[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
=======
This template is designed for depending on NPM packages that contain
Rust-generated WebAssembly and using them to create a Website.

* Want to create an NPM package with Rust and WebAssembly? [Check out
  `wasm-pack-template`.](https://github.com/rustwasm/wasm-pack-template)
* Want to make a monorepo-style Website without publishing to NPM? Check out
  [`rust-webpack-template`](https://github.com/rustwasm/rust-webpack-template)
  and/or
  [`rust-parcel-template`](https://github.com/rustwasm/rust-parcel-template).

## 🚴 Usage

```
npm init wasm-app
```

## 🔋 Batteries Included

- `.gitignore`: ignores `node_modules`
- `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you
- `README.md`: the file you are reading now!
- `index.html`: a bare bones html document that includes the webpack bundle
- `index.js`: example js file with a comment showing how to import and use a wasm pkg
- `package.json` and `package-lock.json`:
  - pulls in devDependencies for using webpack:
      - [`webpack`](https://www.npmjs.com/package/webpack)
      - [`webpack-cli`](https://www.npmjs.com/package/webpack-cli)
      - [`webpack-dev-server`](https://www.npmjs.com/package/webpack-dev-server)
  - defines a `start` script to run `webpack-dev-server`
- `webpack.config.js`: configuration file for bundling your js with webpack

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

