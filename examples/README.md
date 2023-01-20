# Mantle Examples

Example projects for learning Mantle. Follow the [Getting
Started](https://mantledeploy.vercel.app/docs/getting-started) guide for a quick start.

## Usage

```sh
# 1. Clone the repo
git clone https://github.com/blake-mealey/mantle

# 2. CD into the examples directory
cd mantle/examples

# 3. Install Mantle with Foreman - for more information, see
#    https://mantledeploy.vercel.app/docs/Installation
foreman install

# 4. Deploy an example project. All projects configure at least
#    two environments, `dev` and `prod`
mantle deploy projects/getting-started --environment dev

# 5. If you're done with a project, you can destroy it
mantle destroy projects/getting-started --environment dev
```

> Note that if you are not logged in to Roblox Studio on your computer, you will need to set your
> `ROBLOSECURITY` environment variable. See the
> [Authentication](https://mantledeploy.vercel.app/docs/Authentication) guide for more details.

To get a good understanding of Mantle, you are encouraged to play around with the example projects
and see how things change when you re-run `mantle deploy`. Check out the
[Configuration Reference](https://mantledeploy.vercel.app/docs/configuration/reference) for the full list of
options.
