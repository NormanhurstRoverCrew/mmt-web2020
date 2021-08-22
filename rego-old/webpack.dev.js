const path = require("path");
const merge = require('webpack-merge');

const common = require('./webpack.common.js');

module.exports = merge(common, {
	mode: 'development',
	devtool: 'inline-source-map',
	devServer: {
		host: '0.0.0.0',
		historyApiFallback: true,
		hot: true,
		inline: true,
		disableHostCheck: true,
		quiet: false,
		port: 8080,
		proxy: {
			'/api': {
				target: 'http://localhost:3000',
				secure: false
			},
			'/.well-known': {
				target: 'http://localhost:3000',
				secure: false
			}
		}
	},
});