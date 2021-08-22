const HtmlWebPackPlugin = require("html-webpack-plugin");
const DefinePlugin = require("webpack").DefinePlugin;

const path = require("path");
module.exports = {
	entry: './client/index.js',
	output: {
		path: __dirname + '/dist',
		filename: 'bundle.js'
	},
	devtool: 'source-map',
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
	devServer: {
		historyApiFallback: true,
		hot: true,
		inline: true,
		disableHostCheck: true,
		port: 8080, // Defaults to 8080
		proxy: {
			'/api': {
				target: 'http://localhost:3000',
				secure: false
			}
		}
	},
	plugins: [
		new HtmlWebPackPlugin({
			template: "./client/index.html",
			filename: "./index.html"
		}),
		new DefinePlugin({
			'process.env.NODE_ENV': JSON.stringify('development' + process.env.HOST), // nested in object, requires quotes
		})
	],
	resolve: {
		modules: [
			path.resolve('./client'),
			path.resolve('./node_modules')
		]
	},
};
