const path = require("path");
const merge = require('webpack-merge');

const common = require('./webpack.common.js');

module.exports = merge(common, {
	entry: {
		main: './client/index.js',
		views: './client/views/index.js',
		store: './client/store/index.js',
		actions: './client/actions/index.js',
	},
	output: {
		filename: '[name].[contenthash:8].js',
	},
	mode: 'production',
	devtool: 'source-map',
	optimization: {
		runtimeChunk: 'single',
		splitChunks: {
			chunks: 'all',
			maxInitialRequests: Infinity,
			minSize: 0,
			cacheGroups: {
				vendor: {
					test: /node_modules/,
					chunks: "initial",
					name: "vendor",
					priority: 10,
					enforce: true,
				},
			},
		},
	}
});
