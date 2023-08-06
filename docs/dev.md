# Development Documentation

- [Kesko](#kesko)
  - [Run](#kesko-run)
  - [Tests](#kesko-tests)
  - [WebAssembly](#kesko-webassembly)
    - [Using Trunk](#kesko-trunk)
    - [Using wasm-bindgen](#kesko-wasm-bindgen)
  - [Profiling](#kesko-profiling)
    - [Tracing](#kesko-tracing)
- [PyKesko](#pykesko)
  - [Build and Run](#pykesko-build-and-run)
  - [Tests](#pykesko-tests)
  - [Release](#pykesko-release)


## Kesko <a id="kesko"></a>

### Run <a id="kesko-run"></a>
For debug build from the `kesko` folder run
```bash
cargo run --bin kesko_main
```
And for the release build add the `--release` flag
```
cargo run --bin kesko_main --release
```

### Tests <a id="kesko-tests"></a>
in the `kesko` folder run
```bash
cargo test --all
```

### WebAssembly <a id="kesko-webassembly"></a>
Kesko can be compiled to web assembly, enable it to being run in the browser.

Add the wasm toolchain
```bash
rustup target install wasm32-unknown-unknown
```

#### Using Trunk <a id="kesko-trunk"></a>
Install Trunk

```bash
cargo install trunk
```

From `kesko` folder run

```bash
trunk serve --release
```

Open the adress that is being displayed in the terminal and Kesko
will start.

#### Using wasm-bindgen <a id="kesko-wasm-bindgen"></a>
Install wasm-bindgen

```bash
cargo install wasm-bindgen-cli
```

##### Build
in the `kesko` folder run
```bash
cargo build --target wasm32-unknown-unknown --bin kesko_main --release
```
Then run wasm-bindgen
```bash
wasm-bindgen --out-dir ./wasm/ --target web ./target/wasm32-unknown-unknown/release/kesko_main.wasm
```

##### Optimize With [Binaryen](https://github.com/WebAssembly/binaryen)
Binaryen is a compiler and toolchain infrastructure library for WebAssembly, written in C++.

Clone the repo
```bash
https://github.com/WebAssembly/binaryen.git
```
Binaryen uses some git submodules so jump into the repo folder and run
```bash
git submodule init
git submodule update
```
Then build it, this might take some time depending on your hardware.
```bash
cmake -DBUILD_TESTS=OFF . && make
# Install, make sure to refresh your terminal after
sudo make install
```

In order to optimize for both speed and size run
```bash
wasm-opt -O -ol 100 -s 100 -o wasm/kesko_main_optimized.wasm wasm/kesko_main_bg.wasm
```


### Profiling <a id="kesko-profiling"></a>

#### Tracing <a id="kesko-tracing"></a>

To create a trace run

```bash
cargo run --release --features bevy/trace_chrome
```

Then go to `https://ui.perfetto.dev/` and upload the produced json trace

## PyKesko <a id="pykesko"></a>

### Build and Run <a id="pykesko-build-and-run"></a>

Make sure you have an `venv` with the dependencies install then from the `pykesko` folder run
```bash
maturin develop --release
```
This will build the python package and install it in the `venv`.

Below is an example how to run the simulator
```python
from pykesko import Kesko
from pykesko.backend import BackendType, RenderMode
from time import sleep


if __name__ == "__main__":
    kesko = Kesko(backend_type=BackendType.BINDINGS, render_mode=RenderMode.WINDOW)
    kesko.initialize()

    for i in range(1000):
        kesko.step()

    kesko.close()
```

### Tests <a id="pykesko-tests"></a>
in the `pykesko` folder run
```bash
cargo test --all
```

### Release <a id="pykesko-release"></a>
TBD