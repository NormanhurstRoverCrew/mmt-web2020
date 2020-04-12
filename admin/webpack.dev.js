const path = require("path");
const merge = require('webpack-merge');

const common = require('./webpack.common.js');

module.exports = merge(common, {
	entry: {
		main: './client/index.js',
	},
	mode: 'development',
	devtool: 'inline-source-map',
	devServer: {
		host: '0.0.0.0',
		historyApiFallback: true,
		hot: true,
		inline: true,
		disableHostCheck: true,
		quiet: false,
		port: 8081 // Defaults to 8080 => this needs to be the external(host not docker) port
	},
});
