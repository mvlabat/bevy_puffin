[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua)

# `bevy_puffin`

[![Crates.io](https://img.shields.io/crates/v/bevy_puffin.svg)](https://crates.io/crates/bevy_puffin)
[![Documentation](https://docs.rs/bevy_puffin/badge.svg)](https://docs.rs/bevy_puffin)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bevyengine/bevy/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/bevy_puffin.svg)](https://crates.io/crates/bevy_puffin)
[![Rust](https://github.com/mvlabat/bevy_puffin/workflows/CI/badge.svg)](https://github.com/mvlabat/bevy_puffin/actions)

This crate integrates the `puffin` library into Bevy.

It provides `PuffinTracePlugin` to use as a replacement for the Bevy's default `LogPlugin`
plugin and exposes `PuffinLayer`, which allows users set up `tracing` manually with `puffin`
as a subscriber layer.

For the usage example, see [`examples/main.rs`](examples/main.rs).
