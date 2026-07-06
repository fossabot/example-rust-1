# example-rust

[![coverage badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fdemo.coveragetracker.dev%2Fapi%2Fbadge%2FCoverageTracker%2Fexample-rust%2Fcoverage.json)](https://demo.coveragetracker.dev/CoverageTracker/example-rust?metric=coverage)
[![complexity badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fdemo.coveragetracker.dev%2Fapi%2Fbadge%2FCoverageTracker%2Fexample-rust%2Fcomplexity.json)](https://demo.coveragetracker.dev/CoverageTracker/example-rust?metric=complexity)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FCoverageTracker%2Fexample-rust.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FCoverageTracker%2Fexample-rust?ref=badge_shield)

A small, idiomatic Rust password strength and policy library used as the
Rust reference example for [Coverage Tracker](https://coveragetracker.dev).
It exists to give the Rust row in the
[coverage report generation guide](https://coveragetracker.dev/docs/generating-coverage-reports)
a live, working reference, and to populate the
[demo dashboard](https://demo.coveragetracker.dev) with real trend data.

**This is a demo/marketing repo, not a test suite for Coverage Tracker
itself.** `cargo-llvm-cov`'s stable-toolchain default emits line coverage
only — branch data requires the nightly toolchain and an unstable `--branch`
flag, which this repo deliberately doesn't take on — so unlike the other
example repos, this one won't show a branch coverage metric on the
dashboard (same situation as `example-go`).

## What's here

- `src/strength.rs` — heuristic password strength scoring from length and
  character-class variety.
- `src/policy.rs` — configurable pass/fail policy checks (min length,
  required character classes) plus human-readable descriptions.
- `src/entropy.rs` — a rough Shannon-entropy estimate and qualitative rating.
- `src/blocklist.rs` — a small common-password denylist shared by the above.
- Each module has unit tests, but each also has a real function or two left
  deliberately untested (`Strength`'s "Excellent" tier, `require_special`
  policy checks, most `Display`/description text), landing at ~77% line
  coverage.
- `.github/workflows/coverage.yml` — runs tests under
  [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov), generates a
  [Lizard](https://github.com/terryyin/lizard) complexity report, then
  reports both to the demo instance via the `coverage-tracker` reporting
  Action.

## Running locally

```sh
cargo install cargo-llvm-cov
cargo llvm-cov --lcov --output-path lcov.info   # writes lcov.info
python -m lizard src --xml > lizard-report.xml
```


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FCoverageTracker%2Fexample-rust.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FCoverageTracker%2Fexample-rust?ref=badge_large)