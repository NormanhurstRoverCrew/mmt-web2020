class CreateTests < ActiveRecord::Migration[5.2]
  def change
    create_table :tests do |t|
      t.timestamps
      t.string    :uid
      t.integer   :number
    end
  end
end
