# Preview all emails at http://localhost:3000/rails/mailers/ticket_notification_mailer
class TicketNotificationMailerPreview < ActionMailer::Preview
  def notify
    TicketNotificationMailer.notify(Ticket.last)
  end
end
