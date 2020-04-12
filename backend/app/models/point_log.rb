class PointLog < ApplicationRecord
    belongs_to  :team

    CLUE_DEDUCTION = 10.0

    def sum_points
        (self.points ? self.points.truncate(1) : 0) +
        (self.trivia ? self.trivia.truncate(1) : 0) +
        (self.clues ? -CLUE_DEDUCTION : 0)
    end
end
