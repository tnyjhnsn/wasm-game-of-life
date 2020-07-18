const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === "production",
      port: 8080
    },
    entry: "./bootstrap.js",
    output: {
      path: distPath,
      filename: "game-of-life.js",
      webassemblyModuleFilename: "game-of-life.wasm"
    },
    module: {
      rules: [
        {
          test: /\.css$/,
          use: [
            "style-loader",
            "css-loader"
            //"postcss-loader"
            //{
            //loader: "postcss-loader",
            //options: {
            //ident: "postcss",
            //plugins: [
            //require("tailwindcss"),
            //require("autoprefixer"),
            //require("@fullhuman/postcss-purgecss")({
            //content: [
            //"./src/**/*.rs",
            //"./dist/**/*.js",
            //"./static/index.html"
            //],
            //defaultExtractor: content => {
            //return content.match(/[A-z0-9-_:\/]+/g) || [];
            //}
            //})
            //]
            //}
            //}
          ]
        }
      ]
    },
    plugins: [
      new CopyWebpackPlugin([{ from: "./static", to: distPath }]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript"
      })
    ],
    watch: argv.mode !== "production"
  };
};
