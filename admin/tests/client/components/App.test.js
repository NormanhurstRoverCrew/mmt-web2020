import chai, { assert, expect } from 'chai';
var should = chai.should();
import { shallow, mount } from 'enzyme';

import sinon from 'sinon'

import React from "react";
import { Provider } from "react-redux";
import { create } from "react-test-renderer";
import moxios from 'moxios';
import configureStore from 'redux-mock-store'
import thunk from 'redux-thunk';
import { initAPI } from 'lib/net/api';

const mockStore = configureStore([thunk]);

import fakes from 'tests/client/fakes'

import { App } from "components/App"
import { MemoryRouter } from 'react-router'
import { auth0Actions } from 'actions/'
import axios from 'axios';
import Root from 'components/window/'
import { Callback } from 'components/auth/Callback';

let component;
let instance;

describe("App component", () => {
    describe("Test Routes", () => {
        let loadState;
        beforeEach(async () => {
            moxios.install(axios)

            const store = mockStore({
                auth: {
                    access_token: "something",
                    expires_at: new Date().getTime() + 6000,
                    isAuthenticated: true // used in / (root)
                },
                window: {
                    sidebar: {
                        open: true
                    }
                },
                bookings: []
            });

            fakes.browser.location.href = "http://localhost/"

            loadState = sinon.stub(auth0Actions, 'loadState')
            loadState.returns({ type: 'TEST:LOAD:STATE' })

            component = mount(
                <Provider store={store}>
                    <MemoryRouter
                        initialEntries={["/"]}
                        initialIndex={0}
                    >
                        <App history={fakes.history}
                            browser={fakes.browser} />
                    </MemoryRouter>
                </Provider>
            );
            await component.update();
            instance = component.instance()

            // Get rid of the MemoryRouter wrap
            // component = component.children().first();        
        })

        afterEach(function () {
            // import and pass your custom axios instance to this method
            moxios.uninstall()
            component.unmount();
            loadState.restore()
        })

        describe("/callback", () => {
            it("should show <Callback />'", async () => {
                //go to the callback route
                component.find(MemoryRouter).instance().history.push("/callback")
                await component.update()
                expect(component.find(Callback).length).to.equal(1);
            });
        });

        describe("/ (root)", () => {
            describe("AuthenticationBroker should block/allow auth", () => {
                [true, false].forEach((state) => {
                    it("when auth.isAuthenticated == " + state, () => {
                        const store = mockStore({
                            auth: {
                                expires_at: new Date().getTime() + 6000,
                                isAuthenticated: state
                            },
                            window: {
                                sidebar: {
                                    open: true
                                }
                            },
                            bookings: []
                        });

                        let component = mount(
                            <Provider store={store}>
                                <MemoryRouter
                                    initialEntries={["/"]}
                                    initialIndex={0}
                                >
                                    <App history={fakes.history}
                                        browser={fakes.browser} />
                                </MemoryRouter>
                            </Provider>
                        );

                        instance = component.instance()

                        if (state) {
                            expect(component.find("#authentication-broker").hasClass("authenticed"), "Not displaying div with authenticated children")
                        } else {
                            expect(component.find("#authentication-broker").hasClass("un-authenticed"), "Not displaying un-authenticated div")
                        }

                        component.unmount()
                    })
                })
            })

            it("should show window/<Root />", () => {
                component.find(MemoryRouter).instance().history.push("/")
                component.update()
                expect(component.find(Root).length).to.equal(1);
            });
        });
    });
});