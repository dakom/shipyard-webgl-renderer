[![Build Status](https://github.com/dakom/awsm-renderer/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/awsm-renderer/actions)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/awsm_renderer.svg)](https://crates.io/crates/awsm_renderer)
[![Documentation](https://docs.rs/awsm_renderer/badge.svg)](https://docs.rs/awsm_renderer)
[![Demo](https://img.shields.io/badge/demo-launch-yellow)](https://dakom.github.io/awsm-renderer)

# Status

Nothing to see here... YET!

# What will it be?

A webgl2 renderer that builds on _awsm-web_ and _Shipyard ECS_:

### Dependency stack
| Web | ECS |
| ----- | ---- |
| [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) | [shipyard](https://github.com/leudz/shipyard) |
| [awsm_web](https://github.com/dakom/awsm-web) | [shipyard-hierarchy](https://github.com/dakom/shipyard-hierarchy) |
| | [shipyard-scenegraph](https://github.com/dakom/shipyard-scenegraph) |


The example additionally uses [Dominator](https://github.com/Pauan/rust-dominator) to tie it all together with a DOM-based UI


