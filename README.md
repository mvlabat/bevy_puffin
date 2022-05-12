[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua)

# `bevy_puffin`

[![Crates.io](https://img.shields.io/crates/v/bevy_puffin.svg)](https://crates.io/crates/bevy_puffin)
[![Documentation](https://docs.rs/bevy_puffin/badge.svg)](https://docs.rs/bevy_puffin)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)
[![Downloads](https://img.shields.io/crates/d/bevy_puffin.svg)](https://crates.io/crates/bevy_puffin)
[![Rust](https://github.com/mvlabat/bevy_puffin/workflows/CI/badge.svg)](https://github.com/mvlabat/bevy_puffin/actions)

This crate integrates the [`puffin`](https://github.com/EmbarkStudios/puffin) library into Bevy.

It provides `PuffinTracePlugin` to use as a replacement for the Bevy's default `LogPlugin`
plugin and exposes `PuffinLayer`, which allows users to set up [`tracing`](https://github.com/tokio-rs/tracing)
manually with [`puffin`](https://github.com/EmbarkStudios/puffin) as a subscriber layer.

For the usage example, see [`examples/main.rs`](examples/main.rs).

## License

`bevy_puffin` is dual-licensed under either

* MIT License ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## See also

- [`mvlabat/bevy_egui`](https://github.com/mvlabat/bevy_egui)
- [`EmbarkStudios/puffin`](https://github.com/EmbarkStudios/puffin)

## Bevy support table

| bevy | bevy_puffin |
|------|-----------|
| 0.7  | 0.1 |
