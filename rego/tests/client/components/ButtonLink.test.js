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

import ButtonLink from "components/ButtonLink";
import Root from "components/window/index";

let component;
let instance;

let locationTo = "/somelocation"
let blChildren = "This is the content of the child";

describe("ButtonLink component", () => {

	let fakeOnClick;

	describe("custom onClick()", () => {

		beforeEach(async () => {
			const store = mockStore({});

			fakeOnClick = sinon.fake();

			component = mount(
				<MemoryRouter store={store}>
					<ButtonLink to={locationTo} onClick={fakeOnClick} >
						<div>{blChildren}</div>
					</ButtonLink>
				</MemoryRouter>
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

		it("should render the children passed to it", () => {
			expect(component.contains(<div>{blChildren}</div>)).to.be.true
		})

		it("should render a <Link/>", () => {
			expect(component.exists("Link")).to.be.true
		})

		it("should have the correct path in the props for the <Link/>", () => {
			expect(component.find("Link").props()["to"]).to.equal(locationTo)
		})

		it("should .preventDefault when clicked", () => {
			// preventing default stops the click event from propogating up to higher levels and calling those. specifically not calling the Link...
			var fake = sinon.fake();

			component.find("ButtonLink").simulate('click', { preventDefault: fake })

			expect(fake.callCount).to.equal(1)
		})

		it("should call props.onClick() when clicked", () => {
			expect(fakeOnClick.callCount).to.equal(0)
			component.find("ButtonLink").simulate('click')
			expect(fakeOnClick.callCount).to.equal(1)
		})

	})
	
	describe("empty onClick()", () => {

		beforeEach(async () => {
			const store = mockStore({});

			component = mount(
				<MemoryRouter store={store}>
					<ButtonLink to={locationTo} >
						<div>{blChildren}</div>
					</ButtonLink>
				</MemoryRouter>
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

		it("does not prevent default", () => {
			var fake = sinon.fake();
	
			component.find("ButtonLink").simulate('click', { preventDefault: fake })
	
			expect(fake.callCount).to.equal(0)
		})

	})

});