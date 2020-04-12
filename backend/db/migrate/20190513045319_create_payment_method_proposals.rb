class CreatePaymentMethodProposals < ActiveRecord::Migration[5.2]
  def change
    create_table :payment_method_proposals do |t|
      t.timestamps
      t.belongs_to :booking, index: true
      t.string      :method
    end
  end
end
