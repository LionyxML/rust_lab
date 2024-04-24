# Axum Postgres Crud App


## Pre-install
Use docker or podman to serve the postgre db.

``` shell
podman-compose up
```

You should have adminer on localhost:8080 and the posgres on localhost:5432.

username: postgres
password: example
database: postgres


List you containers with:

``` shell
podman container list
```

Get the ID of the postgres container, log into it with:

``` shell
podman container exec -it ID /bin/sh
# inside the container
psql -U postgres
```

Now create the user and database:

``` sql
CREATE ROLE axum_postgres WITH LOGIN PASSWORD 'axum_postgres';
CREATE DATABASE axum_postgres WITH OWNER = 'axum_postgres';
\q
```

Now test the user login with:

``` shell
psql -U axum_postgres
```

Create the table:

``` sql
CREATE TABLE tasks (
  task_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  priority INT
  );
  \q
```

Check the table:

``` sql
SELECT * FROM tasks;
```

