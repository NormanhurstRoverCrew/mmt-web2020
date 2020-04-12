require "test_helper"

class BookingTest < ActiveSupport::TestCase
  test "new booking has unique ID" do
    booking = Booking.create
    assert_not_nil booking.uid
  end
end
