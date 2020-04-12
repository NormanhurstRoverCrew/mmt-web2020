require "test_helper"
require "ulid"

class Api::TicketsControllerTest < ActionDispatch::IntegrationTest
  setup do
    @booking = bookings(:one)
    @ticket = tickets(:one)
  end

  test "should get index" do
    get api_tickets_url, as: :json
    assert_response :success
  end

  test "should create api_ticket" do
    assert_difference("Ticket.count") do
      post api_tickets_url, params: {booking_id: @booking.uid, user: {name: "Grant Perry", email: "#{ULID.generate}@gmail.com"}}, as: :json
    end
    assert_response 201
  end

  test "should show api_ticket" do
    get api_ticket_url(@ticket, id: @ticket.uid), as: :json
    assert_response :success
  end

  # test "should update api_ticket" do
  #   patch api_ticket_url(@api_ticket), params: {api_ticket: {}}, as: :json
  #   assert_response 200
  # end

  # test "should destroy api_ticket" do
  #   assert_difference("Ticket.count", -1) do
  #     delete api_ticket_url(@api_ticket), as: :json
  #   end

  #   assert_response 204
  # end
end
