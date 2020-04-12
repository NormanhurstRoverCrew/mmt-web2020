Rails.application.routes.draw do
  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
  root 'api/hello#index'

  scope :api do
    # Put all the resources here. eg.

    # Tickets
    resources :tickets, only: [:create, :index, :show, :delete, :update] do
      post 'verify' => 'tickets#verify', as: :verify
    end

    # Bookings
    resources :bookings, only: [:create, :update, :index, :show] do
      resources :tickets, only: [:create]
      resources :payments, only: [:create]
    end

    # Payment
    resources :payments, only: [:index]
  end
end
