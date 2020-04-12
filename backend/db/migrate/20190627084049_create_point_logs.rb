class CreatePointLogs < ActiveRecord::Migration[5.2]
  def change
    create_table :point_logs do |t|
      t.timestamps
      t.belongs_to  :team
      
      t.datetime    :logged_at
      
      t.integer     :base   #the index of the base that the log is for
      t.string      :admin  #the auth0 id that made the log
      
      t.boolean     :arrived
      t.boolean     :departed

      t.float       :points
      t.float       :trivia
      t.boolean     :clues
      
      t.string      :comment

      t.boolean     :use  #used to enable/disable counting of this record when computing results
    end
  end
  def self.down
    drop_table  :point_logs
  end
end
