import React from "react";
import {Link} from "react-router-dom";

import Button from "@material-ui/core/Button";

const renderLink = React.forwardRef(
	(props, ref) => (
		<Link to={props.to}
			ref={ref}
			{...props}
		/>
	)
);


class ButtonLink extends React.Component {
	constructor(props) {
		super(props);

		this.doClick = this.doClick.bind(this);
	}


	doClick(e) {
		if (this.props.onClick) {
			e.preventDefault();

			this.props.onClick(e);
		}
	}

	render() {
		return (
			<Button component={renderLink}
				{...this.props}
				onClick={this.doClick}
			>
				{this.props.children}
			</Button>
		);
	}
}

export default ButtonLink;
