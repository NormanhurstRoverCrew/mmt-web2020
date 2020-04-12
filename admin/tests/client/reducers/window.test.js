import chai, { assert, expect } from 'chai';
var should = chai.should();

import window from 'store/reducers/window'

describe("reducers/window", () => {
	let _state;
	describe("WINDOW:SIDEBAR:TOGGLE", () => {
		beforeEach(() => {
			_state = {
				sidebar: {
					open: false,
				}
			}
		})

		it("should toggle the sidebar.open prop", () => {
			let state = window(_state, {
				type: "WINDOW:SIDEBAR:TOGGLE"
			})

			expect(state).to.deep.equal({
				sidebar: {
					open: true,
				}
			});
		})
	})
})