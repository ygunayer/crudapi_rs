## Description
Just a fun project where I try implement a very simple CRUD API in Rust.

I'm not using Rust in professional capacity at the moment, so please don't expect this repo to be a perfect representation of the current conventions.

## Fundamentals
The project is structured with a somewhat traditional mindset: requests are accepted and validated at `endpoints`, which then delegate them to `services` where all the business logic resides, and database interactions are handled at `repositories` using `entities`.

To reduce the potential boilerplate code from repositories I've also added a derive package which can be used to derive commonly used DB operations (e.g. `select_row`, `select_one`, `find_by_id`, `exists`) for a given entity class (e.g. `AccountEntity`), similar to [JPA repositories from Spring Data](https://spring.io/guides/gs/accessing-data-jpa/).

As for dependencies, below is a list of the ones that stand out. For a full list please see [Cargo.toml](./Cargo.toml)

- actix-web
- tokio-postgres
- tokio-pg-mapper
- deadpool
- deadpool-postgres

## Running
The project includes a Docker compose stack to run infrastructural components. Make sure that this stack is up and running before launching the project.
```bash
$ docker-compose up -d
```

Once the Docker compose stack is up, you can either run individual projects from your favorite IDE, or from the command line using sbt commands.

If you'd like to see what's stored in the database you can connect to it at `postgres://localhost:5432/postgres` with the username `postgres` and password `postgres`.

## API Requests
At the moment this repo is pretty much just a husk, and therefore no endpoints are fully operational.

That said, you can access the registration endpoint by sending the following request:

```
POST http://localhost:9000/api/v1/auth/register
```

Request Body:
```json
{
  "firstName": "Foo",
  "lastName": "Bar",
  "organizationName": "Foo Bar Inc.",
  "gsmNumber": "905551111111",
  "email": "foo@bar.com",
  "password": "123",
  "passwordRepeat": "123"
}
```

Sadly you won't receieve a response... at least for now :)

## License
MIT
