class Api::PaymentsController < ApplicationController
  def index
    bookings = []
    Booking.all.each do |booking|
      out = {
        id: booking.id,
        uid: booking.uid,
        user: booking.user,
        created_at: booking.created_at,
        updated_at: booking.updated_at,
        payment_method: booking.payment_method,
        tickets_quantity: booking.tickets.length,
      }
      out[:payments] = booking.collect_payments

      out[:payment_total] = out[:payments].inject(0) { |sum, payment| sum + payment[:amount] }

      out[:payment_remaining] = (booking.ticket_price * out[:tickets_quantity]) - out[:payment_total]
      bookings << out
    end
    render json: bookings
  end

  def paid
    Ticket.where(uid: params[:ticket_id]).each do |ticket|
      render json: {
        paid: ticket.booking.paid?,
        due: ticket.booking.due
      }
    end
  end

  def create
    Booking.where(uid: params[:booking_id]).each do |booking|
      booking.payments << Payment.create(params.require(:payment).permit([:method, :amount]))

      if (params[:send_receipt] == true)
        ReceiptMailer.eft(booking.payments.last).deliver_later
      end

      if not params[:quick_add] then
        CheckBookingPaidJob.perform_later(booking)
      end

      puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} PAYMENTS:CREATE BID:#{booking.id} UID:#{params[:booking_id]} METHOD:#{params[:method]} AMOUNT:#{params[:amount]}"
      render json: {}
      return
    end
  end
end
