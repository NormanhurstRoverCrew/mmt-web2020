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

import { Topbar } from "components/window/topbar/";
import { styles } from "components/window/topbar/"
import { withStyles, IconButton } from '@material-ui/core';
import { windowActions } from '../../../../../client/actions/window.actions';

let component;
let instance;
let store;

describe("component window/topbar/<index />", () => {
    let toggleSideBar;
    beforeEach(async () => {
        moxios.install()
        initAPI(fakes.browser.window);

        store = mockStore({
            window: {
                sidebar: {
                    open: true
                }
            }
        });

        toggleSideBar = sinon.stub()

        component = shallow(
            <Topbar
            classes={{root:{}}}
            toggleSideBar={toggleSideBar}/>
        );

        await component.update();
        instance = component.instance()
    })

    afterEach(function () {
        // import and pass your custom axios instance to this method
        moxios.uninstall()
        component.unmount();
    })

    describe("when the Menu Toggle hamburger is clicked", () => {
        it("should call props.toggleSideBar()", () => {
            //do click
            component.find("#openSidebar").simulate('click')
            assert(toggleSideBar.calledOnce)
        })
    })
});