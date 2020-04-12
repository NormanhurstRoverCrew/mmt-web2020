const path = require("path");
const webpack = require("webpack");
const DefinePlugin = webpack.DefinePlugin;
const HtmlWebPackPlugin = require("html-webpack-plugin");
const MomentLocalesPlugin = require('moment-locales-webpack-plugin');

module.exports = {
	output: {
		path: __dirname + '/dist-client',
		filename: '[name].js',
		publicPath: "/"
	},
	module: {
		rules: [
			{
				test: /\.jsx?$/,
				exclude: /node_modules/,
				use: {
					loader: "babel-loader",
					options: {
						presets: [
							["@babel/preset-env", {
								"targets": { node: "10" }
							}],
							"@babel/preset-react"
						],
						plugins: [
							["module-resolver", {
								"root": ["./client/**", "./tests/**"],
							}]
						]
					}
				}
			},
			{
				test: /\.html$/,
				use: [
					{
						loader: "html-loader"
					}
				]
			},
			{
				test: /\.scss$/,
				loaders: [
					"style-loader",
					"css-loader",
					"sass-loader"
				]
			},
			{
				test: /\.(gif|png|jpe?g|svg)$/i,
				use: [
					'file-loader',
					{
						loader: 'image-webpack-loader',
						options: {
							bypassOnDebug: true, // webpack@1.x
							disable: true, // webpack@2.x and newer
						},
					},
				],
			}
		]
	},
	plugins: [
		new HtmlWebPackPlugin({
			template: "./client/index.html",
			filename: "./index.html"
		}),
		new webpack.HashedModuleIdsPlugin(), // so that file hashes don't change unexpectedly
		new MomentLocalesPlugin({
			localesToKeep: ['en-AU'],
		}),
	],
	// resolve: {
	// 	modules: [
	// 		path.resolve('.'),
	// 		path.resolve('./client'),
	// 		path.resolve('./node_modules')
	// 	]
	// },
};
