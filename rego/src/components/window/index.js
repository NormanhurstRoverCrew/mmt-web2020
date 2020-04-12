import React from "react";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";
import CssBaseline from "@material-ui/core/CssBaseline";
import {MuiThemeProvider, createMuiTheme} from "@material-ui/core/styles";
import {MuiPickersUtilsProvider} from "@material-ui/pickers";
import MomentUtils from "@date-io/moment";


// import Topbar from "components/window/topbar/";
// import Sidebar from "components/window/sidebar/";
import Content from "components/window/Content";

const theme = createMuiTheme({
	palette: {
		type: "light", // Switching the dark mode on is a single property value change.
		primary: {
			main: "#aa6700",
			contrastText: "#fff",
		},
		secondary: {
			main: "#e18900",
			contrastText: "#fff",
		},
		text: {
			primary: "rgba(0, 0, 0, 1.0)",
			secondary: "rgba(0, 0, 0, 0.87)",
			disabled: "rgba(0, 0, 0, 0.38)",
			hint: "rgba(0, 0, 0, 0.48)",
		},
	},
	typography: {
		useNextVariants: true,
		fontFamily: [
			"OpenSans",
			"Arial",
			"Helvetica",
			"sans-serif",
		].join(","),
		body2: {
			lineHeight: "26px",
		},
		h1: {
			color: "#fff",
		},
		h2: {
			color: "#fff",
		},
		h3: {
			color: "#fff",
		},
		h4: {
			color: "#fff",
		},
		button: {
			color: "#fff",
			fontFamily: ["OpenSans", "Roboto", "Helvetica", "Arial", "sans-serif"].join(","),
			fontSize: "1.15em",
			lineHeight: "normal",
			letterSpacing: "normal",
			fontWeight: "bold",
			borderRadius: "0",
		},
	},
});

export class Root extends React.Component {
	constructor(props) {
		super(props);

		this.apiTimer;

		const {classes} = this.props;
		this.classes = classes;
	}

	render() {
		return (
			<div className={classNames(this.classes.root)}>
				<MuiThemeProvider theme={theme}>
					<MuiPickersUtilsProvider utils={MomentUtils}>
						<CssBaseline />
						<Content />
					</MuiPickersUtilsProvider>
				</MuiThemeProvider>
			</div>
		);
	}
}

const styles = (theme) => ({
	root: {
		position: "absolute",
		top: 0,
		left: 0,
		display: "flex",
		height: "100vh",
		width: "100vw",
	},
});

export default withStyles(styles)(Root);
