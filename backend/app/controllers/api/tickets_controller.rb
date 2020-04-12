class Api::TicketsController < ApplicationController
  def index
    if params[:booking_id]
      Booking.where(uid: params[:booking_id]).each do |booking|
        tickets = []
        booking.tickets.each do |ticket|
          tickets << ticket.obj
        end
        render json: tickets
        return
      end
    elsif params[:team_id]
      tickets = []
      Ticket.where(team_id: params[:team_id]).each do |ticket|
        tickets << ticket.obj
      end
      render json: tickets
      return
    else
      tickets = []
      Ticket.all.each do |ticket|
        tickets << ticket.obj
      end
      render json: tickets
    end
  end

  def show
    if params[:byid]
      Ticket.where(id: params[:id]).each do |ticket|
        puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:SHOW TID:#{ticket.id} UID:#{params[:id]}"
        render json: ticket.obj
        return
      end
    else
      Ticket.where(uid: params[:id]).each do |ticket|
        puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:SHOW TID:#{ticket.id} UID:#{params[:id]}"
        render json: ticket.obj
        return
      end
    end
    puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:SHOW FAILED UID:#{params[:id]}"
    render status: 404, json: {error: "Could not find a ticet with that UID"}
  end

  def create
    Booking.where(uid: params[:booking_id]).each do |booking|
      booking.tickets.where(uid: params[:ticket_uid]).each do |ticket|
        ticket.user.update(params.require(:user).permit(:name, :crew, :email, :mobile))
        puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:CREATE BID:#{booking.id} TID:#{ticket.id}"
        render json: booking.obj
        return
      end
      booking.tickets.create do |ticket|
        ticket.user = User.create(params.require(:user).permit(:name, :crew, :email, :mobile))
        booking.save!
        if ticket.save
          puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:CREATE/UPDATE BID:#{booking.id} TID:#{ticket.id}"
          render status: 201, json: booking.obj
          return
        end
        puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:CREATE FAILED BID:#{booking.id}"
        render status: 400, json: {error: "Could not save the new ticket"}
        return
      end
      puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:CREATE FAILED BID:#{booking.id}"
      render status: 400, json: {error: "Could not create a new ticket"}
      return
    end
    puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:CREATE BOOKING NOT FOUND"
    render status: 400, json: {error: "The booking specified does not exist"}
    return
  end

  def update
    Ticket.where(uid: params[:id]).each do |ticket|
      if ticket.user.present?
        if params[:check]
          errors = {}
          updates = params.require(:user).permit(:name, :crew, :email, :mobile, :diet, :email_verified)
          puts updates

          name = updates[:name]
          name = name.gsub(/\s+/m, " ").strip.split(" ")
          if name.length < 2
            errors[:name] = "Please enter your FULL NAME"
          end

          crew = updates[:crew]
          if crew.length < 5
            errors[:crew] = "Please select a valid crew"
          end

          email = updates[:email]
          if email.length < 4
            errors[:email] = "Email too short."
          end
          if (email =~ /\A[\w+\-.]+@[a-z\d\-]+(\.[a-z\d\-]+)*\.[a-z]+\z/i).nil?
            errors[:email] = "Email not valid"
          end

          if not errors.empty?
            puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:UPDATE USER INPUT ERRORS TID:#{ticket.id} UID:#{params[:id]}"
            render json: {errors: errors}
            return
          else
            ticket.user.update updates
          end
        else
          ticket.user.update params.require(:user).permit(:name, :crew, :email, :mobile, :diet, :email_verified)
        end
      else
        ticket.user = User.create params.require(:user).permit(:name, :crew, :email, :mobile, :diet)
      end

      # send emails
      if params[:user][:email_verified] then
        TicketInviteMailer.send_ticket(ticket).deliver_later
      end

      puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:UPDATE TID:#{ticket.id} UID:#{params[:id]}"
      render json: {}
    end
  end

  def destroy
    Ticket.where(uid: params[:id]).each do |ticket|
      if ticket.user
        ticket.user.destroy
      end
      ticket.destroy
      puts "#{Time.zone.now.strftime("%y%m%d %H:%M")} TICEKT:DESTROY TID:#{ticket.id} UID:#{params[:id]}"
      render json: {}
    end
  end

  def verify
  end

  def sendEmail
    id = params[:id]
    uid = params[:ticket_id]

    ticket = Ticket.where(uid: uid).each do |ticket|
      case id.to_i
      when 1
        NewBookingMailer.verify(ticket.booking).deliver_later
      when 2
        TicketInviteMailer.send_ticket(ticket).deliver_later
      end
      render json: {}
      return
    end
    render json: {errors: {general: "Ticket does not exist"}}
  end
end
