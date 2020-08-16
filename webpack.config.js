const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  const isDevelopment = argv.mode !== 'production';
  const features = argv.features && `-- --features ${argv.features}`;
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
      new CopyWebpackPlugin({
        patterns: [
            { from: './static', to: distPath }
        ]
      }),
      new WasmPackPlugin({
        crateDirectory: './',
        extraArgs: `--no-typescript ${features || ''}`,
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
