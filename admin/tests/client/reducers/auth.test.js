import chai, { assert, expect } from 'chai';
var should = chai.should();

import auth from 'store/reducers/auth'

describe("reducers/auth", () => {
	let _state;
	describe("alters state, type:", () => {
		describe("AUTH:LOGIN", () => {
			beforeEach(() => {
				_state = {}
			})
			let setFields = [
				'access_token', 'id_token', 'expires_at', 'name', 'avatar', 'scopes', 'email', 'isAuthenticated'
			]
			setFields.forEach((field) => {
				it("should set: " + field, () => {
					let response = auth(_state, {
						type: "AUTH:LOGIN",
						auth: {
							accessToken: 'abcd',
							idToken: 'abcdefg',
							expiresIn: 7200,
							scope: 'profile',
							idTokenPayload: {
								nickname: 'apple',
								picture: 'http://google.com/images/random',
								email: 'joblo@gmail.com'
							}
						}
					})
					should.exist(response[field])
				})
			})
		})

		describe("AUTH:LOGIN:LOCAL", () => {
			beforeEach(() => {
				_state = {}
			})

			let setFields = [
				'access_token', 'id_token', 'expires_at', 'name', 'avatar', 'scopes', 'email', 'isAuthenticated'
			]

			setFields.forEach((field) => {
				it("should set: " + field, () => {
					let response = auth(_state, {
						type: "AUTH:LOGIN:LOCAL",
						auth: {
							access_token: 'abcd',
							id_token: 'abcdefg',
							expires_at: 7200,
							scopes: 'profile',
							name: 'apple',
							avatar: 'http://google.com/images/random',
							email: 'joblo@gmail.com'
						}
					})
					should.exist(response[field])
				})
			})
		})

		describe("AUTH:LOGOUT", () => {
			beforeEach(() => {
				_state = {
					access_token: 'a',
					id_token: 'b',
					name: 'c',
					avatar: 'd',
					scopes: 'e',
					email: 'f'
				}
			})

			let setFields = [
				'expires_at', 'isAuthenticated'
			]

			let unsetFields = [
				'access_token', 'id_token', 'name', 'avatar', 'scopes', 'email'
			]

			setFields.forEach((field) => {
				it("should set: " + field, () => {
					let response = auth(_state, {
						type: "AUTH:LOGOUT"
					})
					should.exist(response[field])
				})
			})

			unsetFields.forEach((field) => {
				it("should un-set: " + field, () => {
					let response = auth(_state, {
						type: "AUTH:LOGOUT"
					})
					should.not.exist(response[field])
				})
			})
		})
	})
})