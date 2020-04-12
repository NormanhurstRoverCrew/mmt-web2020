class QrTicketJob < ApplicationJob
  queue_as :qr

    def perform(ticket)
      qrcode = RQRCode::QRCode.new("MMT19:TICKET:#{ticket.uid}")
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
      file = path.join("ticket_#{ticket.uid}.png")
		  `mkdir -p #{path}`
      File.open(file, 'wb') do |pngfile|
        pngfile << png
      end
		  puts "[QR] Generated Ticket#{file}.png"
    end
end
