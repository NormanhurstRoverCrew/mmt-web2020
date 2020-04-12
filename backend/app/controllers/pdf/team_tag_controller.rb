class Pdf::TeamTagController < ApplicationController
    include ActionController::MimeResponds

    def index
		respond_to do |format|
			format.html
			format.pdf do
				puts "[PDF]"
				#dir = Rails.root.join('tmp', "invitations")
				dir = "/PDFCache"
				#file = dir.join("invite.pdf")
				file = "#{dir}/MMT2019_ALL_TEAMS_ #{Time.now.to_i}.pdf"
				`mkdir -p #{dir}`

				#intergrate the QR code in here. use the uid as the string
				#File.
				@qr = []
				Team.all.each do |team|
					@qr << {image: team.createQR, id: team.id}
				end

				#render template: nil, pdf: "invitation", dpi: '380', page_size: 'A4', save_to_file: file   # Excluding ".pdf" extension.
				#send_file template: nil, pdf: "invitation", dpi: '380', page_size: 'A4', save_to_file: file   # Excluding ".pdf" extension.
				#pdf = render_to_string pdf: "some_file_name", template: 'pdf_renderer/invitation', encoding: "UTF-8", dpi: '380', page_size: 'A4'
				pdf = render_to_string template: 'pdf/team_tag/index', encoding: "UTF-8", dpi: '380', page_size: 'A4'
				pdf = WickedPdf.new.pdf_from_string(pdf)
				# puts pdf

				File.open(file, 'wb') do |file|
					file << pdf
				end
				send_file file
				return
			end
		end
	end
end
