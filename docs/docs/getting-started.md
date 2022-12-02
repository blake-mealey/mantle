---
sidebar_position: 3
title: Getting Started
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

The quickest way to get started is with the
[Examples](https://github.com/blake-mealey/mantle-examples) repo. This guide will walk you through
deploying your first project with Mantle using the [Getting
Started](https://github.com/blake-mealey/mantle-examples/tree/main/examples/getting-started)
example.

:::tip
This guide will require you to run commands from a terminal. If you're new to terminals, we
recommend you use **PowerShell on Windows** and **Terminal on MacOS**. Some commands we use in this guide
will _not_ work in Window's CMD terminal.
:::

## Get the tools

Before we get started, you'll need to have a couple of tools installed.

### Git

First of all, make sure you have Git installed. If not, you can install it from its official
[downloads](https://git-scm.com/downloads) page.

### Foreman

Next, you'll need to install Foreman which is a Roblox toolchain manager (in other words, it's what
you will use to install Mantle). You can find more information on this in the
[Installation](/docs/installation) guide, but we'll go through it step-by-step here.

<Tabs>
<TabItem value="windows" label="Windows" default>

1. Head over to the Foreman [Releases](https://github.com/roblox/foreman/releases) page, and download
   the latest Windows version (look for the `foreman-x.x.x-win64.zip` link).
2. Once downloaded, unzip the folder
3. Copy the `foreman.exe` file to a reusable location. I like to put mine in `C:\Programs` (you'll
   have to create this folder as it's not a default one)
4. Now we will need to configure your path variable to include both the `foreman.exe` **and** the
   tools that Foreman will install later.
   1. To configure your path variable, open the start menu and search "Edit the system environment
      variables." In the dialog that opens, click "Environment Variables..." Select the "Path"
      variable under "User variables for <username\>" and click "Edit..."
   2. Add `foreman.exe` to your path by clicking "New" to add an entry and enter the path of the
      _folder_ you put `foreman.exe` into (for me it was `C:\Programs`).
   3. Add the Foreman `bin` directory to your path by clicking "New" to add an entry and enter
      `C:\Users\<username>\.foreman\bin` where `<username>` is your Windows username.
   4. Click "Ok" to save your changes.
5. Open a terminal and run `foreman --version` to verify it's working

</TabItem>
<TabItem value="macos" label="MacOS">

1. Head over to the Foreman [Releases](https://github.com/roblox/foreman/releases) page, and download
   the latest MacOS version (look for the `foreman-x.x.x-macos.zip` link).

> TODO: finish writing these docs for MacOS

</TabItem>
</Tabs>

## Clone the examples repo

Now it's time to clone the [Examples](https://github.com/blake-mealey/mantle-examples) repo. In a
terminal, run `git clone https://github.com/blake-mealey/mantle-examples` to clone the repo to your
computer. Now run `cd mantle-examples` to enter the project.

## Install Mantle

Now you can install Mantle using Foreman for the Examples project! Run `foreman install` to install
Mantle. Foreman will download the version of Mantle which the Examples project is configured to use
in its [foreman.toml](https://github.com/blake-mealey/mantle-examples/tree/main/foreman.toml) file
and make it available to your terminal as `mantle` within the Examples project directory.

To verify that it was installed correctly, run `mantle --version`. If you are having trouble, you
can ask for help in the "tooling" channel of the [Roblox OSS Discord](https://discord.gg/wH5ncNS).

## Deploy your first project

Now it's time to deploy your first project! Run `mantle deploy examples/getting-started --environment dev`
to deploy the getting started project.

:::caution
If you are not currently logged in to Roblox Studio, you will need to provide a
`ROBLOSECURITY` environment variable. You can read more about this in the
[Authentication](/docs/authentication) guide.
:::

If everything goes well, you should see that the deployment completed successfully. Let's break down
the output.

First, Mantle tells us about the project we are trying to deploy:

```ansi title="mantle deploy"
Loading project:
  ╷
  │  Loaded config file [36mexamples/getting-started/mantle.yml[0m
  │  Selected provided environment configuration [36mdev[0m
  │  Loading previous state from local file [36mexamples/getting-started/.mantle-state.yml[0m
  │  No previous state for environment [36mdev[0m
  │
  ╰─ Succeeded
```

Here we can see the configuration file that Mantle used was `examples/getting-started/mantle.yml`
because by default Mantle will look for a file called `mantle.yml` in the provided folder. It also
tells us that it is targeting the `dev` environment configuration because we passed `--environment dev`
to the deployment command. Finally, it tells us that it is looking for previous state from the
local file `examples/getting-started/.mantle-state.yml` and that no previous state was found. We'll
come back to this state file in more detail later, but for now you can just know that Mantle
realized we didn't have a state file yet so it created a new one.

Next, Mantle tells us about the resources it is deploying:

```ansi title="mantle deploy"
Deploying resources:
  ╷
  │  [32m+[0m Creating: experience_singleton
  │    ╷
  │    │  Dependencies:
  │    │      [2m[][0m
  │    │  Inputs:
  │    │    [32m+[0m [32mexperience:[0m
  │    │    [32m+[0m [32m  groupId: ~[0m
  │    │
  │    ╰─ Succeeded with outputs:
  │         [32m+[0m [32mexperience:[0m
  │         [32m+[0m [32m  assetId: 3296599132[0m
  │         [32m+[0m [32m  startPlaceId: 8667346609[0m

[2m... cut for brevity ...[0m

  │
  ╰─ Succeeded with 6 create(s), 0 update(s), 0 delete(s), 0 noop(s), 0 skip(s)
```

Here we can see all of the Roblox resources which Mantle has decided it needs to create for us to
match the configuration file. In the snippet above, Mantle is creating a new Roblox experience, and
we can see its asset ID and its start place's asset ID in its outputs section. At the end of this
section, Mantle provides a summary of the operations it performed. In this case, all it did was
create 6 resources.

Next, Mantle tells us where it is saving the state of the current deployment:

```ansi title="mantle deploy"
Saving state:
  ╷
  │  Saving to local file [36mexamples/getting-started/.mantle-state.yml[0m. It is recommended you commit this file to your source control
  │
  ╰─ Succeeded
```

Here's the mysterious state file again! Let's crack it open and take a look. Run
`cat examples/getting-started/.mantle-state.yml` to print the file's contents into your terminal. As
you can see, it's a YAML file containing a list of all of the resources in each environment. If you
look closely, you'll see that its contents actually look very similar to the output which Mantle
already printed out for us. Mantle uses this file to know which resources need to be changed between
deployments, but more on that later. Mantle also told us that "It is recommended you commit this
file to your source control" which is true! You might notice that the examples project doesn't do
this, but this is just so that everyone can try their own deployments.

Finally, Mantle tells us the final results of the deployment as it relates to the "target" resource
which in this case was an experience:

```ansi title="mantle deploy"
Target results:
  ╷
  │  Experience:
  │    https://www.roblox.com/games/8667346609
  │
  │  Places:
  │    start: https://www.roblox.com/games/8667346609
  │
  ╰──○
```

Open one of the links to view and play your new experience! It should look like the following image:

![Roblox place created with Mantle](/img/tutorial/getting-started-place.png)

Let's modify the configuration and see how Mantle responds. Open the
`examples/getting-started/mantle.yml` file in your favourite text editor (I use
[VSCode](https://code.visualstudio.com/)) and change the start place configuration's `name` field to
something new:

```yml title="examples/getting-started/mantle.yml" {15}
environments:
  - name: dev
    targetNamePrefix: environmentName
  - name: prod
    targetAccess: public

target:
  experience:
    configuration:
      genre: building
    places:
      start:
        file: game.rbxlx
        configuration:
          name: I changed the Mantle config
          description: |-
            Made with Mantle
```

Now, re-run the deploy command (`mantle deploy examples/getting-started -e dev`) and see what
happens.

You should see a very similar output to the first deployment, except instead of recreating all of
the resources, Mantle is able to just apply the single change you made to the place configuration:

```ansi title="mantle deploy"
Deploying resources:
  ╷
  │  [33m~[0m Updating: placeConfiguration_start
  │    ╷
  │    │  Dependencies:
  │    │      [2m- place:[0m
  │    │      [2m    assetId: 8667346609[0m
  │    │  Inputs:
  │    │      [2mplaceConfiguration:[0m
  │    │    [31m-[0m [31m  name: "[DEV] Getting Started with Mantle"[0m
  │    │    [32m+[0m [32m  name: "[DEV] I changed the Mantle config"[0m
  │    │      [2m  description: Made with Mantle[0m
  │    │      [2m  maxPlayerCount: 50[0m
  │    │      [2m  allowCopying: false[0m
  │    │      [2m  socialSlotType: Automatic[0m
  │    │      [2m  customSocialSlotsCount: ~[0m
  │    │
  │    ╰─ Succeeded with outputs:
  │           [2mplaceConfiguration[0m
  │
  │
  ╰─ Succeeded with 0 create(s), 1 update(s), 0 delete(s), 5 noop(s), 0 skip(s)
```

As you can see, Mantle highlights for us in the output exactly what changed and which resources
needed to be updated as a result of it. Mantle is able to do this because of the state file! All it
has to do is compare what the results of the previous deployment were by reading the state file with
the "desired state" as defined by the configuration file. Now we finally understand why Mantle uses
a state file!

Let's make a couple more changes to our configuration file. Add a `maxPlayerCount` to the start
place's configuration and add some social links to your experience:

```yml title="examples/getting-started/mantle.yml" {16,19-23}
environments:
  - name: dev
    targetNamePrefix: environmentName
  - name: prod
    targetAccess: public

target:
  experience:
    configuration:
      genre: building
    places:
      start:
        file: game.rbxlx
        configuration:
          name: I changed the Mantle config!
          maxPlayerCount: 25
          description: |-
            Made with Mantle
    socialLinks:
      - title: Follow on Twitter
        url: https://twitter.com/blakemdev
      - title: Official Roblox YouTube
        url: https://youtube.com/channel/UCjiPEaapiHbJMoAdi_L8fNA
```

Re-run the deploy command, then take a look at your experience on the Roblox website again. It
should now be updated to look something like this:

![Roblox place created with Mantle](/img/tutorial/getting-started-place-updated.png)

Note that the title, server size, and social links have all been changed!

At this point I would encourage you to continue playing around with the configuration file and
redeploying to see how Mantle handles the changes and to get familiar with the format. Here are some
things you can try on your own:

- Try deploying to the `prod` environment
- Remove one of the social links
- Add a game icon and thumbnails to the experience
- Try adding a second place to the experience
- Try deploying some of the other example projects

To see what all of the options are for the configuration file, check out the
[Configuration](/docs/configuration) guide!

## Destroy the project when you are finished

If you are done with the example project and you would like to get rid of the places it created in
your Roblox account, you can run `mantle destroy examples/getting-started --environment dev` to
destroy the project. Note that the resources will still be in Roblox but they will be archived and
hidden wherever possible.
