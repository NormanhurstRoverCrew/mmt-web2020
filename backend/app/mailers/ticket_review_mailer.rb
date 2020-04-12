class TicketReviewMailer < ApplicationMailer
    def send_ticket(ticket)
        @ticket = ticket
        @uid = ticket.uid
        base_url = (ENV["RAILS_ENV"] == "development") ? "http://localhost:8080" : "https://mmt.***REMOVED***"
        if ticket
            @user = ticket.user
            if @user
                email = @user.email
                if email
                #     if not ticket.email_log.where(email_id: "ticket_review_01").any?
                #         EmailLog.create(ticket: ticket, email_id: "ticket_review_01")
                #         puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICKET_INVITE_MAILER:NOTIFY TID:#{ticket.id} UID:#{ticket.uid}"
                        mail(to: "#{@user.name} <#{email}>", subject: "MMT 2019 Thank you and feedback")
                        return
                #     else
                #         puts "Not sending \"Ticket Review\" as it has alredy been sent"
                #     end
                end
            end
        end
    end
end
