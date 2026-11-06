# Linux Process Explorer

Inspect parent-child process trees, sort by resident memory, filter by name/PID, and watch a live view.

[![Build and verify](https://github.com/netcodermike/linux-process-explorer/actions/workflows/verify.yml/badge.svg)](https://github.com/netcodermike/linux-process-explorer/actions/workflows/verify.yml)

**Language:** Rust · **Platform:** Linux · **App type:** Linux terminal inspector

First-release learning implementation, with explicit scope below. This is one independently runnable app, not a placeholder in a combined repository.

## Features

- Process tree with PID, PPID, RSS KiB, name, and state.
- Memory-descending flat view.
- Case-insensitive name substring or exact numeric PID filter.
- Two-second watch mode and control-character sanitization.

## Requirements

Rust stable (edition 2021), a Linux system exposing /proc, and a terminal. No crates or root privileges are required.

## Quick start

```sh
git clone https://github.com/netcodermike/linux-process-explorer.git
cd linux-process-explorer
cargo run --release
cargo run --release -- --memory
cargo run --release -- --filter ssh --watch
```

Commands run from the repository root. Lines marked for a particular operating system apply only there. Language tools are prerequisites, not installed by the app. Close desktop windows normally; terminal/server apps stop with Ctrl+C unless their usage notes say otherwise.

## Example workflow

Run the default tree, then --memory to inspect large resident sets. Filter for a known process name or a PID. Run --watch while opening and closing another program; the visible process list should change.

## Architecture, configuration, and data

`src/main.rs` parses per-process `/proc/PID/status` files. Numeric directory names supply PIDs. A parent graph builds the tree; orphans remain visible and a visited set breaks cycles. Processes that exit or cannot be read are omitted. Filtering happens before tree rendering, so missing filtered ancestors are treated as roots. --memory changes sorting; --watch redraws using ANSI escape sequences. The app is read-only and creates no files or network connections.

## Verification

Build with the quick-start command where needed, then run:

```sh
cargo test
```

Manual acceptance: Compare a known PID’s PPID/RSS to its /proc/PID/status, test a nonexistent filter, and observe process creation/removal in watch mode.

See [validation record](docs/VALIDATION.md) for observed checks and their limits. CI results verify only what the workflow runs; they do not establish native GUI correctness or production readiness.

## Limitations and next steps

RSS is not unique/private memory and may count shared pages in several processes. This is a sampled, non-atomic view of a changing system. Names may be truncated by the kernel. Parent matching is quadratic and intended for personal machines. The view does not signal/kill processes, inspect command lines, track CPU, or cross PID namespaces. Watch mode requires an ANSI terminal.

## Troubleshooting

A non-Linux host exits with a platform explanation. Restricted /proc mounts can hide processes. Missing VmRSS is shown as zero for kernel tasks. Filtered parents disappearing from the tree is expected. Stop watch mode with Ctrl+C.

## Source map

- `Cargo.toml`
- `src/main.rs`

## Development and license

Read [contributing](CONTRIBUTING.md) before changing the app, and [security/privacy](SECURITY.md) before supplying private data. Use synthetic fixtures for tests. No telemetry, paid API key, or account is required by the app itself. Dependencies, when present, are stated in Requirements.

[MIT license](LICENSE), copyright 2026 netcodermike.
