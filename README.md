# example-rust

[![coverage badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fdemo.coveragetracker.dev%2Fapi%2Fbadge%2FCoverageTracker%2Fexample-rust%2Fcoverage.json)](https://demo.coveragetracker.dev/CoverageTracker/example-rust?metric=coverage)
[![complexity badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fdemo.coveragetracker.dev%2Fapi%2Fbadge%2FCoverageTracker%2Fexample-rust%2Fcomplexity.json)](https://demo.coveragetracker.dev/CoverageTracker/example-rust?metric=complexity)

A small, idiomatic Rust password strength and policy library used as the
Rust reference example for [Coverage Tracker](https://coveragetracker.dev).
It exists to give the Rust row in the
[coverage report generation guide](https://coveragetracker.dev/docs/generating-coverage-reports)
a live, working reference, and to populate the
[demo dashboard](https://demo.coveragetracker.dev) with real trend data.

**This is a demo/marketing repo, not a test suite for Coverage Tracker
itself.**

## What's here

- `src/strength.rs` — heuristic password strength scoring from length and
  character-class variety.
- `src/policy.rs` — configurable pass/fail policy checks (min length,
  required character classes) plus human-readable descriptions.
- `src/entropy.rs` — a rough Shannon-entropy estimate and qualitative rating.
- `src/blocklist.rs` — a small common-password denylist shared by the above.
- Each module has unit tests with a few deliberately uncovered
  branches, so `branch_coverage < line_coverage` shows up on the dashboard.
- `.github/workflows/coverage.yml` — runs tests under
  [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov), generates a
  [Lizard](https://github.com/terryyin/lizard) complexity report, then
  reports both to the demo instance via the `coverage-tracker` reporting
  Action.

## Running locally

```sh
rustup toolchain install nightly --component llvm-tools-preview
cargo install cargo-llvm-cov
cargo +nightly llvm-cov --branch --lcov --output-path lcov.info   # writes lcov.info
python -m lizard src --xml > lizard-report.xml
```

Branch coverage in the LCOV output requires the **nightly** toolchain and
the (unstable) `--branch` flag — `cargo-llvm-cov`'s stable-toolchain default
command emits no `BRDA` branch records at all.
