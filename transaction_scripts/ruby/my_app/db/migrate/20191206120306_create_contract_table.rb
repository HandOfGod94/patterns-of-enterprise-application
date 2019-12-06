class CreateContractTable < ActiveRecord::Migration[5.0]
  class << self
    def up
      create_table :contracts do |t|
        t.integer :revenue
        t.date :date_signed
        t.references :products, foreign_key: true, on_delete: :cascade
      end
    end
    
    def down
      drop_Table :contracts
    end
  end
end
