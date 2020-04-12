require "test_helper"
require "ulid"

class UserTest < ActiveSupport::TestCase
  setup do
    @ticket = Booking.create.tickets.create
  end

  test "new user has unique ID" do
    @ticket.user = User.create name: "Grant Perry", email: "#{ULID.generate}@gmail.com"
    user = @ticket.user
    assert_not_nil user.uid
  end

  test "new user requires valid email" do
    assert_raises ActiveRecord::RecordNotSaved do
      @ticket.user = User.create name: "Grant Perry", email: "gmail.com"
    end
  end

  test "new user: valid email causes no errors" do
    @ticket.user = User.create name: "Grant Perry", email: "grant@gmail.com"
    assert @ticket.user.valid?
  end

  test "new user: invalid name causes errors" do
    assert_raises ActiveRecord::RecordNotSaved do
      @ticket.user = User.create name: "", email: "#{ULID.generate}@gmail.com"
    end
  end

  test "new user: nil name causes errors" do
    assert_raises ActiveRecord::RecordNotSaved do
      @ticket.user = User.create name: nil, email: "#{ULID.generate}@gmail.com"
    end
  end

  test "new user: name must be two words or more" do
    assert_raises ActiveRecord::RecordNotSaved do
      @ticket.user = User.new name: "Grant", email: "#{ULID.generate}@gmail.com"
    end

    assert_raises ActiveRecord::RecordNotSaved do
      @ticket.user = User.new name: "Grant-jack", email: "#{ULID.generate}@gmail.com"
    end

    @ticket.user = User.new name: "Grant Perry", email: "#{ULID.generate}@gmail.com"
    assert @ticket.user.valid?

    @ticket.user = User.new name: "Grant Alfred Perry", email: "#{ULID.generate}@gmail.com"
    assert @ticket.user.valid?
  end

  test "new user: name can contain hyphens(-)" do
    @ticket.user = User.new name: "Grant Alfred-Perry", email: "#{ULID.generate}@gmail.com"
    assert @ticket.user.valid?
  end
end
