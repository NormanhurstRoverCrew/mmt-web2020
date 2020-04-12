import React, { Component } from "react";
import PropTypes from "prop-types";
import ApolloClient from "apollo-boost";
import { ApolloProvider } from "@apollo/react-hooks";
import BookingContextProvider from "context/BookingContext";
import { InMemoryCache } from "apollo-cache-inmemory";

import Root from "components/window";

const appollo = new ApolloClient({
	uri: "http://localhost:8082/graphql",
	cache: new InMemoryCache(),
});

export class App extends Component {
	constructor(props) {
		super(props);
	}

	render() {
		return (
			<ApolloProvider client={appollo}>
				<BookingContextProvider>
					<Root {...this.props} />
				</BookingContextProvider>
			</ApolloProvider>
		);
	}
}

App.propTypes = {
	history: PropTypes.object.isRequired,
};

export default App;
