class CreateEmailLogs < ActiveRecord::Migration[5.2]
  def change
    create_table :email_logs do |t|
      t.timestamps
      t.belongs_to  :ticket
      t.string      :email_id
    end
  end
end
