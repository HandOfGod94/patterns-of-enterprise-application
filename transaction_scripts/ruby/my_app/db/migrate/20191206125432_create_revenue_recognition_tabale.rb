class CreateRevenueRecognitionTabale < ActiveRecord::Migration[5.0]
  class << self
    def up
      create_table :revenue_recognitions, primary_key: [:contracts_id, :amount] do |t|
        t.references :contracts, foreign_key: true, on_delete: :cascade
        t.integer :amount
        t.date :recognized_on
      end
    end
    
    def down
      drop_table :revenue_recognitions
    end
  end
end
