class Booking < UniqueRecord
  has_many :tickets
  has_many :payments
  has_many :stripe_payment_intent
  has_one :user
  has_one :payment_method_proposal

  def create_tickets(n)
    n.times do
      self.tickets.create
    end
  end

  def obj
    user = self.user
    tickets = []

    self.tickets.each do |ticket|
      tickets << ticket.obj
    end

    return {
             id: self.id,
             uid: self.uid,
             user: user,
             tickets: tickets,
             created_at: self.created_at,
             updated_at: self.updated_at,
             payment_method: self.payment_method,
           }
  end

  def payment_method
    self.payment_method_proposal ? self.payment_method_proposal.method : ""
  end

  def payment_method= method
    if self.payment_method_proposal
      self.payment_method_proposal.update(method: method)
    else
      self.payment_method_proposal = PaymentMethodProposal.create(method: method)
    end
  end

  def ticket_price
    self.payment_method_proposal ? self.payment_method_proposal.price : 40.00
  end

  STRIPE_ENABLED = false

  def collect_payments
    if STRIPE_ENABLED
      stripe_charges = Stripe::Charge.list()
    end

    out = []
    self.payments.each do |payment|
      out << {
        method: payment.method,
        amount: payment.amount,
      }
    end

    if STRIPE_ENABLED
      self.stripe_payment_intent.each do |payment|
        stripe_payments = stripe_charges.select do |sc|
          sc.payment_intent == payment.pi_id
        end

        if stripe_payments.length > 0
          out << {
            method: "stripe",
            amount: stripe_payments[0].amount / 100.0,
          }
        end
      end
    end

    return out
  end

  def paid?
    nTickets = self.tickets.length
    self.amount_paid >= (nTickets * 30.0)
  end

  def due
    nTickets = self.tickets.length
    (nTickets * 30.0) - self.amount_paid
  end

  def amount_paid
    nTickets = self.tickets.length
    payments = collect_payments

    totalPaid = 0.0

    payments.each do |payment|
      totalPaid += payment[:amount]
    end

    totalPaid
  end
end
