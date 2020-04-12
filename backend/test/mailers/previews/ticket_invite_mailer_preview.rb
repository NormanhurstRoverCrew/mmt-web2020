# Preview all emails at http://localhost:3000/rails/mailers/ticket_invite_mailer
class TicketInviteMailerPreview < ActionMailer::Preview
    def send_ticket
        TicketInviteMailer.send_ticket(Ticket.last)
    end
end
