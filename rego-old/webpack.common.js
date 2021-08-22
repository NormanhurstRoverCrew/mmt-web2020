const path = require("path");
const HtmlWebPackPlugin = require("html-webpack-plugin");

module.exports = {
	entry: {
		app: './client/index.js',
	},
	output: {
		publicPath: "/",
		path: __dirname + '/dist-client',
		filename: 'bundle.js'
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
					"sass-loader",
				]
			},
			{
				test: /\.(gif|png|jpe?g|svg)$/i,
				use: [{
					loader: 'file-loader',
					options: {
						name: '[name].[ext]',
						outputPath: 'client/img/'
					}
				}]
			},
			{
				test: /\.(woff(2)?|ttf|eot|otf)(\?v=\d+\.\d+\.\d+)?$/,
				use: [{
					loader: 'file-loader',
					options: {
						name: '[name].[ext]',
						outputPath: 'client/fonts/'
					}
				}]
			},
		]
	},
	plugins: [
		new HtmlWebPackPlugin({
			template: "./client/index.html",
			filename: "./index.html"
		}),
	],
	resolve: {
		modules: [
			path.resolve('./client'),
			path.resolve('./node_modules')
		]
	},
};