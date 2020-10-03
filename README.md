# kubectl-reveal-secret

`kubectl-reveal-secret` is a kubectl plugin that reveals Kubernetes Secret data to allow you to see raw data.

## Requirements

- `kubectl`
- `jq`

## Installation

```console
$ git clone git@github.com:micnncim/kubectl-reveal-secret.git
$ cp kubectl-reveal-secret/kubectl-reveal-secret /usr/local/bin
```

## Usage

```console
$ kubectl reveal secret --help

  kubectl-reveal-secret reveals Kubernetes Secret data.

  Usage:
      kubectl reveal secret SECRET_NAME [optional flags]

  Options:
      -n, --namespace    If present, the namespace scope for this CLI request
      --context          The name of the kubeconfig context to use
      -h, --help         Show the usage of this CLI

  Examples:
    # Reveal Secret data in the current namespace.
    $ kubectl reveal secret my-secret

    # Reveal Secret data in the specified namespace.
    $ kubectl reveal secret my-secret -n my-namespace

    # Reveal Secret data in the specified context and namespace.
    $ kubectl reveal secret my-secret --context my-context -n my-namespace

```

## Examples

```
$ kubectl create secret generic my-secret --from-literal key=value
$ kubectl reveal secret
key     hello
```
