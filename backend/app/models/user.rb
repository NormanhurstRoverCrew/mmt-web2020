class User < UniqueRecord
  belongs_to :ticket, required: false

  validates :name, format: {with: /([\w\-\']{2,})(([\s]+)([\w\-\']{2,})){1,}/}, required: false
  #validates :email, format: {with: /\A[^@\s]+@[^\@\s\-][^@\s]+[\s]*/, message: "is not Valid"}, uniqueness: true

  def fname
    return self.name.split(" ")[0]
  end

  # a Unique computational code
  def code
    md5 = Digest::MD5.new
    md5 << self.email_verified.to_s
    md5 << self.name
    md5 << self.email_verified.to_s
    md5 << (self.email || "")
    md5 << self.email_verified.to_s
    md5 << (self.mobile || "")
    md5 << self.email_verified.to_s
    md5 << self.created_at.to_i.to_s
    md5 << self.email_verified.to_s
    md5 << (self.crew || "")
    md5 << self.email_verified.to_s
    return md5.hexdigest
  end
end
