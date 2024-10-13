**kent** - use kubernetes secrets in nix shells

## Overview

Managing multiple Kubernetes environments (like staging or development) often involves juggling various configuration settings, such as database connection strings or Kafka credentials. Keeping these secrets organized and up-to-date across different environments can be tedious.

**kent** simplifies this process by pulling secret values directly from a Kubernetes namespace and passing them into your Nix flake's input. This allows you to leverage Nix's features seamlessly, such as setting up development shells that export environment variables or create scripts for data access. Switching environments is easy - just exit the current devshell and rerun the `kent` command with the desired namespace.

## Usage

1. Add the `kent` package to your nix configuration or use a shell to try it out

```bash
nix shell github:nikolaiser/kent
```

2. Modify the flake that you want to use kubernetes secrets in. By default `kent` will pass secret values the input called `kent`, however it can be overridden (see Reference)

```nix
{
  inputs = {
    ...
    kent = {
      url = "file+file:///dev/null";
      flake = false;
    };
    ...
  };

  outputs = {kent, ...}: 
    let
    ...
      secrets = lib.trivial.importJson kent.outPath;

      # now you can access all arguments by `secrets.<secretName>.<data|stringData>.<name>`
    ...
    in 
    {
    ...
    };
}
```

3. Run `kent`

```bash
kent -f <your-flake>
```

## Reference

```bash

❯ kent --help
Usage: kent [OPTIONS] --flake <FLAKE>

Options:
  -f, --flake <FLAKE>          Nix flake to use
  -i, --input <INPUT>          Flake input that will be used to provide the secret values [default: kent]
  -s, --selector <SELECTOR>    Filter which secret are propagated. For example '-s metadata.name=foo -s metadata.name=bar,metadata.labels.bazz=true'. Multiple expressions are combined as following '-s a -s b,c' <==> 'a || (b && c)' [default: ]
  -n, --namespace <NAMESPACE>  Namespace to extract secret values from. If not provided the currently active one will be used
  -c, --command <COMMAND>      Nix command to run [default: develop]
  -h, --help                   Print help
  -V, --version                Print version

```
