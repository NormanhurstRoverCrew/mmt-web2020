Rails.application.routes.draw do
  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
  root "api/hello#index"

  namespace :pdf do
    get '/invitation/:uid' => 'invitation#index'
    get '/team_tag' => 'team_tag#index'
  end

  namespace :api do
    # Put all the resources here. eg.


    # Tickets
    resources :tickets, only: [:create, :index, :show, :destroy, :update] do
      post "verify" => "tickets#verify", as: :verify
      post "email/:id" => "tickets#sendEmail"
      get "paid" => "payments#paid"
    end

    # Bookings
    resources :bookings, only: [:create, :update, :index, :show, :destroy] do
      resources :tickets, only: [:create, :index]
      resources :payments, only: [:create]
      post "verify" => "bookings#verify", as: :verify
    end

    # Payment
    resources :payments, only: [:index]

    resources :teams, only: [:index, :show, :create, :update] do
      resources :point_logs, only: [:index, :create]
      resources :tickets, only: [:index]
      patch "ticket" => "teams#add_ticket"
    end

    resources :point_logs, only: [:index]

    namespace :stripe do
      post "checkout/:uid" => "checkout#create"
      resources :events, only: [:index, :show]
      resources :customers, only: [:index, :show]
      resources :payment_intents, only: [:index, :show]
      resources :charges, only: [:index]
      resources :tickets, only: [:show]
      resources :bookings, only: [] do
        resources :charges, only: [:index]
      end
    end
  end
end
