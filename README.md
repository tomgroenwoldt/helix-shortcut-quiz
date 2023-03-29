[![pages-build-deployment](https://github.com/tomgroenwoldt/helix-shortcut-quiz/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/tomgroenwoldt/helix-shortcut-quiz/actions/workflows/pages/pages-build-deployment)
# Helix Shortcut Quiz

A web quiz built with [yew](https://yew.rs/) for testing your knowledge of [helix editor](https://helix-editor.com/) shortcuts.

![helix-shortcut-quiz](https://user-images.githubusercontent.com/70777530/228351818-b7ebe8f2-a672-4759-86ab-e395c9cf211b.gif)

## Development

#### Installation

###### Frontend

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
Install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Install trunk:

```bash
cargo install trunk
```

###### GIF generation

The `yew` app serves GIFs generated by [`vhs`](https://github.com/charmbracelet/vhs). The current state of `vhs` doesn't
support the recording of the helix block cursor. The problem lies deeper within the [`ttyd`](https://github.com/tsl0922/ttyd)
dependency which uses the [`xterm.js`](https://github.com/xtermjs/xterm.js) canvas addon.

If you want to generate the GIFs yourself you should go ahead, clone the custom forks I made and use those binaries instead.
After building the two binaries place them in the `gif_generation/` directory and execute `generate_all_gifs.sh`. 

#### Running

```bash
trunk serve
```
