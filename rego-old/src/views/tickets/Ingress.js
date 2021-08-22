import React, { Component } from "react";
import { withStyles } from "@material-ui/core/styles";

import queryString from "query-string";

export class Ingress extends Component {
    constructor(props) {
        super(props);
    }

    componentDidMount() {
        const {uid} = this.props.match.params;
        this.props.ingress(uid);
    }

    render() {
        return (
            <>
            </>
        );
    }
}

const styles = (theme) => ({
    root: {

    },
});

const mapStateToProps = (state) => {
    return {
    };
};

const mapDispatchToProps = (dispatch) => {
    return {
        home: () => dispatch(push("/")),
        ingress: (uid) => dispatch(ticketActions.ingressDecision(uid)),
    };
};


export default withStyles(styles)(Ingress);
