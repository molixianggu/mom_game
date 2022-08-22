
<div align="center">

# 🕊️ A Bevy Game

</div>


## 🚀 Launchers
### WASM (Web)
This launcher depends on the [trunk](https://trunkrs.dev/) crate.
To build and run the WASM app locally:
> Serve with `trunk serve` and open [`http://127.0.0.1:8080`](http://127.0.0.1:8080) in your browser
- Assets are streamed through the hosting provider, so that the initial WASM bundle is smaller.
- We use all the WASM optimizations discussed described [here](https://rustwasm.github.io/book/reference/code-size.html) in the Rust and WebAssembly book.
- There is an initial loading screen provided through [Yew](https://yew.rs) while the WASM bundle loads.

### Native (Windows, MacOS, Linux)
> Run with `cargo run`
- Assets are bundled with the release when cut.
- There is no loading screen.
