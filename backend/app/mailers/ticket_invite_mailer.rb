class TicketInviteMailer < ApplicationMailer
    def send_ticket(ticket)
        @ticket = ticket
        @uid = ticket.uid
        base_url = (ENV["RAILS_ENV"] == "development") ? "http://localhost:8080" : "https://mmt.***REMOVED***"
        if ticket
            ticket.createQR

            attachments.inline["qr.png"] = File.read("#{Rails.root}/app/assets/images/qr/ticket/#{@uid}.png")

            attachments['MMT_E1.pdf'] = File.read("#{Rails.root}/public/MMT-2019-E1.pdf")

            @user = ticket.user
            if @user
                email = @user.email
                if email
                    if not ticket.email_log.any?
                        EmailLog.create(ticket: ticket, email_id: "ticket_invite_01")
                        @not_paid = !ticket.booking.paid?
                        @url = "#{base_url}/ticket/#{ticket.uid}"
                        puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICKET_INVITE_MAILER:NOTIFY TID:#{ticket.id} UID:#{ticket.uid}"
                        mail(to: "#{@user.name} <#{email}>", subject: "Your ticket to VALHALLA | MMT VALHALLA")
                        return
                    else
                        puts "Not sending \"Ticket Invite\" as it has alredy been sent"
                    end
                end
            end
        end
    end
end
