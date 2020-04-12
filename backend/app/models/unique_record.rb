class UniqueRecord < ApplicationRecord
  self.abstract_class = true

  require "ulid"

  validates :uid, presence: true, on: :update

  before_create :gen_uid

  def gen_uid
    self.uid = ULID.generate
  end
end
