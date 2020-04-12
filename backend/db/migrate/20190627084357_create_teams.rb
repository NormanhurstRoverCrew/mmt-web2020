class CreateTeams < ActiveRecord::Migration[5.2]
  def change
    create_table :teams do |t|
      t.timestamps
      t.string        :uid
      t.string        :name
      t.string        :registration
    end
  end
end
