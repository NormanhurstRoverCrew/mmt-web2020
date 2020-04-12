# Backups
## Making

dc exec db pg_dumpall -U postgres > /tmp/mmt19db_`date +%Y%m%d`_`date +%k%M`.bak

## Restoring

Start up a fresh postgres db. no config no nothing.

cat /tmp/mmt19db_20190618_1607.bak | d exec --interactive mmt_db_1 psql -U postgres -f - postgres

this pipes the backup file to psql, which this runs the full setup process of the entire db.
