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
rocat = { source = "blake-mealey/rocat", version = "0.3.0" }
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

By default, Rocat will look for a `rocat.yml` file but you may specifiy an alternate file with the
`--config` argument.

```yml
# rocat.yml
placeFiles:
  start: start-place.rbxlx

deployments:
  - name: staging
    branches: [dev, experiments/*]
    deployMode: Save # optional; defaults to Publish
    experienceId: 7067418676
    placeIds:
      start: 8468630367
  - name: production
    branches: [main]
    tagCommit: true # optional; defaults to false
    experienceId: 6428418832
    placeIds:
      start: 4927604916
```

With the above configuration, we are telling Rocat that when the `deploy` command is run on the
`dev` branch, it should save the `start-place.rbxl` file to the experience/place specified in the
`staging` environment, and when it is run on the `main` branch, it should publish the
`start-place.rbxl` file to the experience/place specified in the `production` environment.

You can perform the deployment by running the `deploy` command:

```sh
rocat deploy
```

If the current git branch does not match any of the provided configurations, the tool will return a
success exit code but will not do anything.

### Multi-file projects

If your project consists of more than just a start place, you can simply add new keys to the
`placeFiles` and `placeIds` fields:

```yml
# rocat.yml
placeFiles:
  start: start-place.rbxl
  world: world-place.rbxl

deployments:
  - name: staging
    branches: [dev, experiments/*]
    deployMode: Save # optional; defaults to Publish
    experienceId: 7067418676
    placeIds:
      start: 8468630367
      world: 6179245670
  - name: production
    branches: [main]
    tagCommit: true # optional; defaults to false
    experienceId: 6428418832
    placeIds:
      start: 4927604916
      world: 7618543001
```

When the `deploy` command is run with this configuration, the same deployments will be made as with
the above single-file configuration, except that both the `start-place.rbxl` and `world-place.rbxl`
files will be uploaded to their respective places.

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
        run: rojo build --output start-place.rbxlx
      - name: Deploy project
        run: rocat deploy
        env:
          ROBLOX_API_KEY: ${{ secrets.ROBLOX_API_KEY }}
```

Note that you will need to add your Roblox API key as a secret to your GitHub repository. You can
learn how to do this using [their
docs](https://docs.github.com/en/actions/security-guides/encrypted-secrets#creating-encrypted-secrets-for-a-repository).
