# kubectl-reveal-secret

`kubectl-reveal-secret` is a kubectl plugin that reveals Kubernetes Secret data to allow you see raw data.

## Requirements

- `kubectl`
- `jq`

## Installation

```
$ git clone git@github.com:micnncim/kubectl-reveal-secret.git
$ cp kubectl-reveal-secret/kubectl-reveal-secret /usr/local/bin
```

## Usage

```console
# Reveal Secret data in the current namespace.
$ kubectl reveal secret my-secret

# Reveal Secret data in the specified namespace.
$ kubectl reveal secret my-secret -n my-namespace
```
