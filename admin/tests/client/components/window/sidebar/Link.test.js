import chai, { assert, expect } from 'chai';
var should = chai.should();
import { shallow, mount } from 'enzyme';
import sinon from 'sinon';

import React from "react";
import { Provider } from "react-redux";
import moxios from 'moxios';
import configureStore from 'redux-mock-store'
import { initAPI } from 'lib/net/api';

const mockStore = configureStore();

import fakes from 'tests/client/fakes'

import { Link } from "components/window/sidebar/Link";
import { styles } from "components/window/sidebar/Link"
import { withStyles, IconButton } from '@material-ui/core';
import { windowActions } from '../../../../../client/actions/window.actions';
import { mapDispatchToProps, mapStateToProps } from '../../../../../client/components/window/sidebar';

let component;
let instance;
let store;

class Test extends React.Component {
    render() {
        return (<div id="link-test"></div>)
    }
}

describe("component window/sidebar/<Link />", () => {
    describe("children", () => {
        beforeEach(async () => {
            moxios.install()
            initAPI(fakes.browser.window);
    
            component = shallow(
                <Link>
                    <Test></Test>
                </Link>
            );
    
            await component.update();
            instance = component.instance()
        })
    
        afterEach(function () {
            // import and pass your custom axios instance to this method
            moxios.uninstall()
            component.unmount();
        })

        it("should be rendered", () => {
            expect(component.find("#link-test").first())
        })

        it("should push to the history when .buttonClicked()", () => {
                        
        })
    })
});