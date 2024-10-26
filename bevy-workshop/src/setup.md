# Setup

## Clone the repository

TODO
```sh
git clone ...
```

## Environment setup

Option 1 is recommended if your local machine supports it. This workshop won't be GPU heavy so most hardware configurations should support running it.

### Option 1: Local Setup

* Install rust: [https://rustup.rs](https://rustup.rs)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* Install linux dependencies: [https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md)

* First build of the workshop. The initial build can take some time.

```sh
cargo build
```

### Option 2: Docker Setup

This option can be interesting if you can't install dependencies on your machine, or the setup fails for some obscure reason. Instead of running natively, the workshop will run in your browser using wasm and WebGL2, delegating most OS/hardware integration to the browser.

* Run a docker image

```sh
docker run -v `pwd`:/workspace -p 8000:8000 rust
rustup target add wasm32-unknown-unknown
python3 -m http.server --directory wasm
```
