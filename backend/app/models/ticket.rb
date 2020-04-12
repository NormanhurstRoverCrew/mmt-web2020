class Ticket < UniqueRecord
  belongs_to :booking
  belongs_to :team, optional: true
  has_one :user
  has_many :email_log

  def obj
    user = self.user
    return {
            id: self.id,
            team_id: self.team ? self.team.id : nil,
             uid: self.uid,
             created_at: self.created_at,
             updated_at: self.updated_at,
             user: user,
           }
  end

  def price
    date = self.booking.created_at.to_i

    case date
    when 0..Time.zone.parse("2019-09-7").to_i
      30.00
    else
      40.00
    end
  end

  def createQR
    qrcode = RQRCode::QRCode.new("MMT19:TICKET:#{self.uid}")
		png = qrcode.as_png(resize_gte_to: false,	resize_exactly_to: false,	fill: 'white', color: '#000000', size: 500, border_modules: 0, module_px_size: 0)
    dir = Rails.root.join( "app", "assets", "images", "qr", "ticket")
    `mkdir -p #{dir}`
    fileName = "#{self.uid}.png"
		filePath = dir.join(fileName)
		File.open(filePath, 'wb') do |pngfile|
			pngfile << png
		end
		puts "[QR] Generated Rover ID#{fileName}"
		filePath
  end
end
