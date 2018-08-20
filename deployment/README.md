# docker

This module contains info related to deployment.

## Files

- `docker`: docker files that define how to build the production binaries.
    - `docker-compose.yaml`: architecture to deploy in develop.
    - `docker-compose.prod.yaml`: architecture to deploy in production.
    - `.env`: configuration to deploy in local the necessary services for development.
      > This file is not persisted in git, you have to create it.

      The default content is:
      ```dotenv
      ARANGODB_DATABASE_PATH=../.local/database/data
      ARANGODB_DATABASE_APPS_PATH=../.local/database/apps
      ARANGODB_ROOT_PASSWORD=gmi
      ```
- `nginx`: the configuration for the production nginx.
- `config.toml`: configuration of the application.
  > The file persisted in git is for development, change it to deploy for production.
