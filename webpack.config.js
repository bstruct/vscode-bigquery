//@ts-check

'use strict';

const { copyFileSync } = require('fs');
const path = require('path');

//@ts-check
/** @typedef {import('webpack').Configuration} WebpackConfig **/

/** @type WebpackConfig */
const extensionConfig = {
  target: 'node', // vscode extensions run in a Node.js-context 📖 -> https://webpack.js.org/configuration/node/
  mode: 'none', // this leaves the source code as close as possible to the original (when packaging we set this to 'production')

  entry: './src/extension.ts', // the entry point of this extension, 📖 -> https://webpack.js.org/configuration/entry-context/
  output: {
    // the bundle is stored in the 'dist' folder (check package.json), 📖 -> https://webpack.js.org/configuration/output/
    path: path.resolve(__dirname, 'dist'),
    filename: 'extension.js',
    libraryTarget: 'commonjs2',
    wasmLoading: 'fetch',
  },
  externals: {
    vscode: 'commonjs vscode' // the vscode-module is created on-the-fly and must be excluded. Add other modules that cannot be webpack'ed, 📖 -> https://webpack.js.org/configuration/externals/
    // modules added here also need to be added in the .vscodeignore file
  },
  resolve: {
    // support reading TypeScript and JavaScript files, 📖 -> https://github.com/TypeStrong/ts-loader
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
          path.resolve(__dirname, 'bqsql_parser'),
        ]
      }
    ]
  },
  // module: {
  //   rules: [
  //     {
  //       exclude: [
  //         path.resolve(__dirname, '.github'),
  //         path.resolve(__dirname, 'grid_render'),
  //         // path.resolve(__dirname, 'node_modules'),
  //       ]
  //     }
  //   ]
  // },
  devtool: 'nosources-source-map',
  experiments: {
    syncWebAssembly: true
  },
  plugins: [
    // (a, b) => {
    //   copyFileSync(
    //     require('path').join(__dirname, 'node_modules', '@bstruct', 'bqsql-parser', 'bqsql_parser_bg.wasm'),
    //     require('path').join(__dirname, 'dist', 'bqsql_parser_bg.wasm')
    //   );
    // },
    (a, b) => {
      copyFileSync(
        require('path').join(__dirname, 'node_modules', 'grid_render', 'grid_render_bg.wasm'),
        require('path').join(__dirname, 'dist', 'grid_render_bg.wasm')
      );
    },
    (a, b) => {
      copyFileSync(
        require('path').join(__dirname, 'node_modules', 'grid_render', 'grid_render.js'),
        require('path').join(__dirname, 'dist', 'grid_render.js')
      );
    },
    (a, b) => {
      copyFileSync(
        require('path').join(__dirname, 'bqsql_parser', 'pkg', 'bqsql_parser_bg.wasm'),
        require('path').join(__dirname, 'dist', 'bqsql_parser_bg.wasm')
      );
    },
    (a, b) => {
      copyFileSync(
        require('path').join(__dirname, 'bqsql_parser', 'pkg', 'bqsql_parser.js'),
        require('path').join(__dirname, 'dist', 'bqsql_parser.js')
      );
    },
    // copyFileSync('', '')
  ],
  infrastructureLogging: {
    level: "log", // enables logging required for problem matchers
  },
};
module.exports = [extensionConfig];