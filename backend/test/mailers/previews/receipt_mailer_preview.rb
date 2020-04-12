# Preview all emails at http://localhost:3000/rails/mailers/receipt_mailer
class ReceiptMailerPreview < ActionMailer::Preview
  def eft
    ReceiptMailer.eft(Payment.new(booking: Booking.last, method: "eft", amount: 30.00))
  end
end
