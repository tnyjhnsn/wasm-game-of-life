# wasm-game-of-life

Play [Game of Life](http://www.tosp.net.au/game-of-life/)

## About

This is a Rust learning project, using Yew for the frontend.
The scaffold is also setup with Tailwindcss but because that was
overkill for this project, it has been commented out in Webpack and
a simple css file was used instead.

### Inspiration

* [Rust/Yew Scaffold](https://github.com/yewstack/yew-wasm-pack-template)
* [Yew's Interval Service](https://github.com/huangjj27/game-of-life)
* [Implementing Game of Life logic](https://rustwasm.github.io/docs/book/game-of-life/implementing.html)



## 🚴 Usage

### 🛠️ Build

When building for the first time, ensure to install dependencies first.

```
yarn install
```

```
yarn run build
```

### 🔬 Serve locally

```
yarn run start:dev
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
