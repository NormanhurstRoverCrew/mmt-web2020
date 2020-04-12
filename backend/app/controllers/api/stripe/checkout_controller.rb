class Api::Stripe::CheckoutController < ApplicationController
  SITE_URL = "https://f71db05b.ngrok.io"

  def create
    begin
      Booking.where(uid: params[:uid]).each do |booking|
        session = Stripe::Checkout::Session.create(
          payment_method_types: ["card"],
          line_items: [{
            name: "MMT19 Ticket",
            description: "Magical Mystery Tour 2019 Valhalla Ticket",
            amount: (30.87 * 100).round,
            currency: "aud",
            quantity: booking.tickets.length,
          }],
          customer_email: booking.user.email,
          client_reference_id: "MMT19-#{booking.id}",
          success_url: "#{SITE_URL}/stripe/success",
          cancel_url: "#{SITE_URL}/stripe/cancel",
        )

        if not booking.payment_method_proposal
          PaymentMethodProposal.create(method: "stripe", booking: booking)
        else
          booking.payment_method_proposal.update(method: "stripe")
        end

        stripe_pi = StripePaymentIntent.create(pi_id: session.payment_intent, booking: booking)

        if stripe_pi.persisted?
          render json: {sessionId: session.id}
        else
          render status: 500, json: {error: {msg: "Could not create PaymentIntent record"}}
        end

        return
      end
    rescue Stripe::CardError => e
      # Since it's a decline, Stripe::CardError will be caught
      body = e.json_body
      err = body[:error]

      puts "Status is: #{e.http_status}"
      puts "Type is: #{err[:type]}"
      puts "Charge ID is: #{err[:charge]}"
      # The following fields are optional
      puts "Code is: #{err[:code]}" if err[:code]
      puts "Decline code is: #{err[:decline_code]}" if err[:decline_code]
      puts "Param is: #{err[:param]}" if err[:param]
      puts "Message is: #{err[:message]}" if err[:message]
    rescue Stripe::RateLimitError => e
      # Too many requests made to the API too quickly
      puts e
    rescue Stripe::InvalidRequestError => e
      puts e
      # Invalid parameters were supplied to Stripe's API
    rescue Stripe::AuthenticationError => e
      puts e
      # Authentication with Stripe's API failed
      # (maybe you changed API keys recently)
    rescue Stripe::APIConnectionError => e
      puts e
      # Network communication with Stripe failed
    rescue Stripe::StripeError => e
      puts e
      # Display a very generic error to the user, and maybe send
      # yourself an email
    rescue => e
      puts e
      # Something else happened, completely unrelated to Stripe
    end
  end
end
