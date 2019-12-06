class CreateProductTable < ActiveRecord::Migration[5.0]
  class << self
    def up
      create_table :products do |t|
        t.string :name, null: false
        t.string :type, null: false
      end
    end

    def down
      drop_table :products
    end
  end
end
