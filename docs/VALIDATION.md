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

## Observed CI result

Passed on November 6, 2026: [target-platform build/check](https://github.com/netcodermike/linux-process-explorer/actions/runs/42198852722).

Verified source commit: `2cd4d0701c7367ea76f1da163bca488bb0b8870f`. This evidence identifies the exact tested source; a later documentation-only commit does not change that source.

The commands listed above passed. Native GUI/manual acceptance is not claimed by this CI result.
