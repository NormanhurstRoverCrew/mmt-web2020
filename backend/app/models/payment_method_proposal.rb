class PaymentMethodProposal < ApplicationRecord
    belongs_to :booking, required: false

    def price
        case self.method
		when "eft"
			30.00
		else
			40.00
		end
    end
end
