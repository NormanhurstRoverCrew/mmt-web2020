ARRIVED_BONUS = 10.0
LOKI_DEDUCTION = 10.0
CLUES_OPENED = 10.0

team_points = []

Team.all.eager_load(:point_logs).each do |team|
        team_total = 0.0

        team.point_logs.where(arrived:true).each do |p|
                case p.base
                when (1..3)
                        team_total += ARRIVED_BONUS
                when 10
                        team_total -= LOKI_DEDUCTION
                end
        end

        team.point_logs.where.not(points:nil).each do |p|
                team_total += p.points
        end

        team.point_logs.where.not(trivia:nil).each do |p|
                team_total += p.trivia
        end

        team.point_logs.where(clues:true).each do |p|
                team_total -= CLUES_OPENED
        end

        team_points << {id: team.id, points: team_total}
end

team_points = team_points.sort_by { |k| k[:points] }.reverse
team_points.each do |t|
        puts t.inspect
end
