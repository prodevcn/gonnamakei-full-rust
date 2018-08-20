# gonnamakeit.app

This repo contains all the code related to the website and it is divided into:

## Backend

The backend is developed following a microservices architecture in Rust. The database
is [ArangoDB](https://www.arangodb.com/).

- [`commons/`](commons/README.md): contains all the code that is (or will be) shared between across all microservices.
- `libs/`:
    - `db-model-macro/`: library that simplifies the generation of DB models in ArangoDB.
    - `enum-derive/`: library that helps to work with enums in Rust.
- [`api-server/`](api-server/README.md): offers the API to the world.

## Frontend

- `clients/`:
    - [`web-client/`](clients/web-client/README.md): the web client that is a plain landing. In Angular + TypeScript.
    - [`web-client-old/`](clients/web-client-old/README.md): the web client that consumes the API. Old versions not
      following the design. In Vue + TypeScript.

> Currently, both clients are the same BUT the old one must be migrated to the new one applying the design.

## Auxiliary

- [`deployment/`](deployment/README.md): contains all data necessary to deploy the front/back.
- [`scripts/`](scripts/README.md): auxiliary program to automate actions, for example, clean the database.

## How to locally deploy the server

To deploy the server you need to:

1. Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
   and [solana](https://docs.solana.com/cli/install-solana-cli-tools).
2. Go to `deployment/docker/` folder and run `docker-compose up -d` to run the database docker.
    1. To initiate it and create the collections, go to `scripts/` and run `cargo run -- reset_db`.
   > WARN: this command will remove the entire DB and set it up again.
3. Go to another terminal tab and run `solana-test-validator`

> Before next step if you are a linux user, ensure you have the necessary libs executing the command `apt-get install pkg-config libudev-dev -y`.

4. Then go to `api-server/` folder and
   run `export RUST_LOG=info;CONFIG_PATH=<your-path-to-the-repo>/deployment/config.toml cargo run`.

> If an error happens saying something about _phantom not existing run:
> [Linux] `sed -i -E "s/_phantom/phantom/g" ~/.cargo/registry/src/github.com-1ecc6299db9ec823/arangors-0.5.0/src/aql.rs`
> [Mac] `sed -i -E "s/_phantom/phantom/g" ~/.cargo/registry/src/github.com-1ecc6299db9ec823/arangors-0.5.0/src/aql.rs`
> And then try again step 4.

Now the server should be working properly. After this moment, you just have to execute steps 2. and 4. to run it again.

### Shut down

1. Stop any active `cargo run` commands.
2. Go to `deployment/docker/` folder and run `docker-compose down` to close the database docker.
    1. To remove the entire DB data, remove the `deployment/docker/.local` folder.

## How to locally deploy the server

> Ensure `API_SERVER_URL` is set to `http://localhost:3003` in `clients/src/environments/environment.ts`.

Go to `clients/web-client` folder and run `npm install` then `npm run start`