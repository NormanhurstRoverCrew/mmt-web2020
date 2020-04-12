FROM ruby:2.6.0

RUN apt-get update && apt-get install -qq -y build-essential nodejs libpq-dev postgresql --fix-missing --no-install-recommends

WORKDIR /usr/src/app

COPY Gemfile Gemfile
COPY Gemfile.lock Gemfile.lock
RUN bundle install
COPY Rakefile Rakefile
COPY config config
COPY lib/tasks lib/tasks
RUN rake app:update

ADD . .

CMD ["bash","START.sh"]
