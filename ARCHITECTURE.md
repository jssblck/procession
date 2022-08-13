Describes the architecture of the `procession` server.

# High Level Architecture

- A `procession` server manages jobs using a backing [`redis`](https://redis.io/) database.
- One or more `client` instances push jobs to the server to be processed.
- One or more `client` instances consume jobs from the server.

# Future

As the server gets implemented, this'll be fleshed out more.
At a high level, here are the architectural goals:

- Server
  - Support connecting to a pool of replicated redis databases.
  - Support running multiple `procession` servers against the same pool.
- Clients
  - Any client can be a job producer or consumer.
