const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  entry: './src/bootstrap.js',
  devtool: 'inline-source-map',
  mode: "development",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.css$/,
        use: [
          "style-loader",
          "@teamsupercell/typings-for-css-modules-loader",
          {
            loader: "css-loader",
            options: {
              modules: {
                localIdentName: '[name]__[local]___[hash:base64:5]',}
              },
          }
        ]
      },
      {
        test: /\.scss$/,
        use: [
          "sass-loader"
        ]
      }
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js', '.css'],
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname + "/pw", // Define where the root of the rust code is located (where the cargo.toml file is located)
    }),
  ],
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
};
