# Development Documentation

## Kesko

### Run Locally
For debug build from the `kesko` folder run
```
cargo run --bin kesko_main
```
And for the release build add the `--release` flag
```
cargo run --bin kesko_main --release
```

### Testing
in the `kesko` folder run
```bash
cargo test --all
```

### Build Wasm

#### Optimize
### Profiling

#### Tracing

To create a trace run

```bash
cargo run --release --features bevy/trace_chrome
```

Then go to `https://ui.perfetto.dev/` and upload the produced json trace

## PyKesko

### Build and Run Locally

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

### Testing
in the `pykesko` folder run
```bash
cargo test --all
```

### Release