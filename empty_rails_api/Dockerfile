FROM ruby:2.5.1

RUN apt-get update && apt-get install -qq -y build-essential nodejs libpq-dev postgresql --fix-missing --no-install-recommends

WORKDIR /usr/src/app

COPY Gemfile Gemfile
RUN bundle install

ADD . .

CMD rails s
