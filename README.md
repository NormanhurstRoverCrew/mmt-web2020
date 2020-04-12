# mmt-web

## Environment Variables
- MMT19_ENV: [production / development]

### DB
- MMT19_DB_POSTGRES_PASSWORD - password of the 'postgres'(root) user of the pg db
- MMT19_DB_PATH
- MMT19_DB_USER - username of the production pg user
- MMT19_DB_PASSWORD - password of the production pg user
- MMT19_DB_DEV_USER - username of the development pg user
- MMT19_DB_DEV_PASSWORD - password of the development pg user

## Docker
### Networks
- mmt-dmz: all backend services should only be part of the dmz(db, root api, ...)
- proxy: all front end applications should be part of this so NGINX can proxy to them. NGINX should not be able to access db and such

When first running 'docker-compose up' an error like:
`ERROR: Network mmt-dmz declared as external, but could not be found. Please create the network manually using 'docker network create mmt-dmz' and try again.`

run:

`docker network create mmt-dmz;
docker network create proxy`
