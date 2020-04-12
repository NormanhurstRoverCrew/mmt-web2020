class CreateUsers < ActiveRecord::Migration[5.2]
  def change
    create_table :users do |t|
      t.timestamps
      t.belongs_to :ticket
      t.belongs_to :booking # => has one
      t.string :uid
      t.string :name
      t.string :crew
      t.string :email
      t.boolean :email_verified, default: 0
      t.string :mobile
      t.text :diet
    end
  end
end
