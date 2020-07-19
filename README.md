# wasm-game-of-life

[View the Production Version](http://www.tosp.net.au/game-of-life/)

## About

This is a Rust/WASM learning project, using Yew for the frontend. HTML markup in Yew uses a syntax similar to JSX. I also added Tailwindcss to the scaffold, but because that was
overkill for this project, it was disabled in Webpack in favour of a simple css file.

### Inspiration & acknowledgements

* Original [Rust/Yew Scaffold](https://github.com/yewstack/yew-wasm-pack-template)
* How to implement [Yew's Interval Service](https://github.com/huangjj27/game-of-life)
* Rust non-Yew [Game of Life tutorial](https://rustwasm.github.io/docs/book/game-of-life/implementing.html)



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
