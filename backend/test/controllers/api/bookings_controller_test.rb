require "test_helper"
require "ulid"

class Api::BookingsControllerTest < ActionDispatch::IntegrationTest
  setup do
    @booking = bookings(:one)
  end

  test "should get index" do
    get api_bookings_url, as: :json
    assert_response :success
  end

  test "should create booking" do
    assert_difference("Booking.count") do
      post api_bookings_url, params: {tickets: 1, user: {
                               name: "Grant Perry",
                               email: "#{ULID.generate}@gmail.com",
                               mobile: "04312837618",
                               crew: "Normanhurst Rover Crew",
                             }}, as: :json
    end

    assert_response 201
  end

  test "should show api_booking" do
    get api_booking_url(@api_booking, id: "@booking.uid"), as: :json
    assert_response :success
  end

  # test "should update api_booking" do
  #   patch api_booking_url(@api_booking), params: {api_booking: {}}, as: :json
  #   assert_response 200
  # end

  # test "should destroy api_booking" do
  #   assert_difference("Api::Booking.count", -1) do
  #     delete api_booking_url(@api_booking), as: :json
  #   end

  #   assert_response 204
  # end
end
