class Api::PointLogsController < ApplicationController
    def index
        team_id = params[:team_id]
        if team_id
            team = Team.find(team_id.to_i)
            render json: team.point_logs
            return
        else
            render json: PointLog.all
            return
        end
    end

    def create
        team = Team.find(params[:team_id].to_i)
        if team then
            if params[:point_log]
                render json: team.add_log(
                    PointLog.new(params.require(:point_log)
                        .permit(:logged_at, :base, :admin, :arrived, :departed, :points, :trivia, :clues, :comment))
                )
                return
            end
        end
        render status: :bad_request, json: {}
    end
end
