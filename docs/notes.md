## Profiling

### Tracing

To create a trace run

```bash
cargo run --release --features bevy/trace_chrome
```

Then go to `https://ui.perfetto.dev/` and upload the produced json trace