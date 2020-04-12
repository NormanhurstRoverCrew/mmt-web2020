class CreateStripePaymentIntents < ActiveRecord::Migration[5.2]
  def change
    create_table :stripe_payment_intents do |t|
      t.timestamps
      t.string :pi_id  #payment intent ID
      t.string :customer_id  #customer ID
      t.belongs_to :booking, index: true
    end
  end
end
