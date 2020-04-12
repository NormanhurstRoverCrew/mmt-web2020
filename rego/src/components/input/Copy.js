import React, {Component, createRef} from "react";
import {withStyles} from "@material-ui/core/styles";
import classNames from "classnames";

class Copy extends Component {
	constructor(props) {
		super(props);
		this.myRef = createRef();
	}

	render() {
		const {classes, text} = this.props;
		return (
			<textarea
				rows="1"
				type="text"
				ref={this.myRef}
				className={classNames(classes.copy)}
				value={text}
				onChange={(e)=>{}}
				onClick={(e) => {
					this.myRef.current.select();
					document.execCommand("copy");
					e.target.focus();
				}} />
		);
	}
}

const styles = (theme) => ({
	copy: {
		"width": "-webkit-fill-available",
		"-webkit-appearance": "none",
		"resize": "none",
		"cursor": "point",
		"border": "none",
		"padding": 0,
		"outline": "none",
		"fontFamily": theme.typography.fontFamily,
		"fontSize": "1em",
		fontWeight: "bold",
	},
});


export default (withStyles(styles)(Copy));
