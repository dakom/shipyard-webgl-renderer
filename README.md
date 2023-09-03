[![Build Status](https://github.com/dakom/awsm-renderer/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/awsm-renderer/actions)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/awsm_renderer.svg)](https://crates.io/crates/awsm_renderer)
[![Documentation](https://docs.rs/awsm_renderer/badge.svg)](https://docs.rs/awsm_renderer)
[![Demo](https://img.shields.io/badge/demo-launch-yellow)](https://dakom.github.io/awsm-renderer)

# Status

Nothing to see here... YET!

# [DEMO](https://dakom.github.io/awsm-renderer)

# What will it be?

A Rust wasm/webgl2 renderer that uses [Shipyard ECS](https://github.com/leudz/shipyard) for the Entity-Component-System engine and [awsm-web](https://github.com/dakom/awsm-web) for web api helpers.

Though by the time it's done, WebGPU might be a thing... so we'll see :P

While this crate can be used as a library, and the Shipyard World can be passed around freely, another viable approach is to just clone the code locally. That's probably the most straightforward way to add more post-processing effects, particle systems, and other things that may want to hook into the core renderer logic mid-pipeline.

# Thank you 

PBR shader code and more "ported" (with much copy-paste!) from the [glTF Sample Viewer](https://github.com/KhronosGroup/glTF-Sample-Viewer)

### Dependency stack
| Web | ECS |
| ----- | ---- |
| [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) | [shipyard](https://github.com/leudz/shipyard) |
| [awsm_web](https://github.com/dakom/awsm-web) | [shipyard-hierarchy](https://github.com/dakom/shipyard-hierarchy) |
| | [shipyard-scenegraph](https://github.com/dakom/shipyard-scenegraph) |


The demo additionally uses [Dominator](https://github.com/Pauan/rust-dominator) to tie it all together with a DOM-based UI

# Local Dev

**One-time setup**

* clone `https://github.com/KhronosGroup/glTF-Sample-Models.git` into `demo/media/glTF-Sample-Models`
* install [trunk](https://trunkrs.dev/#install)
* install [http-server](https://www.npmjs.com/package/http-server) (this is just for locally serving the media, you can easily edit package.json and use a different http server)
* `yarn install` in `demo/`

**Up and running**
* `yarn serve:dev` from inside `demo/`

# Media

HDRIs from https://polyhaven.com/hdris