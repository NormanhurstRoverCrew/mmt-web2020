import React from 'react';
import {render} from 'react-dom';
import * as Sentry from '@sentry/browser';
import ApolloClient from 'apollo-boost';
import {ApolloProvider} from '@apollo/react-hooks';
import {BrowserRouter} from 'react-router-dom';
import Url from 'url-parse';

Sentry.init({
	dsn: 'https://b9b82efea5c44dcea771e159b491664f@sentry.io/1483117',
});

import axios from 'axios';

import App from 'components/App';

window.axios = axios;

import 'style.scss';

var graphql = new Url(window.location.href);
graphql.set('pathname', "graphql");
if (graphql.port == 8081) {
		graphql.set('port', 8083);
}
console.log(graphql.href);

const apolloClient = new ApolloClient({
	uri: graphql.href,
	request: operation => {
		const token = JSON.parse(localStorage.getItem('access_token'));
		operation.setContext({
			headers: {
				authorization: token ? `Bearer ${token}` : '',
			},
		});
	},
});

render(
	<BrowserRouter>
		<ApolloProvider client={apolloClient}>
			<App
				browser={{
					window: window,
					localStorage: localStorage,
					history: history,
				}}
			/>
		</ApolloProvider>
	</BrowserRouter>,
	document.getElementById('root'),
);
