const rust = require("@wasm-tool/rollup-plugin-rust");
const serve = require("rollup-plugin-serve");
const livereload = require("rollup-plugin-livereload");
const injectProcessEnv = require('rollup-plugin-inject-process-env');
const json = require('rollup-plugin-json');

require('dotenv').config({ path: require("path").resolve('../.env') })

const buildMode = process.env.BUILD_MODE;

if(buildMode !== "dev" && buildMode !== "release") {
    throw new Error("BUILD_MODE env must be one of 'dev' or 'release'");
}

const config = {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        dir: "public/wasm",
        format: "iife",
        sourcemap: true,
    },
}

const commonPlugins = [
    json({}),
    injectProcessEnv({
    }),
];


if(buildMode === "dev") {
    config.plugins = [ 
        rust({
            serverPath: "/wasm/",
            debug: true,
            watchPatterns: [
                "../../shipyard-scenegraph/crate/src/**", 
                "src/**", 
                "Cargo.toml", 
                "../crate/Cargo.toml", 
                "../crate/src/**", 
                "public/**/*.html", 
                "public/**/*.css"
            ],
            cargoArgs: ["--features", "dev"],
            watch: true,
        }),

        ...commonPlugins,

        serve({
            contentBase: "public",
            open: true,
            historyApiFallback: true,
        }),

        livereload("public"),
    ];
} else {
    config.plugins = [
        rust({
            serverPath: "/wasm/",
            debug: false,
        }),
        ...commonPlugins
    ];
}

export default config;
