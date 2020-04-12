class QrTeamJob < ApplicationJob
  queue_as :qr

    def perform(team)
      qrcode = RQRCode::QRCode.new("MMT19:TEAM:#{team.uid}")
      png = qrcode.as_png(
        resize_gte_to: false,
        resize_exactly_to: false,
        fill: 'white',
        color: '#000000',
        size: 500,
        border_modules: 0,
        module_px_size: 0
      )
      path = Rails.root.join("tmp", "qr")
      file = path.join("team_#{team.uid}.png")
		  `mkdir -p #{path}`
      File.open(file, 'wb') do |pngfile|
        pngfile << png
      end
		  puts "[QR] Generated Team #{file}.png"
    end
end
