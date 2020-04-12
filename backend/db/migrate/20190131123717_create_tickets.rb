class CreateTickets < ActiveRecord::Migration[5.2]
  def change
    create_table :tickets do |t|
      t.timestamps
      t.string :uid
      t.belongs_to :booking, index: true
    end
  end
end
