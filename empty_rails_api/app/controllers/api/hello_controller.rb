class Api::HelloController < ApplicationController
	def index
		render json: {msg: "Hello, World!"}
	end
end
