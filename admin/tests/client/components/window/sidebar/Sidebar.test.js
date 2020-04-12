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

import { Sidebar } from "components/window/sidebar/";
import { styles } from "components/window/sidebar/"
import { withStyles, IconButton } from '@material-ui/core';
import { windowActions } from '../../../../../client/actions/window.actions';
import { mapDispatchToProps, mapStateToProps } from '../../../../../client/components/window/sidebar';

let component;
let instance;
let store;

describe("component window/sidebar/<index />", () => {
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
            <Sidebar
                classes={{ root: {} }}
                toggleSideBar={toggleSideBar} />
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
            component.find("#closeSidebar").simulate('click')
            assert(toggleSideBar.calledOnce)
        })
    })
    
    describe("mapDispatchToProps", () => {
        let dispatch;
        beforeEach(() => {
            dispatch = sinon.stub();
            store = mockStore({});
        })

        it("should map to dispatch => windowActions.toggleSideBar()", () => {
            const props = mapDispatchToProps(store.dispatch)
            props.toggleSideBar()
            const actions = store.getActions()
            expect(actions[0]).to.eql({ type: 'WINDOW:SIDEBAR:TOGGLE' } )
        })
    })

    describe("mapStateToProps", () => {
        beforeEach(() => {
            store = mockStore({
                window: {
                    sidebar: {
                        open: sinon.spy()
                    }
                }
            });
        })

        it("should map state to props: window.sidebar.open => props.isSideBarOpen", () => {
            const state = mapStateToProps(store.getState())

            expect(state.isSideBarOpen).to.equal(store.getState().window.sidebar.open)
        })
    })
});