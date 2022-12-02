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

```bash title="Bash"
export ROBLOSECURITY = "<your cookie>"
```

```ps1 title="PowerShell"
$env:ROBLOSECURITY = "<your cookie>"
```

Note that these will be temporary and you will have to reset them whenever you start a new terminal
instance.

To get your `.ROBLOSECURITY` cookie manually, you have a couple options:

<Tabs>
<TabItem value="browser" label="From Browser Dev Tools" default>

Navigate to [roblox.com](https://www.roblox.com) in your browser and open the dev tools (right-click
and select "Inspect"). Navigate to the "Application" tab, then look for "Cookies" under "Storage" in
the left-hand sidebar. Under "Cookies", select "`https://www.roblox.com`" then select
"`.ROBLOSECURITY`" from the list of cookies. Copy the value from the "Cookie Value" section. You can
then set your environment variable using one of the above methods.

</TabItem>
<TabItem value="windows-studio" label="From Roblox Studio (Windows)">

Open the Start Menu and search for `regedit` and hit enter. In the window that opens, navigate to
`Computer\HKEY_CURRENT_USER\SOFTWARE\Roblox\RobloxStudioBrowser\roblox.com\`. Double-click on
`.ROBLOSECURITY` to open its value, then look for the text in the format `COOK::<value>`. Copy the
`value` part (not including the `<`/`>`). This is your `.ROBLOSECURITY` cookie. You can then set
your environment variable using one of the above methods.

The advantage of this method is that the cookie is less likely to expire or be revoked.

</TabItem>
<TabItem value="macos-studio" label="From Roblox Studio (MacOS)" default>

Open a terminal and run:

```sh
defaults read com.roblox.RobloxStudioBrowser
```

Look in the output for a key called `roblox\\U00b7com.\\U00b7ROBLOSECURITY`,
then look for the text in the format `COOK::<value>`. Copy the `value` part (not
including the `<`/`>`). This is your `.ROBLOSECURITY` cookie. You can then set
your environment variable using one of the above methods.

The advantage of this method is that the cookie is less likely to expire or be
revoked.

</TabItem>
</Tabs>

## Remote State Management

Mantle supports managing remote state files using AWS S3 storage which requires authentication.
Under the hood, Mantle uses the `rusoto` library and therefore supports all methods of
authentication which [rusoto
supports](https://github.com/rusoto/rusoto/blob/master/AWS-CREDENTIALS.md#credentials).

The simplest way to supply authentication for your remote state file is via the `AWS_ACCESS_KEY_ID`
and `AWS_SECRET_ACCESS_KEY` environment variables.

You can set your environment variables in various ways, like the following:

```bash title="Bash"
export AWS_ACCESS_KEY_ID = "<your access key id>"
export AWS_SECRET_ACCESS_KEY = "<your secret access key>"
```

```ps1 title="PowerShell"
$env:AWS_ACCESS_KEY_ID = "<your access key id>"
$env:AWS_SECRET_ACCESS_KEY = "<your secret access key>"
```

Note that these will be temporary and you will have to reset them whenever you start a new terminal
instance.

If you are new to using AWS, I recommend you read their guide on [best practices for managing AWS
access keys](https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html)
before getting started.

To learn how to get an access key ID and secret, you can read their guide on [understanding and
getting your AWS credentials](https://docs.aws.amazon.com/general/latest/gr/aws-sec-cred-types.html)
(read the intro and "Programmatic access" sections).
