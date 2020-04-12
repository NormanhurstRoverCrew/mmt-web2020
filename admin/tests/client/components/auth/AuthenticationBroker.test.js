import chai, { assert, expect } from 'chai';
var should = chai.should();
import { shallow } from 'enzyme';
import sinon from 'sinon';

import React, { Component } from "react";
import { create } from "react-test-renderer";
import moxios from 'moxios';
import configureStore from 'redux-mock-store'
import { initAPI } from 'lib/net/api';

const mockStore = configureStore();

import fakes from 'tests/client/fakes'

import { AuthenticationBroker, mapDispatchToProps, mapSToP } from "components/auth/AuthenticationBroker"
import axios from 'axios';

let component;
let instance;
let store;

class BasicComponent extends Component {
	render() {
		return (
			<div>this is a test component</div>
		)
	}
}

describe("auth/AuthenticationBroker component", () => {
	beforeEach(async () => {
		moxios.install()

		initAPI(fakes.browser.window);

		store = mockStore({
			isAuthenticated: false,
			expires_at: 42
		})

		component = shallow(
			<AuthenticationBroker
				browser={fakes.browser}
				loadAuth={sinon.spy()}
				logout={() => { }}
				isAuthenticated={store.getState().isAuthenticated}
				expires_at={store.getState().expires_at} >
				<BasicComponent />
			</AuthenticationBroker>
		);
	})

	afterEach(function () {
		// import and pass your custom axios instance to this method
		moxios.uninstall()
		component.unmount();
	})

	describe("calculateExpiry()", () => {
		beforeEach(async () => {
			store = mockStore({
				isAuthenticated: false,
				expires_at: new Date().getTime()
			})

			component.setProps({
				store,
				loadAuth: sinon.spy(),
				logout: sinon.spy(),
				isAuthenticated: store.getState().isAuthenticated,
				expires_at: store.getState().expires_at
			})

			await component.update();
			instance = component.instance()
		})

		afterEach(() => {
			component.unmount()
		})

		describe("expires_at > current time", () => {
			it("should return a positive number", () => {
				component.setProps({ expires_at: new Date().getTime() + 6000 })
				expect(instance.calculateExpiry()).to.be.greaterThan(3000)
			})
		})

		describe("expires_at < current time", () => {
			it("should return 0 (zero)", () => {
				component.setProps({ expires_at: new Date().getTime() - 6000 })
				expect(instance.calculateExpiry()).to.be.equal(0)
			})
		})
	})


	describe("mount", () => {
		var clock;
		let logout;
		beforeEach(() => {
			clock = sinon.useFakeTimers({
				now: new Date(),
				toFake: ['setTimeout', 'clearTimeout', 'setInterval', 'clearInterval']
			});

			store = mockStore({
				expires_at: new Date().getTime() + 6000,
				isAuthenticated: true
			});

			logout = sinon.spy()

			component.unmount()

			component = shallow(
				<AuthenticationBroker
					browser={fakes.browser}
					loadAuth={sinon.spy()}
					logout={logout}
					isAuthenticated={store.getState().isAuthenticated}
					expires_at={store.getState().expires_at} >
					<BasicComponent />
				</AuthenticationBroker>
			);
		})

		afterEach(function () {
			clock.restore()
			component.unmount()
		})

		it("should start a timer that fires at the expires_at time", async () => {
			// await component.update();
			clock.tick(1000)
			assert(logout.notCalled, "logout called before expires_at reached")
			clock.tick(6000)
			assert(logout.calledOnce, "logout not called once expires at reached")
		})

		it("when isAuthenticated == false: render an empty div", () => {
			store = mockStore({
				expires_at: new Date().getTime() + 6000,
				isAuthenticated: false
			});

			component = shallow(
				<AuthenticationBroker
					browser={fakes.browser}
					loadAuth={sinon.spy()}
					logout={logout}
					isAuthenticated={store.getState().isAuthenticated}
					expires_at={store.getState().expires_at} >
					<BasicComponent />
				</AuthenticationBroker>
			);

			expect(component.find(BasicComponent).length).to.be.eql(0)
		})
		it("when isAuthenticated == true: render all Child components", () => {
			store = mockStore({
				expires_at: new Date().getTime() + 6000,
				isAuthenticated: true
			});

			component = shallow(
				<AuthenticationBroker
					browser={fakes.browser}
					loadAuth={sinon.spy()}
					logout={logout}
					isAuthenticated={store.getState().isAuthenticated}
					expires_at={store.getState().expires_at} >
					<BasicComponent />
				</AuthenticationBroker>
			);

			expect(component.find(BasicComponent).length).to.be.eql(1)
		})
	});

	describe("mapDispatchToProps", () => {
		let dispatch;
		beforeEach(() => {
			dispatch = sinon.stub()
		})

		it("should dispatch auth0Actions.loadAuth()", () => {
			mapDispatchToProps(dispatch).loadAuth(fakes.browser.window, fakes.browser.window.localStorage)
			assert(dispatch.calledOnce);
		})

		it("should dispatch auth0Actions.login()", () => {
			mapDispatchToProps(dispatch).login(fakes.browser.window)
			assert(dispatch.calledOnce);
		})

		it("should dispatch auth0Actions.logout()", () => {
			mapDispatchToProps(dispatch).logout(fakes.browser.localStorage)
			assert(dispatch.calledOnce);
		})
	})

	describe("mapStateToProps", () => {
		let state;
		beforeEach(() => {
			state = {
				auth: {
					isAuthenticated: true,
					expires_at: new Date().getTime()
				}
			}
		})

		it("should return state/props isAuthenticated", () => {
			expect(mapSToP(state).isAuthenticated).to.equal(state.auth.isAuthenticated)
		})

		it("should return state/props expires_at", () => {
			expect(mapSToP(state).isAuthenticated).to.equal(state.auth.isAuthenticated)
		})
	})
});