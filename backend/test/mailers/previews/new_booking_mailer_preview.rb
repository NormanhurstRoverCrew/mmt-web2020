# Preview all emails at http://localhost:3000/rails/mailers/new_booking_mailer
class NewBookingMailerPreview < ActionMailer::Preview
  def verify
    NewBookingMailer.verify(Booking.last)
  end
end
