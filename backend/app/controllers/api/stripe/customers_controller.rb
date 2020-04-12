class Api::Stripe::CustomersController < ApplicationController
    def index
        customers = Stripe::Customer.list(email: params[:email])

        render json: customers
    end

    def show
        customer = Stripe::Customer.retrieve(params[:id])

        render json: customer
    end
end
