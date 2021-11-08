# Rocat 🚀

An infrastructure-as-code and deployment tool for Roblox.

⚠ Please note that this is an early release and the API is unstable. Releases follow pre-release
semantic versioning (minor updates indicate breaking changes) ⚠

## Installation

### Manual downloads (simplest)

You can download prebuilt binaries from the [GitHub
Releases](https://github.com/blake-mealey/rocat/releases) page.

### With Foreman (recommended)

It is recommended to install with Foreman, with the following config:

```toml
# foreman.toml

[tools]
rocat = { source = "blake-mealey/rocat", version = "0.5.1" }
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

If you are using the `templates` feature you will also need to provide a `ROBLOSECURITY` cookie via
the `ROBLOSECURITY` environment variable. You can get the cookie from your browser dev tools on
roblox.com.

### Configure deployments

Rocat configuration is typically defined in a `rocat.yml` file. Rocat will look for a configuration
file in the provided directory.

```yml
# rocat.yml

deployments:
  - name: staging
    branches: [dev, dev/*]
    deployMode: save # optional; defaults to Publish
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

templates:
  experience:
    genre: building # any valid genre in camelCase
    playableDevices: [computer, phone, tablet, console]
    playability: public # or `friends` or `private`
    paidAccessPrice: 25 # enables paid access and sets its price
    # privateServerPrice: 0 # enables private servers and sets their price
    enableStudioAccessToApis: true
    allowThirdPartySales: true
    allowThirdPartyTeleports: true
    avatarType: r15 # or `r6` or `playerChoice`
    avatarAnimationType: playerChoice # or `standard`
    avatarCollisionType: outerBox # or `innerBox`
    icon: game-icon.png
    thumbnails:
      - game-thumbnail-1.png
      - game-thumbnail-2.png
      - game-thumbnail-3.png
  places:
    start:
      file: start-place.rbxlx
      name: The Best Experience Ever
      description: |
        The best multi-line
        description of all time!
      maxPlayerCount: 25
      serverFill: { reservedSlots: 10 } # or robloxOptimized or maximum
      allowCopying: false
    world:
      file: world-place.rbxl
```

To deploy the above configuration with Rocat, run `rocat deploy` from the file's directory.

If the current git branch does not match any of the provided configurations, the tool will return a
success exit code but will not do anything.

### State management

By default, Rocat will manage state via a local `.rocat-state.yml` file. You can change this with
the `state` field in the configuration file:

```yml
# rocat.yml

state:
  remote:
    region: [us-west-2]
    bucket: rocat-states
    key: project-name
```

With this configuration, Rocat will load and save the state file to the provided AWS S3 object. You
can read this
[guide](https://docs.aws.amazon.com/AmazonS3/latest/userguide/create-bucket-overview.html) on how to
create an S3 bucket.

In order to use this feature, you must also provide two additional environment variables:
`AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`. You can read this
[guide](https://docs.aws.amazon.com/general/latest/gr/aws-sec-cred-types.html) on how to access
these credentials.

Since Rocat requires the state file to be present, there are currently two recommended workflows:

1. Use the default local state management and include `.rocat-state.yml` in your source control.
   Since you need to commit all changes to this file, it is recommended to not use CI and to run the
   tool manually when you want to deploy.
2. Use the remote state management. Since the state file is available from both local and CI
   environments, you can deploy from both whenever you want.

### GitHub Actions

Combined with the [Roblox/setup-forman](https://github.com/Roblox/setup-foreman) Action, it is easy
to create a workflow to deploy your places using Rocat. Note that for this example to work properly
you should be using the remote state management workflow. See [state management](#state-management)
for more info.

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
          ROBLOSECURITY: ${{ secrets.ROBLOSECURITY }}
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
```

Note that you will need to add your secrets to your GitHub repository. You can learn how to do this
using [their
docs](https://docs.github.com/en/actions/security-guides/encrypted-secrets#creating-encrypted-secrets-for-a-repository).
