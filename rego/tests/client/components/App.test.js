import chai, { assert, expect } from 'chai';
var should = chai.should();
import { shallow, mount } from 'enzyme';

import sinon from 'sinon'

import React from "react";
import moxios from 'moxios';
import configureStore from 'redux-mock-store'
import { MemoryRouter } from 'react-router'

const mockStore = configureStore();

import fakes from 'tests/client/fakes'

import { App } from "components/App"
import Root from "components/window/index";

let component;
let instance;

describe("App component", () => {
	beforeEach(async () => {
		const store = mockStore({});

		fakes.browser.location.href = "http://localhost/"

		component = shallow(
			<App history={fakes.history}
				browser={fakes.browser} />
		);
		await component.update();
		instance = component.instance()

		// Get rid of the MemoryRouter wrap
		// component = component.children().first();        
	})

	afterEach(function () {
		// import and pass your custom axios instance to this method
		component.unmount();
	})

	it("should render <Root />", () => {
		expect(component.contains(
		<Root history={fakes.history}
			browser={fakes.browser}/>
		)).to.be.true;
	})
});