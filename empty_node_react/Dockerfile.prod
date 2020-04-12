FROM node:8-alpine

WORKDIR /usr/src
USER root

#https://github.com/npm/npm/issues/20861
RUN npm config set unsafe-perm true

COPY package.json .
RUN npm install

COPY . .

RUN npm run build

CMD ["npm", "run", "prod"]
