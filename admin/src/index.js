import React from 'react';
import {render} from 'react-dom';
import * as Sentry from '@sentry/browser';
import ApolloClient from 'apollo-boost';
import { ApolloProvider } from '@apollo/react-hooks';
import {BrowserRouter} from 'react-router-dom';

Sentry.init({
	dsn: 'https://b9b82efea5c44dcea771e159b491664f@sentry.io/1483117',
});

import axios from 'axios';

import App from 'components/App';

window.axios = axios;

import 'style.scss';

const apolloClient = new ApolloClient({
		uri: "http://localhost:8083/graphql",
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
