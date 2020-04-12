class ReceiptMailer < ApplicationMailer
  def eft(payment)
    if payment
      booking = payment.booking
      if booking
        @user = booking.user
        if @user
          email = @user.email
          if email
            @payment = payment
            @payments = booking.payments
            puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} RECEIPT_MAILER:EFT BID:#{booking.id} UID:#{booking.uid}"
            mail(to: "#{@user.name} <#{email}>", subject: "Your Receipt! | MMT Valhalla")
            return
          end
        end
      end
    end
  end
end
