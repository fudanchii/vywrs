const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  const isDevelopment = argv.mode !== 'production';
  return {
    devServer: {
      contentBase: distPath,
      compress: !isDevelopment,
      port: 8000
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "vywrs.js",
      webassemblyModuleFilename: "vywrs.wasm"
    },
    plugins: [
      new CopyWebpackPlugin([
        { from: './static', to: distPath }
      ]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
      new MiniCssExtractPlugin({ filename: "style.css" })
    ],
    watch: isDevelopment,
    module: {
      rules: [
        {
          test: /\.scss$/,
          use: [
            { loader: MiniCssExtractPlugin.loader },
            'css-loader',
            'sass-loader'
          ]
        }
      ]
    }
  };
};