class CreatePayments < ActiveRecord::Migration[5.2]
  def change
    create_table :payments do |t|
      t.timestamps
      t.belongs_to :booking
      t.string :method
      t.float :amount
      t.boolean :verified
    end
  end
end
