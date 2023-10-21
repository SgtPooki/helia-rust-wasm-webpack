// @ts-check
import { resolve } from "path";
import CopyPlugin from "copy-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import * as url from 'url';

const __filename = url.fileURLToPath(import.meta.url);
const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

const dist = resolve(__dirname, "dist");

export const mode = "production";
export const entry = {
  index: "./js/index.js"
};

/**
 * @type {import("webpack").WebpackOptionsNormalized["devServer"]}
 */
export const output = {
  path: dist,
  filename: "[name].js"
};
// export const devServer = {
//   contentBase: dist,
// };

/**
 * @type {import("webpack").WebpackOptionsNormalized["devServer"]}
 */
export const devServer = {
  static: {
    directory: dist,
  },

}

/**
 * @type {import("webpack").WebpackOptionsNormalized["plugins"]}
 */
export const plugins = [
  // new CopyPlugin([
  //   resolve(__dirname, "static")
  // ]),
  new CopyPlugin({
    patterns: [
      { from: resolve(__dirname, "static") }
    ]
  }),
  new WasmPackPlugin({
    crateDirectory: __dirname,
  }),
];



/**
 * @type {import("webpack").Configuration}
 */
export default {
  devServer,
  entry,
  experiments: {
    // I need to handle the below error:
    // BREAKING CHANGE: Since webpack 5 WebAssembly is not enabled by default and flagged as experimental feature.
    // You need to enable one of the WebAssembly experiments via 'experiments.asyncWebAssembly: true' (based on async modules) or 'experiments.syncWebAssembly: true' (like webpack 4, deprecated).
    // For files that transpile to WebAssembly, make sure to set the module type in the 'module.rules' section of the config (e. g. 'type: "webassembly/async"').
    // (Source code omitted for this binary file)
    // asyncWebAssembly: true,
    futureDefaults: true,
  },
  mode,
  output,
  performance: {
    maxEntrypointSize: 650000,
    maxAssetSize: 650000,
  },
  plugins
};
