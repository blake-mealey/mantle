---
sidebar_position: 5
title: Remote State
---

import TOCInline from '@theme/TOCInline';

Mantle saves the state of your resources in state files which enables it to make only the required
changes to your resources after your configuration and files change.

By default, Mantle will save your state to a local `.mantle-state.yml` file which you can check-in
to your Git repo. But Mantle can also save your state to remote files stored in a cloud provider
which comes with a few benefits.

Since a remote state file can be accessed and updated from anywhere, you can share it across
branches and computers. This makes it less likely your state files gets out of sync with the real
resources in Roblox, and it is ideal for continuous deployment (CD) scenarios.

Currently, the only supported cloud provider for remote state is Amazon S3.

## Amazon S3

<TOCInline toc={toc[0].children} />

### Create an account

In order to create an AWS account you will need to provide a credit card. Amazon S3 provides a
generous free tier which you are very unlikely to reach the limits of while using Mantle, but you
should always monitor your usage to make sure you do not incur unexpected costs.

Create an AWS account from their [sign up page](https://aws.amazon.com/free/).

### Create an S3 bucket

From the AWS console, click the search bar at the top of the page and search for "S3" and select the
S3 service.

From the "Buckets" tab, click the "Create bucket" button. Give your bucket a name like
`<yourname>-mantle-states` and select an AWS region that is near your location. Ensure "Block _all_
public access" is checked to keep your data private.

In the future, Mantle may support object locking, so you may want to expand "Advanced settings" and
enable "Object lock" so that you will not need to re-create your bucket later.

Click "Create bucket" to finish creating your bucket.

### Upload your existing state file

If you are migrating an existing project from local state files to remote, you will need to upload
your state file to your bucket.

Before you do so though you need to pick your project name which will be used for the name of the
state file in your bucket and in your Mantle configuration in the next step.

Rename your local state file to `<project-name>.mantle-state.yml`, then upload it to your bucket.
From the S3 buckets tab, click on your new bucket's name, then click the "Upload" button. Click "Add
files" and open your renamed state file. Click "Upload" to finish uploading the state file.

### Configure your Mantle project

Head back to your Mantle config file and add the `state` configuration:

```yml title="mantle.yml"
# Your existing configuration

state:
  remote:
    region: [<aws-region>]
    bucket: <bucket-name>
    key: <project-name>
```

- Replace `<aws-region>` with the region you selected when you created your bucket (e.g.
  `us-west-2`).
- Replace `<bucket-name>` with the name you selected when you created your bucket.
- Replace `<project-name>` with the name of your project. This will be used to name the state file
  in your bucket. For example, if you supply `pirate-wars` it will create a
  `pirate-wars.mantle-state.yml` file in your bucket.

### Supply your AWS credentials

At this point if you were to run `mantle deploy` it would attempt to download your state file from
S3 but it would fail because you have not provided any authorization for your bucket.

You can read the [Authentication](/docs/authentication#remote-state-management) guide for
information on supplying your AWS credentials.

Once supplied, you can run `mantle deploy` or any other commands and they will operate on your
remote state file! If you were migrating from local state to remote state, you should delete the
`.mantle-state.yml` file from your Git repo as it will be unused and out of date.
