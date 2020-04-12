class NewBookingMailer < ApplicationMailer
  def verify(booking)
    base_url = (ENV["RAILS_ENV"] == "development") ? "http://localhost:8080" : "https://mmt.***REMOVED***"

    @user = booking.user
    @uid = booking.uid

    @verify_url = "#{base_url}/verify?uid=#{@uid}&code=#{@user.code}"
    email = (ENV["RAILS_ENV"] == "development" || ENV["MAILGUN_DEV"]) ? "grant42perry@gmail.com" : @user.email

    puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} NEW_BOOKING_MAILER:VERIFY BID:#{booking.id} UID:#{booking.uid}"
    mail(to: "#{@user.name} <#{email}>", subject: "Please Verify your Email! | MMT Valhalla")
  end
end
