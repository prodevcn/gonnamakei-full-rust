# commons

This module contains all the code that is (or will be) shared between across all microservices.

## Modules

- `api_clients`: contains the clients ready to work with their respective APIs, for example, Clash Royale API.
- `config`: the objects that map the configuration set in `config.toml`.
- `data`: contains shared general information.
- `database`: contains all code related to the database models. For each of them there are a Document and Collection.
- `errors`: a generic structure for the errors in the project.
- `server`: contains generic helpers to work with servers.
- `test`: utilities for the tests in this and other modules.
- `tests`: the tests of this module.
- `utils`: general utilities for the project.
- `wallet_clients`: clients to interact with the different blockchains.