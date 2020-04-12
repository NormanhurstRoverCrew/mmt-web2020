class CheckBookingPaidJob < ApplicationJob
  queue_as :default

  def perform(booking)
    total = 0.0
    booking.payments.each do |payment|
      total += payment.amount
    end

    due = 0.0
    booking.tickets.each do |ticket|
      due += ticket.price
    end

    if total >= due
      booking.tickets.each do |ticket|
        TicketNotificationMailer.notify(ticket).deliver_later(wait: 10.seconds)
      end
    else
      puts "Booking has not been paid: Needs $#{sprintf("%0.02f", (due - total))}"
    end
  end
end
