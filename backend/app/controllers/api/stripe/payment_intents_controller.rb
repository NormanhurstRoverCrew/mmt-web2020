class Api::Stripe::PaymentIntentsController < ApplicationController
  def index
    pis = Stripe::PaymentIntent.list(customer: params[:customer_id])

    render json: pis
  end

  def show
    pi = Stripe::PaymentIntent.retrieve(params[:id])

    render json: pi
  end
end
