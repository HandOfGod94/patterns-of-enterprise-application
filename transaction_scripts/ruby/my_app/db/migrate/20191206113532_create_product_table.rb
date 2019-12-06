class CreateProductTable < ActiveRecord::Migration[4.2]
  class << self
    def up
      create_table :product do |t|
        t.string :name, null: false
        t.string :type, null: false
      end
    end

    def down
      drop_table :product
    end
  end
end
