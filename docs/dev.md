# Development Documentation

## Testing

### Kesko
in the `kesko` folder run
```bash
cargo test --all
```

### PyKesko
in the `pykesko` folder run
```bash
cargo test --all
```

## Profiling

### Tracing

To create a trace run

```bash
cargo run --release --features bevy/trace_chrome
```

Then go to `https://ui.perfetto.dev/` and upload the produced json trace