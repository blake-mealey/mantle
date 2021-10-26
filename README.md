# Rocat 🚀

Early development of a new tool for deploying projects to Roblox using the new [Open Cloud
APIs](https://devforum.roblox.com/t/open-cloud-publishing-your-places-with-api-keys-is-now-live/1485135).

⚠ Please note that this is an early release and the API is unstable ⚠

## Installation

### Manual downloads (simplest)

You can download prebuilt binaries from the [GitHub
Releases](https://github.com/blake-mealey/rocat/releases) page.

### With Foreman (recommended)

It is recommended to install with Foreman, with the following config:

```toml
# foreman.toml

[tools]
rocat = { source = "blake-mealey/rocat", version = "0.1.5" }
```

You can learn more about Foreman including how to install it from [its
documentation](https://github.com/Roblox/foreman#readme).

## Usage

### Authentication

In order to use any of the commands to save/publish places to Roblox, you must provide valid
authentication. This is provided via the `ROBLOX_API_KEY` environment variable. You can create an
API key in the Roblox [Creator portal](https://create.roblox.com/credentials).

You also must ensure your API key has the required permissions. It must have the Place Management
API System with the desired places added to it.

### Manually save/publish a place to Roblox

You can use the `save` and `publish` commands to manually save or publish a Roblox place file
(`rbxl` or `rbxlx`) to a pre-existing place.

```sh
# Save
rocat save place.rbxl <experience_id> <place_id>

# Publish
rocat publish place.rbxl <experience_id> <place_id>
```

### Configure deployments

You can configure reusable Roblox deployments by creating a TOML config file and using the `deploy`
command.

By default, Rocat will look for a `rocat.toml` file but you may specifiy an alternate file with the
`--config` argument.

```toml
# rocat.toml
place_file = "place.rbxl"

[branches.dev]
environment = "staging"
deploy_mode = "Save" # optional; defaults to "Publish"

[branches.main]
environment = "production"
tag_commit = true # optional; defaults to false

[environments.staging]
experience_id = 3028808377
place_id = 7818935418

[environments.production]
experience_id = 6428418832
place_id = 4927604916
```

With the above configuration, we are telling Rocat that when the `deploy` command is run on the
`dev` branch, it should save the `place.rbxl` file to experience/place specified in the `staging`
environment, and when it is run on the `main` branch, it should publish the `place.rbxl` file to the
experience/place specified in the `production` environment.

You can perform the deployment by running the `deploy` command:

```sh
rocat deploy
```

If the current git branch does not match any of the provided configurations, the tool will return a
success exit code but will not do anything.

### Multi-file projects

Rocat also supports configurations for multi-file projects which uses a slightly altered
configuration format:

```toml
# rocat.toml
[place_files]
main = "place.rbxl"
lobby = "lobby.rbxl"

[branches.dev]
environment = "staging"
deploy_mode = "Save" # optional; defaults to "Publish"

[branches.main]
environment = "production"
tag_commit = true # optional; defaults to false

[environments.staging]
experience_id = 3028808377
place_ids = { main = 7818935418, lobby = 6179245670 }

[environments.production]
experience_id = 6428418832
place_ids = { main = 4927604916, lobby = 7618543001 }
```

When the `deploy` command is run with this configuration, the same deployments will be made as with
the above single-file configuration, except that both the `place.rbxl` and `lobby.rbxl` files will
be uploaded to their respective places.

### GitHub Actions

Combined with the [Roblox/setup-forman](https://github.com/Roblox/setup-foreman) Action, it is easy
to create a workflow to deploy your places using Rocat.

Here is an example for a fully-managed Rojo project:

```yml
# .github/workflows/deploy.yml

name: Deploy

on:
  push:
    branches:
      - dev
      - main

jobs:
  build-and-deploy:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v2
      - uses: Roblox/setup-foreman@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
      - name: Build project
        run: rojo build --output prod.rbxlx
      - name: Deploy project
        run: rocat deploy prod.rbxlx
        env:
          ROBLOX_API_KEY: ${{ secrets.ROBLOX_API_KEY }}
```

Note that you will need to add your Roblox API key as a secret to your GitHub repository. You can
learn how to do this using [their
docs](https://docs.github.com/en/actions/security-guides/encrypted-secrets#creating-encrypted-secrets-for-a-repository).
