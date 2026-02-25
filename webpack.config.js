//@ts-check

'use strict';

const { copyFileSync } = require('fs');
const path = require('path');

//@ts-check
/** @typedef {import('webpack').Configuration} WebpackConfig **/

/** @type WebpackConfig */
const extensionConfig = {
  target: 'node', // vscode extensions run in a Node.js-context
  mode: 'none', // this leaves the source code as close as possible to the original (when packaging we set this to 'production')

  entry: './src/extension.ts', // the entry point of this extension
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'extension.js',
    libraryTarget: 'commonjs2',
    wasmLoading: 'fetch',
  },
  externals: {
    vscode: 'commonjs vscode' // the vscode-module is created on-the-fly and must be excluded
  },
  resolve: {
    extensions: ['.ts', '.js']
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /node_modules/,
        use: [
          {
            loader: 'ts-loader'
          }
        ]
      },
      {
        exclude: [
          path.resolve(__dirname, '.github'),
          path.resolve(__dirname, 'grid_render'),
        ]
      }
    ]
  },
  devtool: 'nosources-source-map',
  experiments: {
    syncWebAssembly: true
  },
  plugins: [
    {
      apply: (compiler) => {
        compiler.hooks.afterEmit.tap('CopyWasmPlugin', () => {
          copyFileSync(
            path.join(__dirname, 'grid_render', 'pkg', 'grid_render_bg.wasm'),
            path.join(__dirname, 'dist', 'grid_render_bg.wasm')
          );
          copyFileSync(
            path.join(__dirname, 'grid_render', 'pkg', 'grid_render.js'),
            path.join(__dirname, 'dist', 'grid_render.js')
          );
        });
      }
    }
  ],
  infrastructureLogging: {
    level: "log",
  },
};

module.exports = [extensionConfig];
