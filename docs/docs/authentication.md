---
sidebar_position: 4
title: Authentication
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

:::danger
Remember to never save your secrets in source control or any insecure environment. Anybody who gets
access to them could use them to steal your accounts.
:::

## Resource management

To manage resources, Mantle requires a valid `.ROBLOSECURITY` cookie value to authenticate all its
requests.

If there is a logged-in Roblox Studio installation, Mantle can automatically
extract its `.ROBLOSECURITY` cookie and will authenticate requests as the user
logged in to Roblox Studio.

Otherwise, you will have to provide the cookie via an environment variable called `ROBLOSECURITY`.

You can set your environment variable in various ways, like the following:

<Tabs>
<TabItem value="dotenv" label="dotenv file" default>

Create `.env` file with the contents:

```conf
ROBLOSECURITY="<your cookie>"
```

[Learn more →](#dotenv-files)

</TabItem>
<TabItem value="powershell" label="Windows (PowerShell)">

```ps1
$env:ROBLOSECURITY = "<your cookie>"
```

Note that this will be temporary and you will have to reset it whenever you start a new terminal
instance.

</TabItem>
<TabItem value="bash" label="MacOS/Linux (Bash)">

```bash
export ROBLOSECURITY = "<your cookie>"
```

Note that this will be temporary and you will have to reset it whenever you start a new terminal
instance.

</TabItem>
</Tabs>
<br/>

To get your `.ROBLOSECURITY` cookie manually, you have a few options:

<Tabs>
<TabItem value="browser" label="From Browser Dev Tools" default>

Navigate to [roblox.com](https://www.roblox.com) in your browser and open the dev tools (right-click
and select "Inspect"). Navigate to the "Application" tab, then look for "Cookies" under "Storage" in
the left-hand sidebar. Under "Cookies", select "`https://www.roblox.com`" then select
"`.ROBLOSECURITY`" from the list of cookies. Copy the value from the "Cookie Value" section. You can
then set your environment variable using one of the above methods.

Note that if you ever log out of your browser session the cookie will be revoked and anything using it
will no longer work. Getting a cookie from a Roblox Studio session is less likely to get revoked as you
typically log out of Roblox Studio less often.

</TabItem>
<TabItem value="rbx_cookie" label="With rbx_cookie utility">

The [`rbx_cookie`](https://crates.io/crates/rbx_cookie) utility is the same tool that Mantle uses to find the
cookie from the logged-in Roblox Studio installation. To install it, first ensure you have Rust installed,
then run `cargo install rbx_cookie`. Now just run `rbx_cookie` in your terminal to access your cookie.

You can also use the [`rbx_auth`](https://crates.io/crates/rbx_auth) utility to check which user is currently
logged-in to Roblox Studio. Install it with `cargo install rbx_auth` and run `rbx_auth` in your terminal to
check the logged-in user.

</TabItem>
<TabItem value="windows-studio" label="From Roblox Studio (Windows)">

Open the Start Menu and search for `regedit` and hit enter. In the window that opens, navigate to
`Computer\HKEY_CURRENT_USER\SOFTWARE\Roblox\RobloxStudioBrowser\roblox.com\`. Double-click on
`.ROBLOSECURITY` to open its value, then look for the text in the format `COOK::<value>`. Copy the
`value` part (not including the `<`/`>`). This is your `.ROBLOSECURITY` cookie. You can then set
your environment variable using one of the above methods.

</TabItem>
<TabItem value="macos-studio" label="From Roblox Studio (MacOS)">

Open a terminal and run:

```sh
defaults read com.roblox.RobloxStudioBrowser
```

Look in the output for a key called `roblox\\U00b7com.\\U00b7ROBLOSECURITY`, then look for the text in the
format `COOK::<value>`. Copy the `value` part (not including the `<`/`>`). This is your `.ROBLOSECURITY`
cookie. You can then set your environment variable using one of the above methods.

</TabItem>
</Tabs>

## Remote state management

Mantle supports managing remote state files using AWS S3 storage which requires authentication. You can
provide your credentials either through environment variables or [an AWS profile
file](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-profiles.html#cli-configure-profiles-create).

If you are new to using AWS, I recommend you read their guide on [best practices for managing AWS
access keys](https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html)
before getting started.

To learn how to get an access key ID and secret, you can read their guide on [understanding and
getting your AWS credentials](https://docs.aws.amazon.com/general/latest/gr/aws-sec-cred-types.html)
(read the intro and "Programmatic access" sections).

The simplest method is to set the `MANTLE_AWS_ACCESS_KEY_ID` and `MANTLE_AWS_SECRET_ACCESS_KEY` environment
variables. Mantle also supports the `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` variables but recommends
you scope your variables to Mantle to avoid conflicts with other tools.

You can set your environment variables in various ways, like the following:

<Tabs>
<TabItem value="dotenv" label="dotenv file" default>

Create `.env` file with the contents:

```conf
MANTLE_AWS_ACCESS_KEY_ID="<your access key id>"
MANTLE_AWS_SECRET_ACCESS_KEY="<your secret access key>"
```

[Learn more →](#dotenv-files)

</TabItem>
<TabItem value="powershell" label="Windows (PowerShell)">

```ps1
$env:MANTLE_AWS_ACCESS_KEY_ID = "<your access key id>"
$env:MANTLE_AWS_SECRET_ACCESS_KEY = "<your secret access key>"
```

Note that these will be temporary and you will have to reset them whenever you start a new terminal
instance.

</TabItem>
<TabItem value="bash" label="MacOS/Linux (Bash)">

```bash
export MANTLE_AWS_ACCESS_KEY_ID = "<your access key id>"
export MANTLE_AWS_SECRET_ACCESS_KEY = "<your secret access key>"
```

Note that these will be temporary and you will have to reset them whenever you start a new terminal
instance.

</TabItem>
</Tabs>

## `dotenv` files

`dotenv` files are a common tool in the industry for storing frequently used environment variables on a
per-developer basis. It is important to make sure you do not check-in your `dotenv` files into your SCM repo.

When a `dotenv` file is present in the current working directory or any of its parents, Mantle will parse its
contents and use the provided variable definitions as environment variables.

To create a `dotenv` file, start by ensuring it will be ignored by your SCM tool. For Git, create or update
your `.gitignore` file:

```shell title=".gitignore"
# ignore all dotenv files
.env
```

Now create a file with the name `.env` in your project, and add any variables you want Mantle to load:

```conf title=".env"
VARIABLE_NAME="<value>"
```

It's good practice to update your `README.md` or `CONTRIBUTING.md` file as well so that other developers on
your team know they need to create a `.env` file themselves and add the necessary variables, for example:

````md title="README.md"
## Contributing

After cloning the repo, create a `.env` file in the root of the project, and add the following variables:

```
MANTLE_AWS_ACCESS_KEY_ID="<your access key id>"
MANTLE_AWS_SECRET_ACCESS_KEY="<your secret access key>"
```

You can get the secrets by...
````
