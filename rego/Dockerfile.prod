FROM node:8-alpine

WORKDIR /usr/src
USER root

#https://github.com/npm/npm/issues/20861
RUN npm config set unsafe-perm true

COPY package.json package.json
COPY package-lock.json package-lock.json
RUN npm install
COPY . .
RUN npm run build && rm -rf client && rm -rf server && rm -rf coverage && rm -rf tests

CMD ["npm", "run", "prod"]
