# Log Dump Guide

In order to debug some issues, it is useful to supply a log dump of your Mantle command.

> ⚠️ Generating a log dump will include your `ROBLOSECURITY` cookie and potentially your AWS credentials. Make
> sure to remove all secrets from your log file before sending it to us.

To generate a log dump:

1. Set the `RUST_LOG` environment variable to exactly `trace,cookie_store=error,html5ever=error,selectors=error`.
   ```sh
   # PowerShell
   $env:RUST_LOG="trace,cookie_store=error,html5ever=error,selectors=error"
   # Bash
   export RUST_LOG="trace,cookie_store=error,html5ever=error,selectors=error"
   ```
2. Re-run your Mantle command with a stderr redirect to a log file.
   ```sh
    # PowerShell or Bash
    mantle deploy 2> out.log
   ```
3. Remove all secrets from `out.log`. Search the file for `ROBLOSECURITY` and your `AWS_ACCESS_KEY_ID` (if
   applicable) and remove all references.
