require "test_helper"

class TicketTest < ActiveSupport::TestCase
  booking = Booking.create
  test "new ticket has unique ID" do
    ticket = booking.tickets.create
    assert_not_nil ticket.uid
  end
end
