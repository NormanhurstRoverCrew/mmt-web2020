class TicketNotificationMailer < ApplicationMailer
  def notify(ticket)
    base_url = (ENV["RAILS_ENV"] == "development") ? "http://localhost:8080" : "https://mmt.***REMOVED***"
    if ticket
      @user = ticket.user
      if @user
        email = @user.email
        if email
          @url = "#{base_url}/ticket/#{ticket.uid}"
          puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICKET_NOTIFICATION_MAILER:NOTIFY TID:#{ticket.id} UID:#{ticket.uid}"
          mail(to: "#{@user.name} <#{email}>", subject: "Your MMT Ticket! Please fill out your details! | MMT Valhalla")
          return
        end
      end
    end
  end
end
