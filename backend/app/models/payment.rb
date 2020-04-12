class Payment < ApplicationRecord
	belongs_to :booking
	
	def method_string
		case self.method
		when "eft"
			"Electronic Funds Transfer(Bank Transfer)"
		when "otd"
			"Cash on the day"
		else
			self.method
		end
	end
end
