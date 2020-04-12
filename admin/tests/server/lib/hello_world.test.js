import chai, {assert, expect} from 'chai';
var should = chai.should();
import sinon from 'sinon';

import HelloWorld from '../../../server/lib/hello_world'

describe("Hello World", () => {
	it ("should return a .msg greater than 20 characters long", () => {
		let req = {}
    // Have `res` have a send key with a function value coz we use `res.send()` in our func
    let res = {
      send: sinon.spy()
		}
		
		new HelloWorld(true).index(req, res);

		expect(res.send.calledOnce).to.be.true;

		expect(res.send.firstCall.args[0].msg.length >= 20).to.be.true;
	})

	it ("should return a .msg less than 20 characters long", () => {
		let req = {}
    // Have `res` have a send key with a function value coz we use `res.send()` in our func
    let res = {
      send: sinon.spy()
		}
		
		new HelloWorld(false).index(req, res);

		expect(res.send.calledOnce).to.be.true;

		expect(res.send.firstCall.args[0].msg.length < 20).to.be.true;
	})
})