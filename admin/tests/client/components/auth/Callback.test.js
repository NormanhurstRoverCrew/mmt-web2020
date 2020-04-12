import chai, { assert, expect } from 'chai';
var should = chai.should();
import { shallow } from 'enzyme';
import sinon from 'sinon';

import React from "react";
import { create } from "react-test-renderer";
import moxios from 'moxios';
import configureStore from 'redux-mock-store'
import { initAPI } from 'lib/net/api';

const mockStore = configureStore();

import fakes from 'tests/client/fakes'

import { Callback, mapDispatchToProps } from "components/auth/Callback"
import axios from 'axios';

let component;
let instance;

describe("auth/Callback component", () => {
	beforeEach(async () => {
		moxios.install()

		initAPI(fakes.browser.window);
	})

	afterEach(function () {
		// import and pass your custom axios instance to this method
		moxios.uninstall()
		component.unmount();
	})


	describe("handleAuthentication", () => {
		beforeEach(async () => {
			const store = mockStore({});

			component = shallow(
				<Callback
					browser={fakes.browser}
					location={fakes.browser.window.location} />
			);

			await component.update();
			instance = component.instance()
		})

		afterEach(function () {
			component.unmount();
		})


		describe("no hash in url", () => {
			beforeEach(() => {
				fakes.browser.window.location.hash = ""

				component.setProps({ handleAuthentication: sinon.spy() })
			})

			afterEach(() => {
			})

			it("should dispatch a login => authorization to Auth0", async () => {
				instance.componentDidMount()
				assert(instance.props.handleAuthentication.notCalled, "handleAuthentication was called when hash empty/invalid")
			});
		})

		describe("no hash in url", () => {
			beforeEach(() => {
				fakes.browser.window.location.hash = "access_token=something&id_token=else"

				component.setProps({ handleAuthentication: sinon.spy() })
			})

			describe("propper hash in url", () => {
				it("should call handleAuthentication()", () => {
					instance.componentDidMount()
					assert(instance.props.handleAuthentication.calledOnce, "handleAuthentication not called when valid hash in url")
				})
			})
		});

		describe("invalid hash in url", () => {
			beforeEach(() => {
				fakes.browser.window.location.hash = "access_tken=so4mething&id_asdftoken=else&error=oh%20no%20somethingbadhashappened"

				component.setProps({ handleAuthentication: sinon.spy() })
			})

			describe("propper hash in url", () => {
				it("should dispatch a login => authorization to Auth0", () => {
					instance.componentDidMount()
					assert(instance.props.handleAuthentication.calledOnce, "handleAuthentication not called when valid hash in url")
				})
			})
		});
	})

	describe("mapDispatchToProps", () => {
		describe("handleAuthentication", () => {
			let dispatch;
			beforeEach(() => {
				dispatch = sinon.stub()
			})

			it("should dispatch auth0Actions.handleAuthentication()", () => {
				mapDispatchToProps(dispatch).handleAuthentication(fakes.browser.window, fakes.browser.window.history, fakes.browser.window.localStorage)

				assert(dispatch.calledOnce);
			})
		})
	})
});