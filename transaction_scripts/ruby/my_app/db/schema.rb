# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# Note that this schema.rb definition is the authoritative source for your
# database schema. If you need to create the application database on another
# system, you should be using db:schema:load, not running all the migrations
# from scratch. The latter is a flawed and unsustainable approach (the more migrations
# you'll amass, the slower it'll run and the greater likelihood for issues).
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema.define(version: 2019_12_06_125432) do

  create_table "contracts", force: :cascade do |t|
    t.integer "revenue"
    t.date "date_signed"
    t.integer "products_id"
    t.index ["products_id"], name: "index_contracts_on_products_id"
  end

  create_table "products", force: :cascade do |t|
    t.string "name", null: false
    t.string "type", null: false
  end

  create_table "revenue_recognitions", primary_key: ["contracts_id", "amount"], force: :cascade do |t|
    t.integer "contracts_id"
    t.integer "amount"
    t.date "recognized_on"
    t.index ["contracts_id"], name: "index_revenue_recognitions_on_contracts_id"
  end

end
