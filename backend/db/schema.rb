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

ActiveRecord::Schema.define(version: 2019_08_27_125844) do

  # These are extensions that must be enabled in order to support this database
  enable_extension "plpgsql"

  create_table "bookings", force: :cascade do |t|
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "uid"
  end

  create_table "email_logs", force: :cascade do |t|
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.bigint "ticket_id"
    t.string "email_id"
    t.index ["ticket_id"], name: "index_email_logs_on_ticket_id"
  end

  create_table "payment_method_proposals", force: :cascade do |t|
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.bigint "booking_id"
    t.string "method"
    t.index ["booking_id"], name: "index_payment_method_proposals_on_booking_id"
  end

  create_table "payments", force: :cascade do |t|
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.bigint "booking_id"
    t.string "method"
    t.float "amount"
    t.boolean "verified"
    t.index ["booking_id"], name: "index_payments_on_booking_id"
  end

  create_table "point_logs", force: :cascade do |t|
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.bigint "team_id"
    t.datetime "logged_at"
    t.integer "base"
    t.string "admin"
    t.boolean "arrived"
    t.boolean "departed"
    t.float "points"
    t.float "trivia"
    t.boolean "clues"
    t.string "comment"
    t.boolean "use"
    t.index ["team_id"], name: "index_point_logs_on_team_id"
  end

  create_table "stripe_payment_intents", force: :cascade do |t|
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "pi_id"
    t.string "customer_id"
    t.bigint "booking_id"
    t.index ["booking_id"], name: "index_stripe_payment_intents_on_booking_id"
  end

  create_table "teams", force: :cascade do |t|
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "uid"
    t.string "name"
    t.string "registration"
  end

  create_table "tickets", force: :cascade do |t|
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "uid"
    t.bigint "booking_id"
    t.bigint "team_id"
    t.index ["booking_id"], name: "index_tickets_on_booking_id"
    t.index ["team_id"], name: "index_tickets_on_team_id"
  end

  create_table "users", force: :cascade do |t|
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.bigint "ticket_id"
    t.bigint "booking_id"
    t.string "uid"
    t.string "name"
    t.string "crew"
    t.string "email"
    t.boolean "email_verified", default: false
    t.string "mobile"
    t.text "diet"
    t.index ["booking_id"], name: "index_users_on_booking_id"
    t.index ["ticket_id"], name: "index_users_on_ticket_id"
  end

  add_foreign_key "tickets", "teams"
end
