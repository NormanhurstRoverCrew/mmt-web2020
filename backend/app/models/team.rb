class Team < UniqueRecord
    has_many    :tickets
    has_many    :point_logs

    def add_log log
        log.team = self
        log.save
        log
    end

    private def calculate_points logs
        logs.collect {|x| x.sum_points }.sum.truncate(1)
    end

    def points
        calculate_points(self.point_logs)
    end

    def points_at_base base
        calculate_points(self.point_logs.where(base: base))
    end

    def arrived_at_base? base
        self.point_logs.where(base: base, arrived: true).any?
    end

    def departed_base? base
        self.point_logs.where(base: base, departed: true).any?
    end

    def arrived_at_base_at base
        log = self.point_logs.where(base: base, arrived: true).order(:logged_at).first
        log ? log.logged_at : nil
    end

    def at_base base
        log_arrived = self.point_logs.where(base: base).where.not(arrived: [nil, ""]).order(:logged_at).last
        log_departed = self.point_logs.where(base: base).where.not(departed: [nil, ""]).order(:logged_at).last

        if not log_arrived
            false            
        elsif not log_departed
            true
        else
            log_arrived.logged_at > log_departed.logged_at
        end
    end

    def departed_base_at base
        log = self.point_logs.where(base: base, departed: true).order(:logged_at).last
        log ? log.logged_at : nil
    end

    def comments_at base
        self.point_logs.where(base: base).where.not(comment: [nil, ""]).collect {|x| {
                comment: x.comment,
                logged_at: x.logged_at,
                admin: x.admin,
            }
        }
    end

    def comments
        self.point_logs.where.not(comment: [nil, ""]).collect {|x| {
                base: x.base,
                comment: x.comment,
                logged_at: x.logged_at,
                admin: x.admin,
            }
        }
    end

    def createQR
        qrcode = RQRCode::QRCode.new("MMT19:TEAM:#{self.uid}")
            png = qrcode.as_png(resize_gte_to: false,	resize_exactly_to: false,	fill: 'white', color: '#000000', size: 500, border_modules: 0, module_px_size: 0)
        dir = Rails.root.join( "app", "assets", "images", "qr", "team")
        `mkdir -p #{dir}`
        fileName = "#{self.uid}.png"
            filePath = dir.join(fileName)
            File.open(filePath, 'wb') do |pngfile|
                pngfile << png
            end
            puts "[QR] Generated Team ID#{fileName}"
            filePath
      end
end
