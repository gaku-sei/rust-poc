#!/usr/bin/env sh

for f in $(ls ./sql/*.sql)
do
  sqlite3 db.sqlite < $f
done
