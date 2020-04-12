import React from "react";
import { render } from "react-dom";
import { BrowserRouter } from "react-router-dom";
import { StripeProvider } from "react-stripe-elements";
import ReactGA from "react-ga";
import * as Sentry from "@sentry/browser";

Sentry.init({
	dsn: "https://f74acf52d25e4e90a8ccbe050a061ab4@sentry.io/1483111",
});

import App from "./components/App";

import "./style.scss";
import norsewoff2 from "./fonts/Norse.woff2";
import norsewoff from "./fonts/Norse.woff";
import norseboldwoff2 from "./fonts/Norse-Bold.woff2";
import norseboldwoff from "./fonts/Norse-Bold.woff";

ReactGA.initialize("UA-96080461-6");

render(
		<BrowserRouter>
			{/* <StripeProvider apiKey="pk_test_3jpeKc8apKbPVGnnNoyY51GK00N6X69Wap"> */}
			<App history={history} />
			{/* </StripeProvider> */}
		</BrowserRouter>,
	document.getElementById("root")
);
