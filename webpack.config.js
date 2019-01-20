const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./js/index.js",
  output: {
    path: dist,
    filename: "bundle.js"
  },
  module: {
    rules: [
        {
            test: /\.(js|jsx)$/,
            exclude: /node_modules/,
            use: ['babel-loader']
        }
    ]
  },
   // resolve: {
   //     extensions: ['*', '.js', '.jsx']
  //},
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),

   new WasmPackPlugin({
     crateDirectory: path.resolve(__dirname, "crate")
   }),
  ]
};
