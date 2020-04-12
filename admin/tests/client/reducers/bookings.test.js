import chai, { assert, expect } from 'chai';
var should = chai.should();

import bookings from 'store/reducers/bookings'

describe("reducers/bookings", () => {
	let _state;
	describe("BOOKINGS:ADD", () => {
		beforeEach(() => {
			_state = [{ id: 1 }]
		})

		it("should add the new booking", () => {
			let state = bookings(_state, {
				type: "BOOKINGS:ADD",
				booking: { id: 2 }
			})

			expect(state).to.deep.equal([{ id: 1 }, { id: 2 }]);
		})
	})
})