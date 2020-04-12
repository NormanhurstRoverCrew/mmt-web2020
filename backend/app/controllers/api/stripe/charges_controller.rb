class Api::Stripe::ChargesController < ApplicationController
  def index
    if params[:booking_id]
      Booking.where(uid: params[:booking_id]).each do |booking|
        out = []

        booking.stripe_payment_intent.each do |pi|
          payment_intent = Stripe::PaymentIntent.retrieve(pi.pi_id)
          charges = payment_intent.charges.data
          if charges.any?
            charges.each do |charge|
              charge.amount /= 100.0
              charge.amount_refunded /= 100.0

              charge.booking = {
                uid: booking.uid,
                id: booking.id,
              }

              out << charge
            end
          end
        end

        render json: out
      end
    elsif params[:by_booking]
      bookings = Booking.includes(:stripe_payment_intent).where.not(stripe_payment_intents: {id: nil})
      out = bookings.map do |booking|
        {
          id: booking.id,
          uid: booking.uid,
          user: booking.user,
          created_at: booking.created_at,
          updated_at: booking.updated_at,
          payment_method: booking.payment_method,
        }
      end
      render json: out
      return
    else
      charges = Stripe::Charge.list(limit: params[:limit])

      charges = charges.map do |charge|
        charge.amount /= 100.0
        charge.amount_refunded /= 100.0

        StripePaymentIntent.where(pi_id: charge.payment_intent).each do |spi|
          booking = spi.booking

          charge.booking = {
            uid: booking.uid,
            id: booking.id,
          }
        end
        charge
      end

      render json: charges
    end
  end
end
