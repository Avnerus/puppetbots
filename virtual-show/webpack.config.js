const path = require("path");
const webpack = require('webpack');

module.exports = (env) => {
  return {
    entry: './client',
    devtool: 'inline-source-map',
    resolve: {
      extensions: [ '.tsx', '.ts', '.js' ],
    },
    output: {
      path: path.resolve(__dirname, "public"),
      publicPath: '/',
      filename: "bundle.js",
    },
    mode: "development",
    devServer: {
        contentBase: path.join(__dirname, "public"),
        compress: true,
        port: 9000,
        hot: true
    },
    plugins: [
       new webpack.HotModuleReplacementPlugin(),
       new webpack.ProvidePlugin({ adapter: ['webrtc-adapter', 'default'] }),
       new webpack.EnvironmentPlugin({
          ENV: env.production ? 'production' : 'development'
      })
    ],
    module: {
        rules: [
          {
            test: /\.tsx?$/,
            use: 'ts-loader',
            exclude: /node_modules/,
          },
          {
            test: /\.js$/,
            exclude: /(node_modules|bower_components|wasm)/,
            use: {
              loader: 'babel-loader',
              options: {
                presets: ['@babel/env']
              }
            }
          },
         {
           test: /\.css$/,
           use: ['style-loader', 'css-loader']
         },
         {
            test: /\.scss$/,
            use: ['style-loader', 'css-loader', 'sass-loader']
         },
         {
                test: /\.(svg|png|jpg)$/,
                use: {
                  loader: 'file-loader',
                  options: {
                    limit: 22000,
                    name: 'assets/[name]-[hash].[ext]'
                  }
                }
         },
        {
            test: require.resolve('janus-gateway'),
            loader: 'exports-loader',
             options: {
               exports: 'Janus',
             },
        }
      ]
    }
  };
} 
