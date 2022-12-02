---
sidebar_position: 6
title: Continuous Deployment
---

Mantle is designed to be used in continuous deployment (CD) scenarios. This guide will show you how
to setup a GitHub Action to automatically deploy your Mantle project on every commit.

This guide assumes:

- Your project is saved in a GitHub repo
- Your project is being built with [Rojo](https://rojo.space)
- Your project tools (Rojo and Mantle) are
  [installed](/docs/installation#install-with-foreman-recommended) with
  [Foreman](https://github.com/Roblox/foreman#readme)
- Your Mantle project is configured to use [remote state](/docs/remote-state)

## GitHub Actions

### Add required secrets

GitHub will need some secrets in order to deploy your Mantle project. The workflow below uses the following secret names:

- `ROBLOSECURITY` - your Roblox cookie used to manage your Roblox resources
- `AWS_ACCESS_KEY_ID` - your AWS access key ID used to manage your remote state file
- `AWS_SECRET_ACCESS_KEY` - your AWS secret access key used to manage your remote state file

You can add these secrets by going to your GitHub repo's settings page, navigating to the "Secrets"
section, then clicking "New repository secret".

### Check-in the workflow

Check-in the workflow file to your repo:

```yml title=".github/workflows/deploy.yml"
name: Deploy

on:
  # Enable manual deploys from the GitHub UI
  workflow_dispatch:
  # Enable automatic deploys whenever code is pushed to the dev or main branches
  push:
    branches:
      - dev
      - main

jobs:
  build-and-deploy:
    runs-on: windows-latest
    steps:
      # Checkout your Git repo
      - uses: actions/checkout@v2
      # Install foreman and all foreman tools (rojo and mantle)
      - uses: Roblox/setup-foreman@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
      # Build the project with rojo
      - name: Build project
        run: rojo build --output pirate-wars.rbxlx
      # Deploy the project with mantle
      - name: Deploy project
        run: mantle deploy
        env:
          ROBLOSECURITY: ${{ secrets.ROBLOSECURITY }}
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
```

Make sure you update the workflow file to build your project to the location your Mantle file
expects. If your project has multiple place files just copy the "Build project" step for each of
them.

If you want GitHub to run the workflow on different branches, just update the list in the workflow
file.

You're all set! GitHub will now deploy your Mantle project whenever code is checked in to your
configured branches.
