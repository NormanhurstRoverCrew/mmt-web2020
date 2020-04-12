class Api::BookingsController < ApplicationController
  def index
    bookings = []
    # Booking.all.each do |booking|
    #   bookings << booking.obj
    # end

    b = Booking.all.load.to_a
    t = Ticket.all.load.to_a
    u = User.all.load.to_a

    b.each do |booking|
      tickets = []
      t.select { |t| t.booking_id == booking.id }.each do |ticket|
        user = u.select { |u| u.ticket_id == ticket.id }
        user = user.length > 0 ? user[0] : {}
        tickets << {
          id: ticket.id,
          uid: ticket.uid,
          user: user,
          created_at: ticket.created_at,
          updated_at: ticket.updated_at,
        }
      end

      user = u.select { |u| u.booking_id == booking.id }
      user = user.length > 0 ? user[0] : {}

      bookings << {
        id: booking.id,
        uid: booking.uid,
        user: user,
        tickets: tickets,
        created_at: booking.created_at,
        updated_at: booking.updated_at,
        payment_method: booking.payment_method,
      }
    end

    render json: bookings
  end

  def show
    Booking.where(uid: params[:id]).each do |booking|
      render json: booking.obj
      return
    end
    render status: 404, json: {}
  end

  def create
    # set quick_add to true if you dont want to validate input
    quick_add = params[:quick_add] || false
    new_user = params.require(:user).permit(:name, :crew, :email, :mobile)

    user = User.new(new_user)

    errors = {}
    if not user.valid?
      if user.errors[:email].any?
        if user.errors[:email].first == "has already been taken"
          errors[:email] = "A booking with this email has been created already. If you have lost your previous registration, please contact Grant and he will fix things up for you."
        else
          errors[:email] = user.errors[:email].first
        end
      end
      if user.errors[:name].any?
        errors[:name] = user.errors[:name].first
      end
    end
    if new_user[:name].strip.length < 4
      errors[:name] = "Name is too short"
    end
    if (not quick_add) and new_user[:crew].length < 5
      errors[:crew] = "Crew is too short"
    end
    if (not quick_add) and new_user[:email].strip.length < 5
      errors[:email] = "Email is too short"
    end
    if new_user[:mobile].strip.length == 0
      errors[:mobile] = "Mobile is too short"
    end
    if not errors.empty?
      render json: {errors: {booking: errors}}
      return
    end

    if quick_add then 
      user.email_verified = true
    end

    Booking.create do |booking|
      booking.save
      booking.create_tickets (params[:tickets] ? params[:tickets].to_i : 1) #default to 1 if p[:tickets] is not specified
      ticket = booking.tickets.first

      user.save!

      ticket.user = user
      booking.user = ticket.user

      if ticket.save
        if booking.save

          #send the verification email
          #TODO user a queuing system instead of instant delivery
          if booking.user.email.present?
            NewBookingMailer.verify(booking).deliver_later
          end

          puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} BOOKING:CREATE ID:#{booking.id} UID:#{booking.uid}"
          render status: 201, json: booking.obj
          return
        end
      end
    end
  rescue ActiveRecord::RecordNotSaved => e
    puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} BOOKING:CREATE RecordNotSaved"
    errors[:email] = "A booking with this email has been created already. If you have lost your previous registration, please contact Grant and he will fix things up for you."
    render json: {errors: {booking: errors}}
  end

  def update
    Booking.where(uid: params[:id]).each do |booking|
      if params[:method]
        pmp = booking.payment_method_proposal
        if pmp
          pmp.update method: params[:method]
        else
          PaymentMethodProposal.create booking: booking, method: params[:method]
        end
        puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} BOOKING:METHOD:UPDATE ID:#{booking.id} UID:#{params[:id]} METHOD:#{params[:method]}"
        render status: :ok, json: {}
        return
      end
    end
    puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} BOOKING:METHOD:UPDATE NOT FOUND UID:#{params[:id]}"
    render status: 404, json: {}
  end

  def verify
    uid = params[:booking_id]
    code = params[:code]

    Booking.where(uid: uid).each do |booking|
      user = booking.user
      if user.code == code
        user.update(email_verified: true)

        TicketInviteMailer.send_ticket(user.ticket).deliver_later(wait: 5.minutes)

        puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} BOOKING:VERIFY ID:#{booking.id} UID:#{params[:uid]} EMAIL:#{user.email}"
        render json: booking.obj
      else
        puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} BOOKING:VERIFY FAILED UID:#{params[:booking_id]}"
        render json: {errors: {general: "Code was inccorect. Could not validate User"}}
      end
    end
  end

  def destroy
    Booking.where(uid: params[:id]).each do |booking|
      booking.tickets.each do |ticket|
        ticket.user.destroy if ticket.user
        ticket.destroy
      end
      booking.destroy
      puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} BOOKING:DESTROY ID:#{booking.id}"
      render json: {}
    end
  end
end
