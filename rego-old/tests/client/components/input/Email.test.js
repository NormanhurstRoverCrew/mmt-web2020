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

import Email from 'components/input/Email';

let component;
let instance;

describe("<Email/>", () => {
	let someValue;
	let fake;
	beforeEach(async () => {
		fake = sinon.fake();

		someValue = "joblo@gmail.com"

		component = mount(
			<Email value={someValue}
				onChange={fake}
			/>
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

	it("should set initial value", () => {
		expect(component.find("Input").props().value).to.equal(someValue)
	})

	it("should change the value when a key is pressed", async () => {
		expect(component.find("Input").props().value).to.equal(someValue)

		someValue = "joblow@gmail.com";

		component.find("input").simulate('change', { target: { value: someValue } });

		assert(fake.calledOnce)
	})

	it("should provide an error message if the email username is too short", () => {
		let invalidEmail = "a@gmail.com";
		component.find("input").simulate('change', { target: { value: invalidEmail } });
		expect(component.html()).contains("Your email username is too short");

		invalidEmail = "ab@gmail.com";
		component.find("input").simulate('change', { target: { value: invalidEmail } });
		expect(component.html()).contains("Your email username is too short");
	})

	it("should provide an error message if the email host is incorrect", () => {
		let invalidEmail = "abcdef@g.c";
		component.find("input").simulate('change', { target: { value: invalidEmail } });

		expect(component.html()).contains("Your email host is not correct");
	})
});