---
sidebar_position: 1
title: Introduction
---

Mantle is an experimental declarative infrastructure-as-code (IaC) and deployment tool for Roblox.
Mantle is not currently ready for production use-cases.

## What is IaC?

First of all, what is infrastructure? In today's world of software development with everything
running in the cloud, we don't often think about the physical computers which run our services.
Those computers, or servers, are the infrastructure that runs the cloud. But the term infrastructure
can be extended to mean the resources that are managed in the cloud for a platform. For example, if
you are using AWS to host an application, you might need to configure a VM to run your service on,
and a storage container to store media files, and a database to store user data. All of things are
created and modified using the AWS website UI.

The biggest reason for using IaC is to enable staging environments. In most software development
projects outside of Roblox, developers work on a staging environment before deploying to the
production environment to make sure their end users only see the working application.

But creating a staging environment that works exactly the same as the production environment can be
hard! You have to manually create, update, and delete the infrastructure using the website UI.
That's where IaC comes in. Instead of manually managing your infrastructure, you can write a
declarative configuration file telling the IaC tool what your infrastructure should look like, and
it will create, update, and delete your infrastructure to look the way you want.

This way you also get the advantages of VCS like Git on your infrastructure, and you decouple your
infrastructure from your code because things like URLs and IDs are dynamic.

At this point you're probably wondering how this applies to Roblox. We don't have to create VMs and
storage containers and databases to work with Roblox! Or do we? You see, you actually manage a lot
of "infrastructure" on Roblox: your experiences, places, game icons and thumbnails, badges,
developer products, game passes, images, audio, meshes, and more are all resources which you have
uploaded to Roblox and then use to deliver your game.

What if you could keep all of that configuration inside your VCS and automatically manage that
infrastructure in Roblox without even navigating to the [Create
page](https://www.roblox.com/develop)? With Mantle, now you can!
