class Api::TeamsController < ApplicationController
    def index
        teams = Team.all

        if params[:with_tickets]
            newTeams = []
            teams.eager_load(:tickets).each do |team|
                tickets = []
                team.tickets.each do |ticket|
                    tickets << ticket.uid
                end
                newTeam = team.as_json
                newTeam[:tickets] = tickets
                newTeams << newTeam
            end
            render json: newTeams
            return
        end

        render json: teams
    end

    def show
        team = Team.find(params[:id].to_i)
        if team
            render json: team
            return
        end
    end

    def create
        render json: Team.create(params.require(:team)
            .permit(:name, :registration))
    end

    def update
        team = Team.find(params[:id].to_i)
        if team then
            if params[:team]
                team.update(
                    params.require(:team)
                    .permit(:name, :registration)
                )
            end
            render json: team
        end
    end

    def add_ticket
        team = Team.find(params[:team_id].to_i)
        ticket = Ticket.where(uid: params[:uid])
        team.tickets << ticket
    end
end
