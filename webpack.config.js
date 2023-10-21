import { resolve } from "path";
import CopyPlugin from "copy-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

const dist = resolve(__dirname, "dist");

export const mode = "production";
export const entry = {
  index: "./js/index.js"
};
export const output = {
  path: dist,
  filename: "[name].js"
};
export const devServer = {
  contentBase: dist,
};
export const plugins = [
  new CopyPlugin([
    resolve(__dirname, "static")
  ]),

  new WasmPackPlugin({
    crateDirectory: __dirname,
  }),
];
