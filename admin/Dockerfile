FROM node:10-alpine

WORKDIR /usr/src
USER root

RUN apk add git

#https://github.com/npm/npm/issues/20861
RUN npm config set unsafe-perm true

# mocha is for running node tests
RUN npm install -g mocha

COPY package.json .
RUN npm install

COPY . .

CMD ["npm", "run", "dev"]
