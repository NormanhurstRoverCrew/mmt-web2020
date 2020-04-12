class Api::Stripe::EventsController < ApplicationController
  def index
    keys_to_extract = [:id, :created]
    events = Stripe::Event.list(limit: params[:limit] || 10)[:data]
    
    events = events.map { |e|
        {
            id: e.id,
            created: e.created,
            data: e.data,
            type: e.type,
        }
    }

    render json: events
  end

  def show
    e = Stripe::Event.retrieve(params[:id])

    case e.type 
    when "checkout.session.completed"
        puts "session creation"
    end
    

    render json: {
            id: e.id,
            created: e.created,
            data: e.data,
            type: e.type,
    }
  end
end
