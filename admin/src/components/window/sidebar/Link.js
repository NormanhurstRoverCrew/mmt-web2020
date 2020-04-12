import React from "react";
import {withStyles} from "@material-ui/core/styles";
import { useHistory } from "react-router-dom";

import {ListItem, ListItemIcon, ListItemText} from "@material-ui/core";


export const Link = ({classes, to, onClick, icon, children, text}) => {
		const history = useHistory();

	const buttonClicked = (e) => {
		history.push(to);
		onClick && onClick(e);
	}
		return (
			<ListItem button
				onClick={buttonClicked}>
				<ListItemIcon>
					{icon}
				</ListItemIcon>
				<ListItemText primary={children || text} />
			</ListItem>
		);
};

export const styles = (theme) => ({
	root: {},
	a: {
		textDecoration: "none",
	},
});

export default withStyles(styles)(Link);
