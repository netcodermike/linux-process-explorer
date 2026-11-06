# Validation

Authoring date: November 6, 2026. Host: Windows. Target: Linux.

## Local result

Linux runtime and Rust compiler are not available on the authoring host. CI builds, tests parsers/tree behavior, and reads live /proc.

## Reproducible checks

The GitHub Actions workflow builds/runs the following on `ubuntu-latest`:

```sh
cargo test
cargo run --release -- --filter sh
```

## Manual acceptance

Compare a known PID’s PPID/RSS to its /proc/PID/status, test a nonexistent filter, and observe process creation/removal in watch mode.

A workflow file is not evidence of a passing run. Observed CI run links and source commits are recorded after publication. Native GUI, sound, sleep/resume, and platform permissions require direct target-platform acceptance even when model tests pass.
