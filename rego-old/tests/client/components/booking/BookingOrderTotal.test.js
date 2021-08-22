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

import { BookingOrderTotal, mapStateToProps, styles } from 'components/booking/BookingOrderTotal';

let component;
let instance;

describe("<BookingOrderTotal/>", () => {
	describe("should render the price correctly for the number of tickets", () => {

		const testTickets = [1,15,16,3000];

		testTickets.forEach((i) => {
			describe ("should show the correct details for " + i + " ticket/s", () => {
				beforeEach(async () => {
					component = mount(
						<BookingOrderTotal tickets={i} />
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
				
				it("should show the right quantity", () => {
					expect(parseInt(component.find("TableCell#quantity").text())).to.equal(i);
				});
				
				it("should show the correct subtotal", () => {
					const ticketPrice = parseFloat(component.find("TableCell#ticketprice").text().split('$')[1])
					expect(parseFloat(component.find("TableCell#subtotal").text().split('$')[1])).to.equal(i*ticketPrice);
				})

				it("should show the correct total", () => {
					const ticketPrice = parseFloat(component.find("TableCell#ticketprice").text().split('$')[1])
					expect(parseFloat(component.find("TableCell#finaltotal").text().split('$')[1])).to.equal(i*ticketPrice);
				})
			})
		})
	})

	describe("when there are Zero(0) tickets", () => {
		beforeEach(async () => {
			component = mount(
				<BookingOrderTotal tickets={0} />
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

		it ("Should display an empty <div/>", () => {
			expect(component.contains(<div/>)).to.be.true
		})

	})
	
	describe("redux", () => {
		it("mapStateToProps should have the correct values", () => {
			const {tickets} = mapStateToProps({
				booking: {
					tickets: [{}, {}], //length 2
				}
			})

			expect(tickets).to.equal(2)
		})
	})
	
	describe("styles", () => {
		it("styles exists", () => {
			expect(styles()).to.not.be.null
		})
	})

});