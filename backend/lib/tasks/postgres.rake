namespace :postgres do
    desc "Drop current database"
    task :drop do
        sh %{ echo db:5432:#{ENV['DB_MASTER']}:#{ENV['DB_MASTER']}:#{ENV['DB_MASTER_PASSWORD']} > ~/.pgpass }
        sh %{ chmod 600 ~/.pgpass }

        databases = `psql -U #{ENV['DB_MASTER']} -h db -c \\\\l`
        if databases.include? ENV['DB_USER'] then
            sh %{ psql -U postgres -h db -c "DROP DATABASE #{ENV['DB_USER']};" }
        end
    end

    namespace :setup do
        desc "set up roles"
        task :roles do 
            sh %{ echo db:5432:#{ENV['DB_MASTER']}:#{ENV['DB_MASTER']}:#{ENV['DB_MASTER_PASSWORD']} > ~/.pgpass }
            sh %{ chmod 600 ~/.pgpass }

            users = `psql -U #{ENV['DB_MASTER']} -h db -c \\\\du` #double escape back slash
            if not users.include? ENV['DB_USER'] then
                puts "CREATING USER"
                sh %{ createuser -U postgres -h db -d -e #{ENV['DB_USER']} }
            end

            sh %{ psql -U postgres -h db -c "alter user #{ENV['DB_USER']} with password '#{ENV['DB_PASSWORD']}';" }

            databases = `psql -U #{ENV['DB_MASTER']} -h db -c \\\\l`
            if not databases.include? ENV['DB_USER'] then
                puts "CREATING DB"
                sh %{ createdb -h db -U postgres -O #{ENV['DB_USER']} #{ENV['DB_USER']} }
            end

            sh %{ rm -f ~/.pgpass }
        end
    end

    desc "Create and init DB"
    task :redo => ["postgres:drop", "postgres:setup:roles", "db:migrate"] do 
        puts "Re-created postgres env"
    end
end
