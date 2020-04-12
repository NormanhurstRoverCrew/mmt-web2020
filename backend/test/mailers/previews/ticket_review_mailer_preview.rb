# Preview all emails at http://localhost:3000/rails/mailers/ticket_invite_mailer
class TicketReviewMailerPreview < ActionMailer::Preview
    def send_ticket
        TicketReviewMailer.send_ticket(Ticket.find(17))
    end
end
